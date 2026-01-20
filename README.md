# suntheme
<img width="642" height="230" alt="image" src="https://github.com/user-attachments/assets/9cc57ce1-3795-4ec5-a463-0f9640c09b10" />

Automatically switch Ghostty and Neovim themes based on sunrise/sunset times.

## Installation

### Homebrew (macOS)
```bash
brew install lucianlavric/tap/suntheme
```

### Linux
```bash
curl -fsSL https://raw.githubusercontent.com/lucianlavric/suntheme/main/install.sh | bash
```

### Cargo (all platforms)
```bash
cargo install suntheme
```

> **Note**: After `cargo install`, ensure `~/.cargo/bin` is in your PATH:
> ```bash
> echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
> source ~/.bashrc
> ```

## Quick Start

```bash
# Initial setup - configures location and themes
suntheme init

# Start automatic theme switching daemon
suntheme start
```

## Commands

| Command | Description |
|---------|-------------|
| `suntheme init` | Interactive setup for location and themes |
| `suntheme start` | Start daemon for automatic switching |
| `suntheme stop` | Stop the daemon |
| `suntheme status` | Show daemon status and current theme |
| `suntheme toggle` | Toggle between light/dark |
| `suntheme set <light\|dark>` | Set specific mode |
| `suntheme sun` | Display today's sunrise/sunset times |
| `suntheme themes` | Change theme configuration |

## How it works

1. **Location-based sun times**: Fetches sunrise/sunset times from [sunrise-sunset.org](https://sunrise-sunset.org) API based on your location
2. **Daily caching**: Sun times are cached daily to minimize API calls
3. **Ghostty**: Modifies your Ghostty config file and triggers a reload
4. **Neovim**: Writes to a state file that Neovim watches for changes

## Requirements

- **macOS**: Accessibility permissions required for Ghostty auto-reload (prompted during setup)
- **Linux**: Auto-reload works via SIGUSR2 signal (no extra dependencies)
- **Neovim 0.9+**: For file watching support
- **Ghostty**: Any recent version

## Configuration

Config is stored at:
- macOS: `~/Library/Application Support/suntheme/config.toml`
- Linux: `~/.config/suntheme/config.toml`

```toml
[location]
latitude = 43.6532
longitude = -79.3832

[themes.ghostty]
light = "tokyonight-day"
dark = "tokyonight"

[themes.neovim]
light = "tokyonight-day"
dark = "tokyonight"
```

## Recommended Themes

Themes with both Ghostty and Neovim support:

| Theme | Ghostty (dark/light) | Neovim Plugin |
|-------|---------------------|---------------|
| Tokyo Night | `tokyonight` / `tokyonight-day` | [folke/tokyonight.nvim](https://github.com/folke/tokyonight.nvim) |
| Gruvbox | `gruvbox-dark` / `gruvbox-light` | [ellisonleao/gruvbox.nvim](https://github.com/ellisonleao/gruvbox.nvim) |
| Catppuccin | `catppuccin-mocha` / `catppuccin-latte` | [catppuccin/nvim](https://github.com/catppuccin/nvim) |
| Nord | `nord` | [shaunsingh/nord.nvim](https://github.com/shaunsingh/nord.nvim) |
| Dracula | `dracula` | [dracula/vim](https://github.com/dracula/vim) |
| Rose Pine | `rose-pine` / `rose-pine-dawn` | [rose-pine/neovim](https://github.com/rose-pine/neovim) |
| Kanagawa | `kanagawa` | [rebelot/kanagawa.nvim](https://github.com/rebelot/kanagawa.nvim) |
| Solarized | `solarized-dark` / `solarized-light` | [maxmx03/solarized.nvim](https://github.com/maxmx03/solarized.nvim) |

> **Note**: For Neovim, use the same theme name for both light and dark (e.g., `gruvbox`). The `background` setting handles the variant automatically.

## Roadmap

- Windows support
- Linux support
- iTerm2 support
- VS Code support
- System theme support
- Gradient transition between light and dark themes to mimic sunrise/sunset

## License

MIT
