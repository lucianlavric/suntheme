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
            ghostty_dark: "Gruvbox Dark",
            ghostty_light: "Gruvbox Light",
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
            display_name: "Bluloco",
            ghostty_dark: "Bluloco Dark",
            ghostty_light: "Bluloco Light",
            neovim_dark: "bluloco-dark",
            neovim_light: "bluloco-light",
        },
        ThemePreset {
            display_name: "Rose Pine",
            ghostty_dark: "Rose Pine",
            ghostty_light: "Rose Pine Dawn",
            neovim_dark: "rose-pine",
            neovim_light: "rose-pine",
        },
        ThemePreset {
            display_name: "Horizon",
            ghostty_dark: "Horizon",
            ghostty_light: "Horizon Bright",
            neovim_dark: "horizon",
            neovim_light: "horizon",
        },
        ThemePreset {
            display_name: "One Dark (Atom)",
            ghostty_dark: "Atom One Dark",
            ghostty_light: "Atom One Light",
            neovim_dark: "onedark",
            neovim_light: "onelight",
        },
        ThemePreset {
            display_name: "Everforest",
            ghostty_dark: "Everforest Dark Hard",
            ghostty_light: "Everforest Light Med",
            neovim_dark: "everforest",
            neovim_light: "everforest",
        },
        ThemePreset {
            display_name: "GitHub",
            ghostty_dark: "GitHub Dark",
            ghostty_light: "GitHub Light Default",
            neovim_dark: "github_dark",
            neovim_light: "github_light",
        },
        ThemePreset {
            display_name: "Nightfox",
            ghostty_dark: "Nightfox",
            ghostty_light: "Dayfox",
            neovim_dark: "nightfox",
            neovim_light: "dayfox",
        },
        ThemePreset {
            display_name: "Monokai Pro",
            ghostty_dark: "Monokai Pro",
            ghostty_light: "Monokai Pro Light",
            neovim_dark: "monokai-pro",
            neovim_light: "monokai-pro",
        },
        ThemePreset {
            display_name: "Material",
            ghostty_dark: "Material Dark",
            ghostty_light: "Material",
            neovim_dark: "material",
            neovim_light: "material",
        },
        ThemePreset {
            display_name: "Ayu",
            ghostty_dark: "Ayu",
            ghostty_light: "Ayu Light",
            neovim_dark: "ayu-dark",
            neovim_light: "ayu-light",
        },
        ThemePreset {
            display_name: "Night Owl",
            ghostty_dark: "Night Owl",
            ghostty_light: "Light Owl",
            neovim_dark: "night-owl",
            neovim_light: "night-owl",
        },
        ThemePreset {
            display_name: "Iceberg",
            ghostty_dark: "Iceberg Dark",
            ghostty_light: "Iceberg Light",
            neovim_dark: "iceberg",
            neovim_light: "iceberg",
        },
        ThemePreset {
            display_name: "Flexoki",
            ghostty_dark: "Flexoki Dark",
            ghostty_light: "Flexoki Light",
            neovim_dark: "flexoki-dark",
            neovim_light: "flexoki-light",
        },
        ThemePreset {
            display_name: "Melange",
            ghostty_dark: "Melange Dark",
            ghostty_light: "Melange Light",
            neovim_dark: "melange",
            neovim_light: "melange",
        },
        ThemePreset {
            display_name: "Zenbones",
            ghostty_dark: "Zenbones Dark",
            ghostty_light: "Zenbones Light",
            neovim_dark: "zenbones",
            neovim_light: "zenbones",
        },
        ThemePreset {
            display_name: "Pencil",
            ghostty_dark: "Pencil Dark",
            ghostty_light: "Pencil Light",
            neovim_dark: "pencil",
            neovim_light: "pencil",
        },
        ThemePreset {
            display_name: "Selenized",
            ghostty_dark: "Selenized Dark",
            ghostty_light: "Selenized Light",
            neovim_dark: "selenized",
            neovim_light: "selenized",
        },
        ThemePreset {
            display_name: "Neobones",
            ghostty_dark: "Neobones Dark",
            ghostty_light: "Neobones Light",
            neovim_dark: "neobones",
            neovim_light: "neobones",
        },
        ThemePreset {
            display_name: "Seoulbones",
            ghostty_dark: "Seoulbones Dark",
            ghostty_light: "Seoulbones Light",
            neovim_dark: "seoulbones",
            neovim_light: "seoulbones",
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
