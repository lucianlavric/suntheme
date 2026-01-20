use anyhow::Result;
use dialoguer::{Confirm, Input};
use std::collections::HashSet;

use crate::config::{Config, ThemePair};
use crate::sun_times::ThemeMode;
use crate::theme_switcher::ThemeSwitcher;
use crate::themes::{get_ghostty_themes, validate_ghostty_theme};

pub fn set(mode: ThemeMode) -> Result<()> {
    let config = Config::load()?;
    let switcher = ThemeSwitcher::new(config);

    switcher.apply_theme(mode)?;

    println!("Theme set to: {}", mode);
    Ok(())
}

pub fn toggle() -> Result<()> {
    let config = Config::load()?;
    let switcher = ThemeSwitcher::new(config);

    let current = switcher.get_current_mode()?;
    let new_mode = match current {
        Some(mode) => mode.opposite(),
        None => ThemeMode::Dark, // Default to dark if no state exists
    };

    switcher.apply_theme(new_mode)?;

    println!("Theme toggled to: {}", new_mode);
    Ok(())
}

pub fn configure_themes() -> Result<()> {
    let mut config = Config::load()?;

    println!("Configure themes\n");
    println!("Current configuration:");
    println!(
        "  Ghostty: light='{}', dark='{}'",
        config.themes.ghostty.light, config.themes.ghostty.dark
    );
    println!(
        "  Neovim:  light='{}', dark='{}'",
        config.themes.neovim.light, config.themes.neovim.dark
    );
    println!();

    // Load available Ghostty themes for validation
    let available_themes = get_ghostty_themes().unwrap_or_default();
    let has_themes = !available_themes.is_empty();

    if has_themes {
        println!("Found {} Ghostty themes installed.\n", available_themes.len());
    }

    // Get Ghostty themes with validation
    println!("Configure Ghostty themes:");

    let ghostty_light = prompt_theme(
        "Ghostty light theme",
        &config.themes.ghostty.light,
        &available_themes,
        has_themes,
    )?;

    let ghostty_dark = prompt_theme(
        "Ghostty dark theme",
        &config.themes.ghostty.dark,
        &available_themes,
        has_themes,
    )?;

    // Get Neovim themes (no validation - too many sources)
    println!("\nConfigure Neovim themes:");

    let neovim_light: String = Input::new()
        .with_prompt("Neovim light theme")
        .default(config.themes.neovim.light.clone())
        .interact_text()?;

    let neovim_dark: String = Input::new()
        .with_prompt("Neovim dark theme")
        .default(config.themes.neovim.dark.clone())
        .interact_text()?;

    config.themes.ghostty = ThemePair {
        light: ghostty_light,
        dark: ghostty_dark,
    };

    config.themes.neovim = ThemePair {
        light: neovim_light,
        dark: neovim_dark,
    };

    config.save()?;

    println!("\nTheme configuration updated!");

    // Re-apply current theme with new settings
    let switcher = ThemeSwitcher::new(config);
    if let Some(current_mode) = switcher.get_current_mode()? {
        switcher.apply_theme(current_mode)?;
        println!("Applied {} theme with new settings.", current_mode);
    }

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
            .filter(|t| {
                t.to_lowercase().contains(&theme.to_lowercase())
                    || theme.to_lowercase().contains(&t.to_lowercase())
            })
            .take(5)
            .collect();

        if !suggestions.is_empty() {
            println!(
                "  Similar themes: {}",
                suggestions
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
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
