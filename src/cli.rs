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
    Blocks,
    /// List only limit messages with [end, end-5h]
    Resets,
    /// Show detailed help and usage examples
    Help,
    /// Manage configuration settings
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
}

#[derive(Subcommand, Clone)]
pub enum ConfigCommands {
    /// Display help for configuration commands
    Help,
    /// Configure Claude data path
    #[command(name = "claude-path")]
    ClaudePath,
}
