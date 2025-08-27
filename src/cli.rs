use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rs-claude-bar", about = "Track Claude usage", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Override data path (default: ~/.claude/projects)
    #[arg(long, global = true)]
    pub data_path: Option<String>,
    
    /// Use mock data mode
    #[arg(long, global = true)]
    pub mock_data: bool,
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
}
