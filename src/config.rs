use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub location: Location,
    pub themes: Themes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Themes {
    pub ghostty: ThemePair,
    pub neovim: ThemePair,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePair {
    pub light: String,
    pub dark: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            location: Location {
                latitude: 0.0,
                longitude: 0.0,
            },
            themes: Themes {
                ghostty: ThemePair {
                    light: "rose-pine-dawn".to_string(),
                    dark: "rose-pine".to_string(),
                },
                neovim: ThemePair {
                    light: "rose-pine-dawn".to_string(),
                    dark: "rose-pine".to_string(),
                },
            },
        }
    }
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("suntheme");
        Ok(config_dir)
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn cache_dir() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .context("Could not determine cache directory")?
            .join("suntheme");
        Ok(cache_dir)
    }

    pub fn state_dir() -> Result<PathBuf> {
        // On macOS, use ~/Library/Application Support/suntheme
        // On Linux, use ~/.local/state/suntheme
        let state_dir = dirs::config_dir()
            .context("Could not determine state directory")?
            .join("suntheme");
        Ok(state_dir)
    }

    pub fn state_file() -> Result<PathBuf> {
        Ok(Self::state_dir()?.join("current_theme"))
    }

    pub fn pid_file() -> Result<PathBuf> {
        Ok(Self::state_dir()?.join("daemon.pid"))
    }

    pub fn log_file() -> Result<PathBuf> {
        Ok(Self::state_dir()?.join("daemon.log"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            anyhow::bail!(
                "Config file not found. Run 'suntheme init' to set up."
            );
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config from {:?}", path))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse config file")?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let dir = path.parent().unwrap();
        fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create config directory {:?}", dir))?;
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&path, content)
            .with_context(|| format!("Failed to write config to {:?}", path))?;
        Ok(())
    }

    pub fn exists() -> Result<bool> {
        Ok(Self::config_path()?.exists())
    }
}
