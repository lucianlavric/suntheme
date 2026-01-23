use anyhow::Result;
use dialoguer::{Confirm, Input, Select};

use crate::banner;
use crate::config::{Config, Location, ThemePair, Themes};
use crate::sun_times::{geocode_location, SunTimes};
use crate::telemetry;
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

    let existing_config = if Config::exists()? {
        Some(Config::load()?)
    } else {
        None
    };

    let (latitude, longitude) = if let Some(ref config) = existing_config {
        // Config exists - ask what they want to do
        let options = vec![
            "Change themes only (keep current location)",
            "Full setup (reconfigure everything)",
            "Cancel",
        ];

        let selection = Select::new()
            .with_prompt("Config already exists. What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                // Keep existing location, skip to theme selection
                println!(
                    "\nKeeping location: ({:.4}, {:.4})\n",
                    config.location.latitude, config.location.longitude
                );
                (config.location.latitude, config.location.longitude)
            }
            1 => {
                // Full setup - get new location
                println!("\n--- Location Setup ---\n");
                println!("Enter your location for sunrise/sunset calculations.\n");
                get_location()?
            }
            _ => {
                println!("Setup cancelled.");
                return Ok(());
            }
        }
    } else {
        // No existing config - do full location setup
        println!("--- Location Setup ---\n");
        println!("Enter your location for sunrise/sunset calculations.\n");
        get_location()?
    };

    // Theme selection with presets
    println!("--- Theme Setup ---\n");
    let (ghostty_light, ghostty_dark, neovim_light, neovim_dark) = select_theme_preset()?;

    // Ask for anonymous telemetry consent (only if new setup or not previously set)
    let telemetry_enabled = if let Some(ref config) = existing_config {
        if let Some(tel) = config.telemetry {
            tel // Keep existing preference
        } else {
            ask_telemetry_consent()?
        }
    } else {
        ask_telemetry_consent()?
    };

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
        telemetry: Some(telemetry_enabled),
    };

    config.save()?;
    println!("Config saved to {:?}", Config::config_path()?);

    // Send telemetry ping if enabled (only on first setup)
    if existing_config.is_none() && telemetry_enabled {
        telemetry::send_install_ping();
    }

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

fn ask_telemetry_consent() -> Result<bool> {
    println!("--- Help Improve Suntheme ---\n");
    let telemetry_enabled = Confirm::new()
        .with_prompt("  Share anonymous install statistics?")
        .default(true)
        .interact()?;

    if telemetry_enabled {
        println!("  Thanks! This helps prioritize development.\n");
    } else {
        println!("  No problem. No data will be collected.\n");
    }

    Ok(telemetry_enabled)
}

fn get_location() -> Result<(f64, f64)> {
    loop {
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

                return Ok((selected.latitude, selected.longitude));
            }
            Err(e) => {
                println!("Error: {}. Please try again.\n", e);
                continue;
            }
        }
    }
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
