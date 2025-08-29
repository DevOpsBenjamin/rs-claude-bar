use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rs-claude-bar", about = "Track Claude usage", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Show basic usage information
    Info,
    /// Show detailed Help with examples
    Help,
    /// Show line prompt (basic use of the app)
    Prompt,
    /// Install command to configure Claude settings
    Install,
    /// Interactively reset display configuration
    #[command(name = "display-config")]
    DisplayConfig,
    /// Manage configuration settings
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
    /// Display last 5-hour usage blocks
    Blocks,


    /// Debug parse JSONL files in specified directory
    Debug {
        /// Show detailed parsing statistics with ANSI table for all files (V1 - reliable)
        #[arg(long)]
        parse: bool,
        /// Use new cached analysis system (V2 - development)
        #[arg(long)]
        cache: bool,
        /// Target specific file for detailed error analysis
        #[arg(long, value_name = "FILEPATH")]
        file: Option<String>,
        /// Show blocks debug information
        #[arg(long)]
        blocks: bool,
        /// Show gaps analysis (sessions with gaps > 1 hour)
        #[arg(long)]
        gaps: bool,
        /// Show limit messages analysis
        #[arg(long)]
        limits: bool,
        /// Show file system information for all Claude data folders and files
        #[arg(long)]
        files: bool,
        /// Force full reparse without using cache (only works with --cache)
        #[arg(long)]
        no_cache: bool,
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
