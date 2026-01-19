use anyhow::Result;
use dialoguer::Input;

use crate::config::{Config, ThemePair};
use crate::sun_times::ThemeMode;
use crate::theme_switcher::ThemeSwitcher;

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
    println!("  Ghostty: light='{}', dark='{}'",
        config.themes.ghostty.light,
        config.themes.ghostty.dark
    );
    println!("  Neovim:  light='{}', dark='{}'",
        config.themes.neovim.light,
        config.themes.neovim.dark
    );
    println!();

    // Get Ghostty themes
    println!("Configure Ghostty themes:");

    let ghostty_light: String = Input::new()
        .with_prompt("Ghostty light theme")
        .default(config.themes.ghostty.light.clone())
        .interact_text()?;

    let ghostty_dark: String = Input::new()
        .with_prompt("Ghostty dark theme")
        .default(config.themes.ghostty.dark.clone())
        .interact_text()?;

    // Get Neovim themes
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
