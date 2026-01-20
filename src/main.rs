use anyhow::Result;
use clap::{Parser, Subcommand};

mod banner;
mod commands;
mod config;
mod sun_times;
mod theme_switcher;
mod themes;

use sun_times::ThemeMode;

#[derive(Parser)]
#[command(name = "suntheme")]
#[command(about = "Switch Ghostty and Neovim themes based on sunrise/sunset times")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize suntheme with location and theme preferences
    Init,

    /// Start the daemon for automatic theme switching
    Start,

    /// Stop the running daemon
    Stop,

    /// Show daemon status and current theme
    Status,

    /// Toggle between light and dark themes
    Toggle,

    /// Set a specific theme mode
    Set {
        /// Theme mode: light or dark
        mode: ThemeMode,
    },

    /// Display today's sunrise and sunset times
    Sun,

    /// Configure theme names for Ghostty and Neovim
    Themes,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run(),
        Commands::Start => commands::daemon::start(),
        Commands::Stop => commands::daemon::stop(),
        Commands::Status => commands::daemon::status(),
        Commands::Toggle => commands::theme::toggle(),
        Commands::Set { mode } => commands::theme::set(mode),
        Commands::Sun => commands::sun::run(),
        Commands::Themes => commands::theme::configure_themes(),
    }
}
