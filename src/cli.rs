use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rs-claude-bar", about = "Track Claude usage", version)]
pub struct Cli {
    /// Force bypass cache and reprocess all files
    #[arg(long, global = true)]
    pub no_cache: bool,
    
    /// Don't save cache after processing
    #[arg(long, global = true)]
    pub no_save: bool,
    
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Show basic usage information
    Info,
    /// Show detailed Help with examples
    Help{        
        #[command(subcommand)]
        command: Option<HelpCommands>,
    },
    /// Show line prompt (basic use of the app)
    Prompt,
    /// Install command to configure Claude settings
    Install,
    /// Manage configuration settings
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
    /// Display last 5-hour usage blocks
    Blocks,
}

#[derive(Subcommand, Clone)]
pub enum HelpCommands {
    /// Configure Claude data path
    #[command(name = "config")]
    Config,
    /// Configure Claude data path
    #[command(name = "prompt")]
    Prompt,
    /// Configure Claude data path
    #[command(name = "install")]
    Install,
    /// Configure Claude data path
    #[command(name = "blocks")]
    Blocks,    
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
