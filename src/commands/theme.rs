use anyhow::Result;
use dialoguer::{Input, Select};

use crate::config::{Config, ThemePair};
use crate::sun_times::ThemeMode;
use crate::theme_switcher::ThemeSwitcher;
use crate::themes::get_theme_presets;

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

    let (ghostty_light, ghostty_dark, neovim_light, neovim_dark) = select_theme_preset()?;

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

fn select_theme_preset() -> Result<(String, String, String, String)> {
    let presets = get_theme_presets();

    // Build display list with presets + custom option
    let mut items: Vec<String> = presets.iter().map(|p| p.display_name.to_string()).collect();
    items.push("Custom (enter manually)".to_string());

    let selection = Select::new()
        .with_prompt("Select a theme")
        .items(&items)
        .default(0)
        .interact()?;

    if selection < presets.len() {
        // User selected a preset
        let preset = &presets[selection];
        println!(
            "\n✓ {} selected:\n  Ghostty: {} / {}\n  Neovim:  {} / {}",
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
        println!("\nEnter theme names manually:\n");

        let ghostty_dark: String = Input::new()
            .with_prompt("Ghostty dark theme")
            .default("tokyonight".to_string())
            .interact_text()?;

        let ghostty_light: String = Input::new()
            .with_prompt("Ghostty light theme")
            .default("tokyonight-day".to_string())
            .interact_text()?;

        let neovim_dark: String = Input::new()
            .with_prompt("Neovim dark theme")
            .default("tokyonight".to_string())
            .interact_text()?;

        let neovim_light: String = Input::new()
            .with_prompt("Neovim light theme")
            .default("tokyonight-day".to_string())
            .interact_text()?;

        Ok((ghostty_light, ghostty_dark, neovim_light, neovim_dark))
    }
}
