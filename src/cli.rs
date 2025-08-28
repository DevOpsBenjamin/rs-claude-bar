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
    /// Debug parse JSONL files in specified directory
    Debug,
    /// Show usage data in table format
    Table,
    /// Analyze and display 5-hour usage blocks
    Blocks {
        /// Show debug information for blocks analysis
        #[arg(long)]
        debug: bool,
        /// Show only gap analysis (requires --debug)
        #[arg(long, requires = "debug")]
        gaps: bool,
    },
    /// List only limit messages with [end, end-5h]
    Resets,
    /// Manage configuration settings
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
}

#[derive(Subcommand, Clone)]
pub enum ConfigCommands {
    /// Configure Claude data path
    #[command(name = "claude-path")]
    ClaudePath,
    /// Configure display settings
    #[command(name = "display")]
    Display,
}
