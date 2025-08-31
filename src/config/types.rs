use serde::{Deserialize, Serialize};


/// Main configuration for Claude Bar application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
    /// Version of the config format
    pub version: String,
    
    /// Path to Claude Code data directory
    pub claude_data_path: String,
    
    /// Display preferences
    pub display: StatusLineConfig,
}
impl Default for ConfigInfo {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            claude_data_path: "~/.claude/".to_string(),
            display: StatusLineConfig::default(),
        }
    }
    
}

/// User's configuration for the status line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLineConfig {
    pub items: Vec<DisplayItem>,
    pub separator: String,  // " | " by default
    pub max_width: Option<usize>,
}

impl Default for StatusLineConfig {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            separator: " | ".to_string(),
            max_width: None,
        }
    }
}

/// A configurable display item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayItem {
    /// What type of stat this is
    pub stat_type: StatType,
    /// How it should be formatted
    pub format: DisplayFormat,
    /// Whether it's currently enabled
    pub enabled: bool,
}

/// Types of stats that can be displayed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum StatType {
    // Token metrics
    TokenUsage,
    TokenProgress,     // Requires limit block context    
    // Time metrics  
    TimeElapsed,
    TimeRemaining,    
    // Block/Status metrics
    BlockStatus,    
    // Message metrics
    MessageCount,    
    // Model info
    Model,    
    // Activity indicators
    ActivityStatus,    // Active/Idle/Limited
}

/// How a stat should be formatted/displayed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisplayFormat {
    // Text formats
    Text,              // "1,234 tokens"
    TextWithEmoji,     // "🧠 1,2K"
    Compact,           // "1.2K"
    
    // Progress formats (require context)
    ProgressBar,       // "[████████░░] 80%"  
    PercentageOnly,    // "80%"
    Ratio,             // "48.7K/70K"
    
    // Time formats
    Duration,          // "2h 15m"
    DurationShort,     // "2h15m" 
    
    // Status formats
    StatusIcon,        // "🟢"
    StatusText,        // "ACTIVE"
    StatusColored,     // Colored text based on status
    
    // Special formats
    Hidden,            // Present but not displayed
}
