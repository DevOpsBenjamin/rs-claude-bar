use std::path::Path;
use chrono::{Utc, Duration};

use crate::{
    analyze::{
        detect_block_status,
        BlockStatus,
        load_entries_since
    },
    common::input::parse_claude_input,
    config_manager::{
        load_stats, 
        save_stats
    },
    status::generate_status_with_config_and_model,
    claudebar_types::{
        config::{
            StatsFile,
            ConfigInfo,
            SimpleBlock
        },
        usage_entry::{ 
            ClaudeBarUsageEntry, 
            UserRole
        }
    }
};

pub fn run(config: &ConfigInfo) {
    // Check if stats need refreshing (older than 5 seconds)  
    let stats = load_stats();
    let now = chrono::Utc::now();
    let should_refresh = stats.last_processed
        .map(|last| now.signed_duration_since(last).num_seconds() > 5)
        .unwrap_or(true);
    
    // TODO: Auto-refresh disabled due to double-counting bug
    // Need to implement proper incremental updates
    if false && should_refresh {
        // Run the resets logic silently (same as resets command but no output)
        let _ = super::resets::refresh_stats_for_status(config);
    }
    
    // Try to get Claude Code input for model info
    let model_name = parse_claude_input()
        .map(|input| input.model.display_name);
    
    match generate_status_with_config_and_model(config, model_name) {
        Ok(status) => print!("{}", status),
        Err(err) => eprintln!("Error generating status: {}", err),
    }
}

/// Refresh stats silently (no output) if needed
fn refresh_stats_silently(config: &ConfigInfo) -> Result<(), Box<dyn std::error::Error>> {   
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);
    if !path.exists() {
        return Ok(());
    }
    
    let mut stats = load_stats();
    let now = Utc::now();
    
    // Load new entries since last processed
    let since_time = stats.last_processed.unwrap_or_else(|| now - Duration::days(7));
    let all_entries = load_entries_since(&base_path, Some(since_time));
    
    if !all_entries.is_empty() {
        // Update stats with new data
        update_stats_with_blocks(&mut stats, now);
        
        // Update token counts
        update_block_tokens(&mut stats, &all_entries);
        
        save_stats(&stats)?;
    }
    
    Ok(())
}

// Helper functions (simplified from resets command)
fn update_stats_with_blocks(
    stats: &mut StatsFile,
    now: chrono::DateTime<chrono::Utc>
) {   
    let block_status = detect_block_status(now, &stats.current);
    
    // Simple logic: if no current block, create one starting now
    if matches!(block_status, BlockStatus::NoCurrentBlock) {
        let current_start = round_to_hour_boundary(now);
        let current_end = current_start + chrono::Duration::hours(5);
        stats.current = Some(SimpleBlock {
            start: current_start,
            end: current_end,
            tokens: 0,
        });
    }
    
    stats.last_processed = Some(now);
}

fn update_block_tokens(stats: &mut StatsFile, entries: &[ClaudeBarUsageEntry]) {
    use UserRole;
    
    // Use same additive logic as resets command
    for entry in entries {
        if !matches!(entry.role, UserRole::Assistant) {
            continue;
        }
        
        let tokens = entry.usage.output_tokens as i64;
        let timestamp = entry.timestamp;
        
        // Check if entry belongs to current block
        if let Some(current) = &mut stats.current {
            if timestamp >= current.start && timestamp <= current.end {
                current.tokens += tokens;
                continue;
            }
        }
        
        // Check if entry belongs to any past block
        for past_block in &mut stats.past {
            if timestamp >= past_block.start && timestamp <= past_block.end {
                past_block.tokens += tokens;
                break;
            }
        }
    }
}

fn round_to_hour_boundary(dt: chrono::DateTime<chrono::Utc>) -> chrono::DateTime<chrono::Utc> {
    use chrono::Timelike;
    dt.date_naive()
        .and_hms_opt(dt.hour(), 0, 0)
        .unwrap()
        .and_utc()
}

