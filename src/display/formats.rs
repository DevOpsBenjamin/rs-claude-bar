use std::cmp::min;

use crate::{common::colors::{BOLD, GREEN, RED, RESET, YELLOW}, config::{DisplayFormat, StatType}, display::prompt::PromptData};


/// Generate a realistic example using data data
pub fn generate_stat_with_format(data: &PromptData, stat_type: &StatType, display: &DisplayFormat) -> String {
       
    match stat_type {
        StatType::TokenUsage => generate_token_with_format(data, display),
        StatType::TokenProgress => generate_progress_with_format(data, display),
        StatType::TimeElapsed => generate_elapsed_with_format(data, display),
        StatType::TimeRemaining => generate_remaining_with_format(data, display),
        StatType::MessageCount => generate_message_with_format(data, display),
        StatType::Model => generate_model_with_format(data, display),
        StatType::BlockStatus => generate_status_with_format(data, display),       

        /*
        (StatType::ActivityStatus, DisplayFormat::StatusIcon) => {
            if data.is_limited { "🚫" }
            else if data.progress_percent > 80.0 { "⚡" }
            else { "🧠" }
        }.to_string(),
        (StatType::ActivityStatus, DisplayFormat::StatusText) => {
            if data.is_limited { "LIMITED" }
            else if data.progress_percent > 80.0 { "BUSY" }
            else { "ACTIVE" }
        }.to_string(),
        
        */        
        // Fallbacks
        _ => "Example".to_string(),
    }
}


fn generate_token_with_format(data: &PromptData, display: &DisplayFormat) -> String  {
    match display {
        DisplayFormat::TextWithEmoji => format!("🧠 {}", format_number(data.tokens_used)),
        DisplayFormat::Compact => format_number_compact(data.tokens_used),
        DisplayFormat::Ratio => format!("{bold}{}/{}{reset}", 
            format_number_compact(data.tokens_used), 
            format_number_compact(data.tokens_limit),
            reset = {RESET},
            bold = {BOLD}),
        _ => format!("{} tokens", format_number(data.tokens_used)),
    }
}

fn generate_progress_with_format(data: &PromptData, display: &DisplayFormat) -> String  {
    match display {        
        // Token Progress Examples  
        DisplayFormat::ProgressBar => {
            let mut filled = (data.progress_percent / 10.0) as usize;
            filled = min(10, filled);
            let empty = 10 - filled;
            let color = match data.progress_percent {
                0.0..=49.0  => GREEN,
                50.0..=79.0  => YELLOW,
                _ => RED
            };
            format!("{color}[{}{}] {bold}{:.1}{reset}%",
                "█".repeat(filled), 
                "░".repeat(empty), 
                data.progress_percent,
                color = {color}, 
                reset = {RESET},
                bold = {BOLD}
            )
        },
        DisplayFormat::StatusColored => {
            if data.progress_percent < 50.0 { "🟢 Good" }
            else if data.progress_percent < 80.0 { "🟡 Near Limit" }
            else { "🔴 Close to Limit" }
        }.to_string(),
        _ =>  format!("{:.1}%", data.progress_percent),
    }
}

fn generate_elapsed_with_format(data: &PromptData, display: &DisplayFormat) -> String  {
    match display {
        DisplayFormat::Duration => format!("{}h {:02}m", data.time_elapsed_hours, data.time_elapsed_minutes),
        DisplayFormat::DurationShort => format!("{}h{:02}m", data.time_elapsed_hours, data.time_elapsed_minutes),
        _ => format!("elapsed {}h {:02}m", data.time_elapsed_hours, data.time_elapsed_minutes),
    }
}

fn generate_remaining_with_format(data: &PromptData, display: &DisplayFormat) -> String  {
    match display {
        DisplayFormat::Duration => format!("{}h {:02}m left", data.time_remaining_hours, data.time_remaining_minutes),
        DisplayFormat::DurationShort => format!("{}h{:02}m", data.time_remaining_hours, data.time_remaining_minutes),
        _ => format!("{}h {:02}m remaining", data.time_remaining_hours, data.time_remaining_minutes),
    }
}

fn generate_message_with_format(data: &PromptData, display: &DisplayFormat) -> String  {
    match display {
        DisplayFormat::TextWithEmoji => format!("💬 {}", data.message_count),
        DisplayFormat::Compact => format!("{}", data.message_count),
        _ => format!("{} messages", data.message_count),
    }
}
        
fn generate_status_with_format(data: &PromptData, display: &DisplayFormat) -> String  {
    match display {
        DisplayFormat::StatusIcon => {
            if data.is_limited { "🚫" } else { "🟢" }
        }.to_string(),
        DisplayFormat::StatusText => data.block_status.clone(),
        _ => {
            if data.is_limited { 
                format!("{red}{bold}{}{reset}", data.block_status, red = RED, bold = BOLD, reset = RESET)
            } else { 
                format!("{green}{bold}{}{reset}", data.block_status, green = GREEN, bold = BOLD, reset = RESET)
            }
        },
    }        
}

fn generate_model_with_format(data: &PromptData, display: &DisplayFormat) -> String  {
    match display {
        _ => format!("🤖 {}", data.model_name)
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