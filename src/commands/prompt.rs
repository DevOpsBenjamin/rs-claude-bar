use crate::{
    analyze::{Analyzer, BlockKind},
    common::input::parse_claude_input,
    claudebar_types::config::ConfigInfo,
    common::colors::*,
};
use chrono::{DateTime, Utc};

pub fn run(_config: &ConfigInfo, analyzer: &Analyzer) {
    // Try to get Claude Code input for model info
    let model_name = parse_claude_input()
        .map(|input| input.model.display_name)
        .unwrap_or_else(|| "Claude".to_string());
    
    match generate_status(analyzer, &model_name) {
        Ok(status) => print!("{}", status),
        Err(err) => eprintln!("Error generating status: {}", err),
    }
}

fn generate_status(analyzer: &Analyzer, model_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let now = Utc::now();
    
    // Get current active block (either limit or gap)
    let current_block = analyzer.find_current_block(now);
    
    match current_block {
        Some(block) => {
            let progress = calculate_progress(&block, now);
            let time_info = format_time_info(&block, now);
            let token_info = format_token_info(&block);
            
            match block.kind {
                BlockKind::Limit => {
                    // In a limit block - show progress toward limit
                    Ok(format!(
                        "🚫 {token_info} {progress} | {time_info} | 🤖 {model_name}",
                        token_info = token_info,
                        progress = progress,
                        time_info = time_info,
                        model_name = model_name
                    ))
                },
                BlockKind::Gap => {
                    // In a gap block - show normal usage
                    Ok(format!(
                        "🧠 {token_info} {progress} | {time_info} | 🤖 {model_name}",
                        token_info = token_info,
                        progress = progress,  
                        time_info = time_info,
                        model_name = model_name
                    ))
                },
                BlockKind::Current => {
                    // Current/active block - show as active session
                    Ok(format!(
                        "⚡ {token_info} {progress} | {time_info} | 🤖 {model_name}",
                        token_info = token_info,
                        progress = progress,
                        time_info = time_info,
                        model_name = model_name
                    ))
                }
            }
        },
        None => {
            // No current block - probably no recent activity
            Ok(format!("💤 No recent activity | 🤖 {}", model_name))
        }
    }
}

fn calculate_progress(block: &crate::analyze::DataBlock, now: DateTime<Utc>) -> String {
    let total_duration = block.end.signed_duration_since(block.start);
    let elapsed_duration = now.signed_duration_since(block.start);
    
    if total_duration.num_seconds() == 0 {
        return "0%".to_string();
    }
    
    let progress_pct = (elapsed_duration.num_seconds() as f64 / total_duration.num_seconds() as f64 * 100.0).min(100.0).max(0.0);
    
    let color = match progress_pct {
        p if p < 50.0 => GREEN,
        p if p < 80.0 => YELLOW, 
        _ => RED,
    };
    
    format!("{}({:.1}%){}", color, progress_pct, RESET)
}

fn format_time_info(block: &crate::analyze::DataBlock, now: DateTime<Utc>) -> String {
    let elapsed = now.signed_duration_since(block.start);
    let remaining = block.end.signed_duration_since(now);
    
    let elapsed_str = format_duration(elapsed);
    let remaining_str = if remaining.num_seconds() > 0 {
        format!(" | ⏰ {} left", format_duration(remaining))
    } else {
        " | ⏰ Expired".to_string()
    };
    
    format!("⏱️ {}{}", elapsed_str, remaining_str)
}

fn format_token_info(block: &crate::analyze::DataBlock) -> String {
    let tokens = block.stats.total_tokens;
    let messages = block.stats.assistant_messages + block.stats.user_messages;
    
    format!("{} tokens | 💬 {}", format_number(tokens), messages)
}

fn format_duration(duration: chrono::Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() - hours * 60;
    
    if hours > 0 {
        format!("{}h{:02}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

fn format_number(num: i64) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        format!("{}", num)
    }
}