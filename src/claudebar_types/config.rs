use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::env;
use crate::display::{DisplayItem, StatType, DisplayFormat};

/// Simple block for stats storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleBlock {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>, 
    pub tokens: i64,
}

/// Stats file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsFile {
    /// Past completed blocks (ascending order by start)
    pub past: Vec<SimpleBlock>,
    /// Current active block (if any)
    pub current: Option<SimpleBlock>,
    /// Last processed timestamp
    pub last_processed: Option<DateTime<Utc>>,
}

/// Main configuration for Claude Bar application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
    /// Version of the config format
    pub version: String,
    
    /// Path to Claude Code data directory
    pub claude_data_path: String,
    
    /// Display preferences
    pub display: DisplayConfig,
    
    /// Last processed limit date for caching (most recent non-projected block)
    pub last_limit_date: Option<DateTime<Utc>>,
}

/// Display configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Whether to use colored output
    pub use_colors: bool,
    
    /// List of display items with their configuration
    pub items: Vec<DisplayItem>,
    
    /// Separator between status items
    pub separator: String,
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
            last_limit_date: None,
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            use_colors: true,
            items: vec![
                DisplayItem::new(StatType::TokenUsage, DisplayFormat::TextWithEmoji),
                DisplayItem::new(StatType::TokenPercentage, DisplayFormat::Ratio),
                DisplayItem::new(StatType::BlockStatus, DisplayFormat::StatusIcon),
                DisplayItem::new(StatType::MessageCount, DisplayFormat::TextWithEmoji),
                DisplayItem::new(StatType::TimeRemaining, DisplayFormat::TextWithEmoji),
                DisplayItem::new(StatType::Model, DisplayFormat::TextWithEmoji),
            ],
            separator: " | ".to_string(),
        }
    }
}

impl Default for StatsFile {
    fn default() -> Self {
        Self {
            past: Vec::new(),
            current: None,
            last_processed: None,
        }
    }
}