# suntheme

Automatically switch Ghostty and Neovim themes based on sunrise/sunset times.

## Installation

```bash
npm install -g @lucianlavric/suntheme
```

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

## More Info

See the [GitHub repo](https://github.com/lucianlavric/suntheme) for full documentation.

## License

MIT
