use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::analyze::Analyzer;

/// Types of stats that can be displayed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum StatType {
    // Token metrics
    TokenUsage,
    TokenProgress,     // Requires limit block context
    TokenRemaining,    // Requires limit block context
    
    // Time metrics  
    TimeElapsed,
    TimeRemaining,
    SessionDuration,
    
    // Block/Status metrics
    BlockStatus,
    BlockType,         // Limit vs Gap vs Current
    
    // Message metrics
    MessageCount,
    AssistantMessages,
    UserMessages,
    
    // Model info
    Model,
    ModelShort,        // Abbreviated version
    
    // Activity indicators
    ActivityStatus,    // Active/Idle/Limited
}

/// How a stat should be formatted/displayed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisplayFormat {
    // Text formats
    Text,              // "1,234 tokens"
    TextWithEmoji,     // "🧠 1,234"
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

/// Definition of a metric and its supported formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    pub stat_type: StatType,
    pub name: String,
    pub description: String,
    pub supported_formats: Vec<DisplayFormat>,
    pub default_format: DisplayFormat,
    pub enabled_by_default: bool,
}

/// Registry of all available metrics
pub struct MetricRegistry {
    metrics: HashMap<StatType, MetricDefinition>,
}

impl MetricRegistry {
    pub fn new() -> Self {
        let mut metrics = HashMap::new();
        
        // Token metrics
        metrics.insert(StatType::TokenUsage, MetricDefinition {
            stat_type: StatType::TokenUsage,
            name: "Token Usage".to_string(),
            description: "Current token count in active block".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji, 
                DisplayFormat::Compact,
                DisplayFormat::Ratio,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::TokenProgress, MetricDefinition {
            stat_type: StatType::TokenProgress,
            name: "Token Progress".to_string(), 
            description: "Progress through current block limit".to_string(),
            supported_formats: vec![
                DisplayFormat::ProgressBar,
                DisplayFormat::PercentageOnly,
                DisplayFormat::StatusColored,
            ],
            default_format: DisplayFormat::PercentageOnly,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::TimeElapsed, MetricDefinition {
            stat_type: StatType::TimeElapsed,
            name: "Time Elapsed".to_string(),
            description: "Time spent in current block".to_string(),
            supported_formats: vec![
                DisplayFormat::Duration,
                DisplayFormat::DurationShort,
                DisplayFormat::Text,
            ],
            default_format: DisplayFormat::Duration,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::TimeRemaining, MetricDefinition {
            stat_type: StatType::TimeRemaining,
            name: "Time Remaining".to_string(),
            description: "Time left in current block".to_string(),
            supported_formats: vec![
                DisplayFormat::Duration,
                DisplayFormat::DurationShort,
                DisplayFormat::Text,
            ],
            default_format: DisplayFormat::Duration,
            enabled_by_default: false,
        });
        
        metrics.insert(StatType::MessageCount, MetricDefinition {
            stat_type: StatType::MessageCount,
            name: "Message Count".to_string(),
            description: "Total messages in current block".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji,
                DisplayFormat::Compact,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::Model, MetricDefinition {
            stat_type: StatType::Model,
            name: "Model Name".to_string(),
            description: "Current Claude model being used".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji,
                DisplayFormat::Compact,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::BlockStatus, MetricDefinition {
            stat_type: StatType::BlockStatus,
            name: "Block Status".to_string(),
            description: "Current block type (Active/Limited/Gap)".to_string(),
            supported_formats: vec![
                DisplayFormat::StatusIcon,
                DisplayFormat::StatusText,
                DisplayFormat::StatusColored,
            ],
            default_format: DisplayFormat::StatusIcon,
            enabled_by_default: false,
        });
        
        metrics.insert(StatType::TokenRemaining, MetricDefinition {
            stat_type: StatType::TokenRemaining,
            name: "Tokens Remaining".to_string(),
            description: "Tokens left before hitting limit".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji,
                DisplayFormat::Compact,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: false,
        });
        
        metrics.insert(StatType::AssistantMessages, MetricDefinition {
            stat_type: StatType::AssistantMessages,
            name: "Assistant Messages".to_string(),
            description: "Number of assistant responses".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji,
                DisplayFormat::Compact,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: false,
        });
        
        metrics.insert(StatType::UserMessages, MetricDefinition {
            stat_type: StatType::UserMessages,
            name: "User Messages".to_string(),
            description: "Number of user messages".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji,
                DisplayFormat::Compact,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: false,
        });
        
        metrics.insert(StatType::ActivityStatus, MetricDefinition {
            stat_type: StatType::ActivityStatus,
            name: "Activity Status".to_string(),
            description: "Overall activity indicator".to_string(),
            supported_formats: vec![
                DisplayFormat::StatusIcon,
                DisplayFormat::StatusText,
            ],
            default_format: DisplayFormat::StatusIcon,
            enabled_by_default: false,
        });
        
        Self { metrics }
    }
    
    pub fn get_metric(&self, stat_type: &StatType) -> Option<&MetricDefinition> {
        self.metrics.get(stat_type)
    }
    
    pub fn all_metrics(&self) -> Vec<&MetricDefinition> {
        let mut metrics: Vec<_> = self.metrics.values().collect();
        // Sort by importance/common usage
        metrics.sort_by_key(|m| match m.stat_type {
            StatType::TokenUsage => 0,
            StatType::TokenProgress => 1,
            StatType::TimeElapsed => 2,
            StatType::TimeRemaining => 3,
            StatType::MessageCount => 4,
            StatType::Model => 5,
            StatType::BlockStatus => 6,
            _ => 99,
        });
        metrics
    }
}

/// Mock data for testing configuration UI
pub struct MockData {
    pub tokens_used: i64,
    pub tokens_limit: i64,
    pub progress_percent: f64,
    pub time_elapsed_hours: i32,
    pub time_elapsed_minutes: i32,
    pub time_remaining_hours: i32,
    pub time_remaining_minutes: i32,
    pub message_count: i32,
    pub assistant_messages: i32,
    pub user_messages: i32,
    pub model_name: String,
    pub model_short: String,
    pub block_status: String,
    pub is_limited: bool,
}

impl Default for MockData {
    fn default() -> Self {
        Self {
            tokens_used: 15234,
            tokens_limit: 28400,
            progress_percent: 53.6,
            time_elapsed_hours: 2,
            time_elapsed_minutes: 15,
            time_remaining_hours: 2,
            time_remaining_minutes: 45,
            message_count: 48,
            assistant_messages: 24,
            user_messages: 24,
            model_name: "Claude 3.5 Sonnet".to_string(),
            model_short: "Sonnet".to_string(),
            block_status: "ACTIVE".to_string(),
            is_limited: false,
        }
    }
}

/// Generate a realistic example using mock data
pub fn generate_format_example_mock(stat_type: StatType, format: &DisplayFormat) -> String {
    let mock = MockData::default();
    
    match (&stat_type, format) {
        // Token Usage Examples
        (StatType::TokenUsage, DisplayFormat::Text) => format!("{} tokens", format_number(mock.tokens_used)),
        (StatType::TokenUsage, DisplayFormat::TextWithEmoji) => format!("🧠 {}", format_number(mock.tokens_used)),
        (StatType::TokenUsage, DisplayFormat::Compact) => format_number_compact(mock.tokens_used),
        (StatType::TokenUsage, DisplayFormat::Ratio) => format!("{}/{}", 
            format_number_compact(mock.tokens_used), 
            format_number_compact(mock.tokens_limit)),
        
        // Token Progress Examples  
        (StatType::TokenProgress, DisplayFormat::ProgressBar) => {
            let filled = (mock.progress_percent / 10.0) as usize;
            let empty = 10 - filled;
            format!("[{}{}] {:.1}%", 
                "█".repeat(filled), 
                "░".repeat(empty), 
                mock.progress_percent)
        },
        (StatType::TokenProgress, DisplayFormat::PercentageOnly) => format!("{:.1}%", mock.progress_percent),
        (StatType::TokenProgress, DisplayFormat::StatusColored) => {
            if mock.progress_percent < 50.0 { "🟢 Good" }
            else if mock.progress_percent < 80.0 { "🟡 Near Limit" }
            else { "🔴 Close to Limit" }
        }.to_string(),
        
        // Time Examples
        (StatType::TimeElapsed, DisplayFormat::Duration) => format!("{}h {:02}m", mock.time_elapsed_hours, mock.time_elapsed_minutes),
        (StatType::TimeElapsed, DisplayFormat::DurationShort) => format!("{}h{:02}m", mock.time_elapsed_hours, mock.time_elapsed_minutes),
        (StatType::TimeElapsed, DisplayFormat::Text) => format!("elapsed {}h {:02}m", mock.time_elapsed_hours, mock.time_elapsed_minutes),
        
        (StatType::TimeRemaining, DisplayFormat::Duration) => format!("{}h {:02}m left", mock.time_remaining_hours, mock.time_remaining_minutes),
        (StatType::TimeRemaining, DisplayFormat::DurationShort) => format!("{}h{:02}m", mock.time_remaining_hours, mock.time_remaining_minutes),
        (StatType::TimeRemaining, DisplayFormat::Text) => format!("{}h {:02}m remaining", mock.time_remaining_hours, mock.time_remaining_minutes),
        
        // Message Examples
        (StatType::MessageCount, DisplayFormat::Text) => format!("{} messages", mock.message_count),
        (StatType::MessageCount, DisplayFormat::TextWithEmoji) => format!("💬 {}", mock.message_count),
        (StatType::MessageCount, DisplayFormat::Compact) => format!("{}", mock.message_count),
        
        (StatType::AssistantMessages, DisplayFormat::Text) => format!("{} assistant", mock.assistant_messages),
        (StatType::AssistantMessages, DisplayFormat::TextWithEmoji) => format!("🤖 {}", mock.assistant_messages),
        (StatType::AssistantMessages, DisplayFormat::Compact) => format!("{}", mock.assistant_messages),
        
        (StatType::UserMessages, DisplayFormat::Text) => format!("{} user", mock.user_messages),
        (StatType::UserMessages, DisplayFormat::TextWithEmoji) => format!("👤 {}", mock.user_messages),
        (StatType::UserMessages, DisplayFormat::Compact) => format!("{}", mock.user_messages),
        
        // Model Examples
        (StatType::Model, DisplayFormat::Text) => mock.model_name.clone(),
        (StatType::Model, DisplayFormat::TextWithEmoji) => format!("🤖 {}", mock.model_short),
        (StatType::Model, DisplayFormat::Compact) => mock.model_short.clone(),
        
        (StatType::ModelShort, DisplayFormat::Text) => mock.model_short.clone(),
        (StatType::ModelShort, DisplayFormat::TextWithEmoji) => format!("🤖 {}", mock.model_short),
        (StatType::ModelShort, DisplayFormat::Compact) => mock.model_short.clone(),
        
        // Status Examples
        (StatType::BlockStatus, DisplayFormat::StatusIcon) => {
            if mock.is_limited { "🚫" } else { "🟢" }
        }.to_string(),
        (StatType::BlockStatus, DisplayFormat::StatusText) => mock.block_status.clone(),
        (StatType::BlockStatus, DisplayFormat::StatusColored) => {
            if mock.is_limited { 
                format!("\x1b[31m{}\x1b[0m", mock.block_status)
            } else { 
                format!("\x1b[32m{}\x1b[0m", mock.block_status) 
            }
        },
        
        (StatType::ActivityStatus, DisplayFormat::StatusIcon) => {
            if mock.is_limited { "🚫" }
            else if mock.progress_percent > 80.0 { "⚡" }
            else { "🧠" }
        }.to_string(),
        (StatType::ActivityStatus, DisplayFormat::StatusText) => {
            if mock.is_limited { "LIMITED" }
            else if mock.progress_percent > 80.0 { "BUSY" }
            else { "ACTIVE" }
        }.to_string(),
        
        // Block Type Examples
        (StatType::BlockType, DisplayFormat::StatusText) => "GAP".to_string(),
        (StatType::BlockType, DisplayFormat::StatusIcon) => "🔄".to_string(),
        (StatType::BlockType, DisplayFormat::StatusColored) => "\x1b[36mGAP\x1b[0m".to_string(),
        
        // Session Duration
        (StatType::SessionDuration, DisplayFormat::Duration) => format!("{}h {:02}m", mock.time_elapsed_hours + 1, mock.time_elapsed_minutes),
        (StatType::SessionDuration, DisplayFormat::DurationShort) => format!("{}h{:02}m", mock.time_elapsed_hours + 1, mock.time_elapsed_minutes),
        (StatType::SessionDuration, DisplayFormat::Text) => format!("session {}h {:02}m", mock.time_elapsed_hours + 1, mock.time_elapsed_minutes),
        
        // Token Remaining
        (StatType::TokenRemaining, DisplayFormat::Text) => format!("{} left", format_number(mock.tokens_limit - mock.tokens_used)),
        (StatType::TokenRemaining, DisplayFormat::TextWithEmoji) => format!("⏳ {}", format_number_compact(mock.tokens_limit - mock.tokens_used)),
        (StatType::TokenRemaining, DisplayFormat::Compact) => format_number_compact(mock.tokens_limit - mock.tokens_used),
        
        // Fallbacks
        _ => "Example".to_string(),
    }
}

fn format_number(num: i64) -> String {
    if num >= 1000 {
        format!("{:,}", num)
    } else {
        format!("{}", num)
    }
}

fn format_number_compact(num: i64) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        format!("{}", num)
    }
}