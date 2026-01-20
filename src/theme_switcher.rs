use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::sun_times::ThemeMode;

pub struct ThemeSwitcher {
    config: Config,
}

impl ThemeSwitcher {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn apply_theme(&self, mode: ThemeMode) -> Result<()> {
        self.apply_ghostty_theme(mode)?;
        self.apply_neovim_theme(mode)?;
        self.write_state(mode)?;
        Ok(())
    }

    fn apply_ghostty_theme(&self, mode: ThemeMode) -> Result<()> {
        let theme_name = match mode {
            ThemeMode::Light => &self.config.themes.ghostty.light,
            ThemeMode::Dark => &self.config.themes.ghostty.dark,
        };

        let ghostty_config = Self::ghostty_config_path()?;

        if !ghostty_config.exists() {
            // Create the config file with just the theme
            let dir = ghostty_config.parent().unwrap();
            fs::create_dir_all(dir)?;
            fs::write(&ghostty_config, format!("theme = {}\n", theme_name))?;
            return Ok(());
        }

        let content = fs::read_to_string(&ghostty_config)
            .with_context(|| format!("Failed to read Ghostty config at {:?}", ghostty_config))?;

        let new_content = Self::update_ghostty_theme(&content, theme_name);

        fs::write(&ghostty_config, new_content)
            .with_context(|| "Failed to write Ghostty config")?;

        // Try to reload Ghostty
        Self::reload_ghostty();

        Ok(())
    }

    fn reload_ghostty() {
        #[cfg(target_os = "macos")]
        {
            // Use AppleScript to click Ghostty's reload menu item
            use std::process::Command;
            let _ = Command::new("osascript")
                .args([
                    "-e",
                    r#"tell application "Ghostty" to activate"#,
                    "-e",
                    r#"tell application "System Events" to tell process "Ghostty" to click menu item "Reload Configuration" of menu "Ghostty" of menu bar 1"#,
                ])
                .output();
        }

        #[cfg(target_os = "linux")]
        {
            // Send SIGUSR2 to all Ghostty processes to trigger config reload
            use std::process::Command;
            if let Ok(output) = Command::new("pgrep").arg("-x").arg("ghostty").output() {
                let pids = String::from_utf8_lossy(&output.stdout);
                for pid in pids.lines() {
                    if let Ok(pid) = pid.trim().parse::<i32>() {
                        unsafe {
                            libc::kill(pid, libc::SIGUSR2);
                        }
                    }
                }
            }
        }
    }

    fn update_ghostty_theme(content: &str, theme_name: &str) -> String {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;

        for line in &mut lines {
            let trimmed = line.trim();
            if trimmed.starts_with("theme") {
                if let Some(eq_pos) = trimmed.find('=') {
                    let before_eq = &trimmed[..eq_pos];
                    if before_eq.trim() == "theme" {
                        *line = format!("theme = {}", theme_name);
                        found = true;
                        break;
                    }
                }
            }
        }

        if !found {
            // Add theme at the beginning
            lines.insert(0, format!("theme = {}", theme_name));
        }

        lines.join("\n") + "\n"
    }

    fn ghostty_config_path() -> Result<PathBuf> {
        // macOS: ~/Library/Application Support/com.mitchellh.ghostty/config
        // Linux: ~/.config/ghostty/config
        if let Some(data_dir) = dirs::data_dir() {
            let macos_path = data_dir.join("com.mitchellh.ghostty").join("config");
            if macos_path.exists() || cfg!(target_os = "macos") {
                return Ok(macos_path);
            }
        }

        let config_dir = dirs::config_dir().context("Could not determine config directory")?;
        Ok(config_dir.join("ghostty").join("config"))
    }

    fn apply_neovim_theme(&self, mode: ThemeMode) -> Result<()> {
        let theme_name = match mode {
            ThemeMode::Light => &self.config.themes.neovim.light,
            ThemeMode::Dark => &self.config.themes.neovim.dark,
        };

        let state_file = Config::state_file()?;
        let state_dir = state_file.parent().unwrap();
        fs::create_dir_all(state_dir)?;

        // Write theme info as simple key=value for easy parsing
        let content = format!(
            "mode={}\ntheme={}\nbackground={}\n",
            mode.as_str(),
            theme_name,
            mode.as_str()
        );

        fs::write(&state_file, content)
            .with_context(|| format!("Failed to write state file at {:?}", state_file))?;

        Ok(())
    }

    fn write_state(&self, _mode: ThemeMode) -> Result<()> {
        // State is already written by apply_neovim_theme
        // This method exists for potential future use
        Ok(())
    }

    pub fn get_current_mode(&self) -> Result<Option<ThemeMode>> {
        let state_file = Config::state_file()?;
        if !state_file.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&state_file)?;
        for line in content.lines() {
            if let Some(mode_str) = line.strip_prefix("mode=") {
                return Ok(Some(mode_str.parse()?));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_ghostty_theme_existing() {
        let content = "font-size = 14\ntheme = old-theme\nwindow-padding = 10\n";
        let result = ThemeSwitcher::update_ghostty_theme(content, "new-theme");
        assert!(result.contains("theme = new-theme"));
        assert!(result.contains("font-size = 14"));
        assert!(!result.contains("old-theme"));
    }

    #[test]
    fn test_update_ghostty_theme_missing() {
        let content = "font-size = 14\nwindow-padding = 10\n";
        let result = ThemeSwitcher::update_ghostty_theme(content, "new-theme");
        assert!(result.contains("theme = new-theme"));
        assert!(result.contains("font-size = 14"));
    }

    #[test]
    fn test_update_ghostty_theme_empty() {
        let content = "";
        let result = ThemeSwitcher::update_ghostty_theme(content, "my-theme");
        assert!(result.contains("theme = my-theme"));
    }

    #[test]
    fn test_update_ghostty_theme_with_spaces() {
        let content = "theme   =   spaced-theme\n";
        let result = ThemeSwitcher::update_ghostty_theme(content, "new-theme");
        assert!(result.contains("theme = new-theme"));
    }
}
