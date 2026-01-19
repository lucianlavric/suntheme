use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub fn get_ghostty_themes() -> Result<HashSet<String>> {
    let mut themes = HashSet::new();

    // Check user themes directory (Linux: ~/.config/ghostty/themes)
    if let Some(config_dir) = dirs::config_dir() {
        let user_themes = config_dir.join("ghostty").join("themes");
        if user_themes.exists() {
            collect_themes_from_dir(&user_themes, &mut themes)?;
        }
    }

    // Check macOS user themes (~/Library/Application Support/com.mitchellh.ghostty/themes)
    if let Some(data_dir) = dirs::data_dir() {
        let macos_themes = data_dir.join("com.mitchellh.ghostty").join("themes");
        if macos_themes.exists() {
            collect_themes_from_dir(&macos_themes, &mut themes)?;
        }
    }

    // Check system/bundled themes
    let system_paths = [
        // macOS app bundle
        PathBuf::from("/Applications/Ghostty.app/Contents/Resources/ghostty/themes"),
    ];

    for path in system_paths {
        if path.exists() {
            collect_themes_from_dir(&path, &mut themes)?;
        }
    }

    Ok(themes)
}

fn collect_themes_from_dir(dir: &PathBuf, themes: &mut HashSet<String>) -> Result<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem() {
                    themes.insert(name.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}

pub fn validate_ghostty_theme(theme: &str, available: &HashSet<String>) -> bool {
    available.contains(theme)
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
        if !init_content.contains("require(\"suntheme\")") && !init_content.contains("require('suntheme')") {
            let new_content = format!("require(\"suntheme\")\n{}", init_content);
            fs::write(&init_lua, new_content)?;
        }
    } else {
        // Create init.lua with just the require
        fs::write(&init_lua, "require(\"suntheme\")\n")?;
    }

    Ok(suntheme_lua)
}
