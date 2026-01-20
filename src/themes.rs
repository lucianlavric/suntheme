use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// A theme preset with friendly name and corresponding Ghostty/Neovim theme names
#[derive(Clone)]
pub struct ThemePreset {
    pub display_name: &'static str,
    pub ghostty_dark: &'static str,
    pub ghostty_light: &'static str,
    pub neovim_dark: &'static str,
    pub neovim_light: &'static str,
}

pub fn get_theme_presets() -> Vec<ThemePreset> {
    // Note: Ghostty 1.2.0+ uses Title Case for theme names
    vec![
        ThemePreset {
            display_name: "Tokyo Night",
            ghostty_dark: "TokyoNight",
            ghostty_light: "TokyoNight Day",
            neovim_dark: "tokyonight",
            neovim_light: "tokyonight-day",
        },
        ThemePreset {
            display_name: "Gruvbox",
            ghostty_dark: "GruvboxDark",
            ghostty_light: "GruvboxLight",
            neovim_dark: "gruvbox",
            neovim_light: "gruvbox",
        },
        ThemePreset {
            display_name: "Catppuccin",
            ghostty_dark: "Catppuccin Mocha",
            ghostty_light: "Catppuccin Latte",
            neovim_dark: "catppuccin",
            neovim_light: "catppuccin",
        },
        ThemePreset {
            display_name: "Nord",
            ghostty_dark: "Nord",
            ghostty_light: "Nord Light",
            neovim_dark: "nord",
            neovim_light: "nord",
        },
        ThemePreset {
            display_name: "Dracula",
            ghostty_dark: "Dracula",
            ghostty_light: "Dracula",
            neovim_dark: "dracula",
            neovim_light: "dracula",
        },
        ThemePreset {
            display_name: "Rose Pine",
            ghostty_dark: "Rose Pine",
            ghostty_light: "Rose Pine Dawn",
            neovim_dark: "rose-pine",
            neovim_light: "rose-pine",
        },
        ThemePreset {
            display_name: "Kanagawa",
            ghostty_dark: "Kanagawa",
            ghostty_light: "Kanagawa Light",
            neovim_dark: "kanagawa",
            neovim_light: "kanagawa",
        },
        ThemePreset {
            display_name: "Solarized",
            ghostty_dark: "Solarized Dark",
            ghostty_light: "Solarized Light",
            neovim_dark: "solarized",
            neovim_light: "solarized",
        },
    ]
}

pub fn setup_neovim_integration() -> Result<PathBuf> {
    // Neovim uses ~/.config/nvim on all platforms (XDG style)
    let nvim_config_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".config")
        .join("nvim");

    // Create nvim config dir if it doesn't exist
    fs::create_dir_all(&nvim_config_dir)?;

    let suntheme_lua = nvim_config_dir.join("lua").join("suntheme.lua");
    fs::create_dir_all(suntheme_lua.parent().unwrap())?;

    let lua_content = r#"-- Suntheme integration: watches state file for theme changes
-- Source this file in your init.lua: require("suntheme")

local M = {}

local state_file = vim.fn.has("mac") == 1
  and vim.fn.expand("~/Library/Application Support/suntheme/current_theme")
  or vim.fn.expand("~/.config/suntheme/current_theme")

local function apply_theme()
  local file = io.open(state_file, "r")
  if not file then return end

  local theme, background
  for line in file:lines() do
    local key, value = line:match("^(%w+)=(.+)$")
    if key == "theme" then theme = value end
    if key == "background" then background = value end
  end
  file:close()

  if background then
    vim.o.background = background
  end
  if theme then
    pcall(vim.cmd.colorscheme, theme)
  end
end

function M.setup()
  -- Apply on startup
  apply_theme()

  -- Watch for changes (requires nvim 0.9+)
  if vim.uv then
    local w = vim.uv.new_fs_event()
    if w then
      w:start(state_file, {}, vim.schedule_wrap(function()
        apply_theme()
      end))
    end
  end
end

-- Auto-setup when required
M.setup()

return M
"#;

    fs::write(&suntheme_lua, lua_content)?;

    // Add require("suntheme") to init.lua if not already present
    let init_lua = nvim_config_dir.join("init.lua");
    if init_lua.exists() {
        let init_content = fs::read_to_string(&init_lua)?;
        if !init_content.contains("require(\"suntheme\")")
            && !init_content.contains("require('suntheme')")
        {
            let new_content = format!("require(\"suntheme\")\n{}", init_content);
            fs::write(&init_lua, new_content)?;
        }
    } else {
        // Create init.lua with just the require
        fs::write(&init_lua, "require(\"suntheme\")\n")?;
    }

    Ok(suntheme_lua)
}
