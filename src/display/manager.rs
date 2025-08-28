use chrono::{DateTime, Utc, Duration};
use crate::{
    claudebar_types::StatsFile,
    analyze::{BlockStatus, detect_block_status},
    colors::*,
};
use super::{DisplayItem, StatType, DisplayFormat, formats};

/// The Display Manager handles rendering the status line based on configuration
pub struct DisplayManager {
    pub items: Vec<DisplayItem>,
    pub separator: String,
    pub show_colors: bool,
}

/// Current usage data for display
pub struct DisplayData {
    pub current_tokens: i64,
    pub max_tokens: Option<i64>,  // 5-hour limit if known
    pub percentage: Option<u8>,
    pub time_elapsed: Option<Duration>,
    pub time_remaining: Option<Duration>,
    pub reset_time: Option<DateTime<Utc>>,
    pub model: Option<String>,
    pub message_count: Option<u32>,
    pub block_status: BlockStatus,
}

impl DisplayManager {
    pub fn new(items: Vec<DisplayItem>) -> Self {
        Self {
            items,
            separator: " | ".to_string(),
            show_colors: should_use_colors_for_status(),
        }
    }
    
    pub fn with_separator(mut self, separator: String) -> Self {
        self.separator = separator;
        self
    }
    
    /// Generate the complete status line
    pub fn render_status_line(&self, data: &DisplayData) -> String {
        let enabled_items: Vec<String> = self.items
            .iter()
            .filter(|item| item.enabled)
            .map(|item| self.render_item(item, data))
            .filter(|rendered| !rendered.is_empty())
            .collect();
        
        if enabled_items.is_empty() {
            return "No display items configured".to_string();
        }
        
        enabled_items.join(&self.separator)
    }
    
    /// Render a single display item
    fn render_item(&self, item: &DisplayItem, data: &DisplayData) -> String {
        match &item.stat_type {
            StatType::TokenUsage => {
                formats::format_tokens(data.current_tokens, data.max_tokens, &item.format, &item.emoji)
            }
            StatType::TokenPercentage => {
                match &item.format {
                    DisplayFormat::Ratio => {
                        formats::format_token_ratio(data.current_tokens, data.max_tokens, &item.format, &item.emoji)
                    }
                    _ => {
                        if let Some(percentage) = data.percentage {
                            formats::format_percentage(percentage, &item.format, &item.emoji)
                        } else {
                            String::new()
                        }
                    }
                }
            }
            StatType::TimeElapsed => {
                if let Some(elapsed) = data.time_elapsed {
                    formats::format_duration_display(elapsed, &item.format, &item.emoji)
                } else {
                    String::new()
                }
            }
            StatType::TimeRemaining => {
                if let Some(remaining) = data.time_remaining {
                    formats::format_duration_display(remaining, &item.format, &item.emoji)
                } else {
                    String::new()
                }
            }
            StatType::ResetTime => {
                if let Some(reset) = data.reset_time {
                    let emoji_str = item.emoji.as_deref().unwrap_or("🔄");
                    match &item.format {
                        DisplayFormat::Text => reset.format("%H:%M").to_string(),
                        DisplayFormat::TextWithEmoji => format!("{} {}", emoji_str, reset.format("%H:%M")),
                        DisplayFormat::Compact => reset.format("%H:%M").to_string(),
                        _ => reset.format("%H:%M").to_string(),
                    }
                } else {
                    String::new()
                }
            }
            StatType::Model => {
                if let Some(model) = &data.model {
                    formats::format_model(model, &item.format, &item.emoji)
                } else {
                    String::new()
                }
            }
            StatType::MessageCount => {
                if let Some(count) = data.message_count {
                    let emoji_str = item.emoji.as_deref().unwrap_or("💬");
                    match &item.format {
                        DisplayFormat::Text => format!("{} messages", count),
                        DisplayFormat::TextWithEmoji => format!("{} {}", emoji_str, count),
                        DisplayFormat::Compact => format!("{}", count),
                        _ => format!("{}", count),
                    }
                } else {
                    String::new()
                }
            }
            StatType::BlockStatus => {
                formats::format_block_status(&data.block_status, &item.format, data.time_remaining)
            }
        }
    }
    
    /// Create DisplayData from StatsFile
    pub fn create_display_data(stats: &StatsFile) -> DisplayData {
        Self::create_display_data_with_model(stats, None)
    }
    
    /// Create DisplayData from StatsFile with optional model info
    pub fn create_display_data_with_model(stats: &StatsFile, model_name: Option<String>) -> DisplayData {
        let now = Utc::now();
        let block_status = detect_block_status(now, &stats.current);
        
        let (current_tokens, time_elapsed, time_remaining, reset_time) = match &stats.current {
            Some(current) => {
                let tokens = current.tokens;
                let elapsed = now.signed_duration_since(current.start);
                let remaining = current.end.signed_duration_since(now);
                let reset = if remaining.num_seconds() < 0 { Some(current.start) } else { None };
                (tokens, Some(elapsed), Some(remaining), reset)
            }
            None => (0, None, None, None),
        };
        
        // Calculate percentage based on historical average (from recent data: ~70K average)
        let historical_average = 70000; // Based on analysis of past blocks 
        let max_tokens = Some(historical_average);
        let percentage = max_tokens.map(|max| {
            ((current_tokens as f64 / max as f64) * 100.0) as u8
        });
        
        DisplayData {
            current_tokens,
            max_tokens,
            percentage,
            time_elapsed,
            time_remaining,
            reset_time,
            model: model_name, // Use provided model name from Claude Code input
            message_count: None, // TODO: Count messages in current block
            block_status,
        }
    }
    
    /// Create default display configuration
    pub fn default_config() -> Vec<DisplayItem> {
        vec![
            DisplayItem::new(StatType::TokenUsage, DisplayFormat::TextWithEmoji),
            DisplayItem::new(StatType::TokenPercentage, DisplayFormat::Ratio),
            DisplayItem::new(StatType::BlockStatus, DisplayFormat::StatusIcon),
            DisplayItem::new(StatType::MessageCount, DisplayFormat::TextWithEmoji),
            DisplayItem::new(StatType::TimeRemaining, DisplayFormat::TextWithEmoji), 
            DisplayItem::new(StatType::Model, DisplayFormat::TextWithEmoji),
        ]
    }
}

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new(Self::default_config())
    }
}