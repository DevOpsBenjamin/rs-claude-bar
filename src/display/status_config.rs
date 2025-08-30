use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::analyze::Analyzer;


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
        format!("{}", num)
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