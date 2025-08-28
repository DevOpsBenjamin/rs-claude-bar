use serde::{Deserialize, Serialize};
use std::env;

/// Main configuration for Claude Bar application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
    /// Version of the config format
    pub version: String,
    
    /// Path to Claude Code data directory
    pub claude_data_path: String,
    
    /// Display preferences
    pub display: DisplayConfig,
}

/// Display configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Whether to use colored output
    pub use_colors: bool,
    
    /// Whether to show progress bars
    pub show_progress_bars: bool,
    
    /// Compact display mode
    pub compact_mode: bool,
}

impl Default for ConfigInfo {
    fn default() -> Self {
        // Default Claude data path: ~/.claude
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let default_claude_path = format!("{}/.claude", home);
        
        Self {
            version: "1.0.0".to_string(),
            claude_data_path: default_claude_path,
            display: DisplayConfig::default(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            use_colors: true,
            show_progress_bars: true,
            compact_mode: false,
        }
    }
}