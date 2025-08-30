use serde::{Deserialize, Serialize};

/// Types of stats that can be displayed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatType {
    /// Current session message count
    MessageCount,
    /// Current token usage count
    TokenUsage,
    /// Current model being used
    Model,
    /// Time elapsed in current window
    TimeElapsed,
    /// Time remaining in current window
    TimeRemaining,
    /// Current block status (ACTIVE/LIMIT/etc.)
    BlockStatus,
}

/// How a stat should be formatted/displayed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisplayFormat {
    /// Simple text: "1,234 tokens"
    Text,
    /// With emoji: "🧠 1,234"
    TextWithEmoji,
    /// Progress bar: "[████████░░] 80%"
    ProgressBar,
    /// Compact: "1.2K"
    Compact,
    /// Percentage only: "53%"
    PercentageOnly,
    /// Time format: "2h 15m"
    Duration,
    /// Just the status indicator: "🟢"
    StatusIcon,
    /// Ratio format: "48.7K/70K"
    Ratio,
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

impl DisplayItem {
    pub fn new(stat_type: StatType, format: DisplayFormat) -> Self {
        Self {
            stat_type,
            format,
            enabled: true,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    
    pub fn default_emoji(stat_type: &StatType) -> Option<String> {
        match stat_type {
            StatType::MessageCount => Some("💬".to_string()),
            StatType::TokenUsage => Some("🧠".to_string()),
            StatType::TimeElapsed => Some("⏱️".to_string()),
            StatType::TimeRemaining => Some("⏰".to_string()),
            StatType::Model => Some("🤖".to_string()),
            StatType::BlockStatus => None, // Status has its own icons
        }
    }
}

impl Default for DisplayItem {
    fn default() -> Self {
        Self::new(StatType::TokenUsage, DisplayFormat::TextWithEmoji)
    }
}