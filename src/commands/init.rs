use anyhow::Result;
use dialoguer::{Confirm, Input, Select};

use crate::banner;
use crate::config::{Config, Location, ThemePair, Themes};
use crate::sun_times::{geocode_location, SunTimes};
use crate::theme_switcher::ThemeSwitcher;
use crate::themes::{get_theme_presets, setup_neovim_integration};

pub fn run() -> Result<()> {
    banner::print_welcome();

    // Request accessibility permissions for Ghostty auto-reload (macOS only)
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        println!("Suntheme needs accessibility permissions to auto-reload Ghostty themes.");
        println!("A system prompt may appear - please grant access to continue.\n");

        // Trigger the permission prompt
        let _ = Command::new("osascript")
            .args([
                "-e",
                r#"tell application "System Events" to return name of first process"#,
            ])
            .output();

        let granted = Confirm::new()
            .with_prompt("Have you granted accessibility permissions?")
            .default(true)
            .interact()?;

        if !granted {
            println!("\nYou can grant permissions later in:");
            println!("  System Settings > Privacy & Security > Accessibility");
            println!("Without this, you'll need to press Cmd+Shift+, to reload Ghostty.\n");
        } else {
            println!();
        }
    }

    if Config::exists()? {
        let overwrite = Confirm::new()
            .with_prompt("Config already exists. Overwrite?")
            .default(false)
            .interact()?;

        if !overwrite {
            println!("Setup cancelled.");
            return Ok(());
        }
    }

    // Get location by name
    println!("--- Location Setup ---\n");
    println!("Enter your location for sunrise/sunset calculations.\n");

    let (latitude, longitude) = loop {
        let location_query: String = Input::new()
            .with_prompt("  Location (city, address, etc.)")
            .interact_text()?;

        println!("Searching...");

        match geocode_location(&location_query) {
            Ok(locations) => {
                if locations.is_empty() {
                    println!("No locations found. Please try again.\n");
                    continue;
                }

                let items: Vec<String> = locations
                    .iter()
                    .map(|loc| loc.display_name.clone())
                    .collect();

                let selection = Select::new()
                    .with_prompt("  Select your location")
                    .items(&items)
                    .default(0)
                    .interact()?;

                let selected = &locations[selection];
                println!(
                    "\nSelected: {} ({:.4}, {:.4})\n",
                    selected.display_name, selected.latitude, selected.longitude
                );

                break (selected.latitude, selected.longitude);
            }
            Err(e) => {
                println!("Error: {}. Please try again.\n", e);
                continue;
            }
        }
    };

    // Theme selection with presets
    println!("--- Theme Setup ---\n");
    let (ghostty_light, ghostty_dark, neovim_light, neovim_dark) = select_theme_preset()?;

    // Create and save config
    let config = Config {
        location: Location {
            latitude,
            longitude,
        },
        themes: Themes {
            ghostty: ThemePair {
                light: ghostty_light,
                dark: ghostty_dark,
            },
            neovim: ThemePair {
                light: neovim_light,
                dark: neovim_dark,
            },
        },
    };

    config.save()?;
    println!("\nConfig saved to {:?}", Config::config_path()?);

    // Set up Neovim integration
    println!("\nSetting up Neovim integration...");
    match setup_neovim_integration() {
        Ok(path) => {
            println!("Created {:?}", path);
            println!("Added require(\"suntheme\") to init.lua");
        }
        Err(e) => {
            println!("Warning: Could not set up Neovim integration: {}", e);
        }
    }

    // Apply theme based on current sun position
    println!("\nApplying theme based on current time...");
    match SunTimes::get_cached_or_fetch(latitude, longitude) {
        Ok(sun_times) => {
            let current_mode = sun_times.current_mode();
            let switcher = ThemeSwitcher::new(config);

            match switcher.apply_theme(current_mode) {
                Ok(_) => {
                    println!("Applied {} theme.", current_mode);
                    println!(
                        "Sunrise: {} | Sunset: {}",
                        sun_times.sunrise_local().format("%H:%M"),
                        sun_times.sunset_local().format("%H:%M")
                    );
                }
                Err(e) => {
                    println!("Warning: Could not apply theme: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Warning: Could not fetch sun times: {}", e);
            println!("Run 'suntheme set dark' or 'suntheme set light' to apply manually.");
        }
    }

    println!("\n--- Setup Complete! ---\n");
    println!("  suntheme sun      Show today's sunrise/sunset times");
    println!("  suntheme start    Start the daemon for automatic switching");
    println!("  suntheme toggle   Manually toggle between light/dark");
    println!("  suntheme set      Set a specific theme mode");
    println!("  suntheme themes   Change theme configuration");
    println!("  suntheme status   Check daemon status");
    println!();

    Ok(())
}

fn select_theme_preset() -> Result<(String, String, String, String)> {
    let presets = get_theme_presets();

    // Build display list with presets + custom option
    let mut items: Vec<String> = presets.iter().map(|p| p.display_name.to_string()).collect();
    items.push("Custom (enter manually)".to_string());

    let selection = Select::new()
        .with_prompt("  Select a theme")
        .items(&items)
        .default(0)
        .interact()?;

    if selection < presets.len() {
        // User selected a preset
        let preset = &presets[selection];
        println!(
            "\n  ✓ {} selected:\n    Ghostty: {} / {}\n    Neovim:  {} / {}\n",
            preset.display_name,
            preset.ghostty_dark,
            preset.ghostty_light,
            preset.neovim_dark,
            preset.neovim_light
        );
        Ok((
            preset.ghostty_light.to_string(),
            preset.ghostty_dark.to_string(),
            preset.neovim_light.to_string(),
            preset.neovim_dark.to_string(),
        ))
    } else {
        // Custom entry
        println!("\n  Enter theme names manually:\n");

        let ghostty_dark: String = Input::new()
            .with_prompt("  Ghostty dark theme")
            .default("tokyonight".to_string())
            .interact_text()?;

        let ghostty_light: String = Input::new()
            .with_prompt("  Ghostty light theme")
            .default("tokyonight-day".to_string())
            .interact_text()?;

        let neovim_dark: String = Input::new()
            .with_prompt("  Neovim dark theme")
            .default("tokyonight".to_string())
            .interact_text()?;

        let neovim_light: String = Input::new()
            .with_prompt("  Neovim light theme")
            .default("tokyonight-day".to_string())
            .interact_text()?;

        Ok((ghostty_light, ghostty_dark, neovim_light, neovim_dark))
    }
}
