use crate::common::colors::{RED, YELLOW, GREEN, RESET, BOLD};
use crate::display::items::DisplayFormat;
use chrono::Duration;

/// Format a token count based on display format
pub fn format_tokens(count: i64, max_tokens: Option<i64>, format: &DisplayFormat, emoji: &Option<String>) -> String {
    match format {
        DisplayFormat::Text => {
            format!("{} tokens", format_number(count))
        }
        DisplayFormat::TextWithEmoji => {
            let emoji_str = emoji.as_deref().unwrap_or("🧠");
            format!("{} {}", emoji_str, format_number(count))
        }
        DisplayFormat::Compact => {
            format_number_compact(count)
        }
        DisplayFormat::ProgressBar => {
            if let Some(max) = max_tokens {
                let percentage = (count as f64 / max as f64 * 100.0) as u8;
                format_progress_bar(percentage, Some(format_number(count)))
            } else {
                format!("{} tokens", format_number(count))
            }
        }
        DisplayFormat::PercentageOnly => {
            if let Some(max) = max_tokens {
                let percentage = (count as f64 / max as f64 * 100.0) as u8;
                format!("{}%", percentage)
            } else {
                "N/A".to_string()
            }
        }
        _ => format_number(count),
    }
}

/// Format a percentage based on display format
pub fn format_percentage(percentage: u8, format: &DisplayFormat, emoji: &Option<String>) -> String {
    match format {
        DisplayFormat::Text => {
            format!("{}%", percentage)
        }
        DisplayFormat::TextWithEmoji => {
            let emoji_str = emoji.as_deref().unwrap_or("🧠");
            let color = percentage_color(percentage);
            format!("{} {}{}%{}", emoji_str, color, percentage, RESET)
        }
        DisplayFormat::ProgressBar => {
            format_progress_bar(percentage, None)
        }
        DisplayFormat::PercentageOnly => {
            format!("{}%", percentage)
        }
        DisplayFormat::Compact => {
            format!("{}%", percentage)
        }
        _ => format!("{}%", percentage),
    }
}

/// Format a token ratio (current/max) based on display format  
pub fn format_token_ratio(current: i64, max_tokens: Option<i64>, format: &DisplayFormat, emoji: &Option<String>) -> String {
    match format {
        DisplayFormat::Ratio => {
            if let Some(max) = max_tokens {
                format!("{}/{}", format_number_compact(current), format_number_compact(max))
            } else {
                format_number_compact(current)
            }
        }
        DisplayFormat::TextWithEmoji => {
            let emoji_str = emoji.as_deref().unwrap_or("🧠");
            if let Some(max) = max_tokens {
                format!("{} {}/{}", emoji_str, format_number_compact(current), format_number_compact(max))
            } else {
                format!("{} {}", emoji_str, format_number_compact(current))
            }
        }
        _ => {
            if let Some(max) = max_tokens {
                format!("{}/{}", format_number_compact(current), format_number_compact(max))
            } else {
                format_number_compact(current)
            }
        }
    }
}

/// Format a duration based on display format
pub fn format_duration_display(duration: Duration, format: &DisplayFormat, emoji: &Option<String>) -> String {
    let duration_str = format_duration_human(duration);
    
    match format {
        DisplayFormat::Text => duration_str,
        DisplayFormat::TextWithEmoji => {
            let emoji_str = emoji.as_deref().unwrap_or("⏱️");
            format!("{} {}", emoji_str, duration_str)
        }
        DisplayFormat::Compact => {
            format_duration_compact(duration)
        }
        DisplayFormat::Duration => duration_str,
        _ => duration_str,
    }
}

/// Format model name based on display format
pub fn format_model(model: &str, format: &DisplayFormat, emoji: &Option<String>) -> String {
    match format {
        DisplayFormat::Text => model.to_string(),
        DisplayFormat::TextWithEmoji => {
            let emoji_str = emoji.as_deref().unwrap_or("🤖");
            format!("{} {}", emoji_str, model)
        }
        DisplayFormat::Compact => {
            // Just use first 3 chars for compact
            model.chars().take(3).collect()
        }
        _ => model.to_string(),
    }
}

/// Format block status based on current state
pub fn format_block_status(
    status: &crate::analyzer::BlockStatus, 
    format: &DisplayFormat,
    remaining_time: Option<Duration>
) -> String {
    match format {
        DisplayFormat::StatusIcon => {
            match status {
                crate::analyzer::BlockStatus::InCurrentBlock => "🟢".to_string(),
                crate::analyzer::BlockStatus::NeedNewBlock | 
                crate::analyzer::BlockStatus::BeforeCurrentBlock => "🔴".to_string(),
                crate::analyzer::BlockStatus::NoCurrentBlock => "🟡".to_string(),
            }
        }
        DisplayFormat::Text => {
            match status {
                crate::analyzer::BlockStatus::InCurrentBlock => {
                    if let Some(remaining) = remaining_time {
                        format!("ACTIVE ({})", format_duration_human(remaining))
                    } else {
                        "ACTIVE".to_string()
                    }
                }
                crate::analyzer::BlockStatus::NeedNewBlock => "LIMIT REACHED".to_string(),
                crate::analyzer::BlockStatus::BeforeCurrentBlock => "LIMIT".to_string(),
                crate::analyzer::BlockStatus::NoCurrentBlock => "NO BLOCK".to_string(),
            }
        }
        _ => format_block_status(status, &DisplayFormat::Text, remaining_time),
    }
}

// Helper functions

fn format_number(num: i64) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        format!("{}", num)
    }
}

fn format_number_compact(num: i64) -> String {
    if num >= 1_000_000 {
        format!("{:.0}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.0}K", num as f64 / 1_000.0)
    } else {
        format!("{}", num)
    }
}

fn format_duration_human(duration: Duration) -> String {
    let hours = duration.num_hours();
    let minutes = (duration.num_minutes() % 60).abs();
    
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

fn format_duration_compact(duration: Duration) -> String {
    let hours = duration.num_hours();
    let minutes = (duration.num_minutes() % 60).abs();
    
    if hours > 0 {
        format!("{}h", hours)
    } else {
        format!("{}m", minutes)
    }
}

fn format_progress_bar(percentage: u8, label: Option<String>) -> String {
    let width = 10;
    let filled = (percentage as f64 / 100.0 * width as f64) as usize;
    let empty = width - filled;
    
    let color = percentage_color(percentage);
    let bar = format!("{}[{}{}{}{}]{}",
        color,
        BOLD,
        "█".repeat(filled),
        "░".repeat(empty),
        RESET,
        color
    );
    
    if let Some(label) = label {
        format!("{} {} {}%{}", bar, label, percentage, RESET)
    } else {
        format!("{} {}%{}", bar, percentage, RESET)
    }
}

fn percentage_color(percentage: u8) -> &'static str {
    match percentage {
        0..=49 => GREEN,
        50..=79 => YELLOW,
        _ => RED,
    }
}

