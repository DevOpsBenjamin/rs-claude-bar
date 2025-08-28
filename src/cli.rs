use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rs-claude-bar", about = "Track Claude usage", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Show help information
    Help,
    /// Show status line prompt
    Prompt,
    /// Show current Claude status
    Update,
    /// Show recent usage windows
    History,
    /// Display detailed statistics
    Stats,
    /// Interactively reset display configuration
    #[command(name = "display-config")]
    DisplayConfig,
    /// Debug parse JSONL files in specified directory
    Debug {
        /// Show detailed parsing statistics with ANSI table for all files
        #[arg(long)]
        parse: bool,
        /// Target specific file for detailed error analysis
        #[arg(long, value_name = "FILEPATH")]
        file: Option<String>,
    },
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
        /// Show all limit messages with timestamps and file paths (requires --debug)
        #[arg(long, requires = "debug")]
        limits: bool,
    },
    /// List only limit messages with [end, end-5h]
    Resets,
    /// Install command to configure Claude settings
    Install,
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
