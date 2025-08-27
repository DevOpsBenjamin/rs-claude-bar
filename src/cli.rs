use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rs-claude-bar", about = "Track Claude usage", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Show current Claude status
    Status,
    /// Force refresh of cached stats
    Update,
    /// Show recent usage windows
    History,
    /// Display detailed statistics
    Stats,
    /// Interactively reset display configuration
    #[command(name = "display-config")]
    DisplayConfig,
}
