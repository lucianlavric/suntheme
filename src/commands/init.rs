use anyhow::Result;
use dialoguer::{Confirm, Input, Select};
use std::collections::HashSet;
use std::process::Command;

use crate::config::{Config, Location, ThemePair, Themes};
use crate::sun_times::{geocode_location, SunTimes};
use crate::theme_switcher::ThemeSwitcher;
use crate::themes::{get_ghostty_themes, setup_neovim_integration, validate_ghostty_theme};

pub fn run() -> Result<()> {
    println!("Welcome to suntheme setup!\n");

    // Request accessibility permissions for Ghostty auto-reload (macOS only)
    #[cfg(target_os = "macos")]
    {
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
    println!("Enter your location for sunrise/sunset calculations.\n");

    let (latitude, longitude) = loop {
        let location_query: String = Input::new()
            .with_prompt("Location (city, address, etc.)")
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
                    .with_prompt("Select your location")
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

    // Load available Ghostty themes
    let available_themes = get_ghostty_themes().unwrap_or_default();
    let has_themes = !available_themes.is_empty();

    if has_themes {
        println!("Found {} Ghostty themes installed.\n", available_themes.len());
    }

    // Get Ghostty themes with validation
    println!("Configure Ghostty themes:");

    let ghostty_light = prompt_theme("Ghostty light theme", "tokyonight-day", &available_themes, has_themes)?;
    let ghostty_dark = prompt_theme("Ghostty dark theme", "tokyonight", &available_themes, has_themes)?;

    // Get Neovim themes (no validation - too many sources)
    println!("\nConfigure Neovim themes:");

    let neovim_light: String = Input::new()
        .with_prompt("Neovim light theme")
        .default("tokyonight-day".to_string())
        .interact_text()?;

    let neovim_dark: String = Input::new()
        .with_prompt("Neovim dark theme")
        .default("tokyonight".to_string())
        .interact_text()?;

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

    println!("\nSetup complete! Commands:\n");
    println!("  suntheme sun      - Show today's sunrise/sunset times");
    println!("  suntheme start    - Start the daemon for automatic switching");
    println!("  suntheme toggle   - Manually toggle between light/dark");
    println!("  suntheme set dark - Set a specific theme mode");
    println!("  suntheme themes   - Change theme configuration");
    println!("  suntheme status   - Check daemon status");

    Ok(())
}

fn prompt_theme(
    prompt: &str,
    default: &str,
    available: &HashSet<String>,
    validate: bool,
) -> Result<String> {
    loop {
        let theme: String = Input::new()
            .with_prompt(prompt)
            .default(default.to_string())
            .interact_text()?;

        if !validate || available.is_empty() {
            return Ok(theme);
        }

        if validate_ghostty_theme(&theme, available) {
            return Ok(theme);
        }

        println!("  Theme '{}' not found in Ghostty themes.", theme);

        // Suggest similar themes
        let suggestions: Vec<&String> = available
            .iter()
            .filter(|t| t.to_lowercase().contains(&theme.to_lowercase())
                || theme.to_lowercase().contains(&t.to_lowercase()))
            .take(5)
            .collect();

        if !suggestions.is_empty() {
            println!("  Similar themes: {}", suggestions.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
        }

        let use_anyway = Confirm::new()
            .with_prompt("  Use this theme anyway?")
            .default(false)
            .interact()?;

        if use_anyway {
            return Ok(theme);
        }
    }
}
