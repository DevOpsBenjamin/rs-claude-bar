use chrono::{Utc, Duration, DateTime, Timelike};
use std::path::Path;

use crate::{
    claudebar_types::{
        config::{
            ConfigInfo,          
            StatsFile,
            SimpleBlock,
        },
        usage_entry::{ 
            UserRole,
            ClaudeBarUsageEntry,
        },
        blocks::{
            GuessBlock,
        },
    },
    analyze::{
        parse_reset_time,
        load_entries_since,
        detect_block_status,
        calculate_unlock_time,
        build_guess_blocks_from_entries,
        BlockStatus,
    },
    config_manager::{load_stats, save_stats},
};

/// Round a timestamp to the current hour boundary (00:00 minutes/seconds)
fn round_to_hour_boundary(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.date_naive()
        .and_hms_opt(dt.hour(), 0, 0)
        .unwrap()
        .and_utc()
}

// Read JSONL files, find only limit-reached entries, and print simple list lines:
// "<end UTC> | <start UTC>" where start = end - 5h

/// Silent version of run() for status command auto-refresh
pub fn refresh_stats_for_status(config: &ConfigInfo) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);
    if !path.exists() {
        return;
    }
    
    let mut stats = load_stats();
    let now = Utc::now();
    
    // Load entries since last processed time  
    let since_time = stats.last_processed.unwrap_or_else(|| now - Duration::days(7));
    let all_entries = load_entries_since(&base_path, Some(since_time));
    
    if all_entries.is_empty() {
        return;
    }
    
    // Reset current block tokens to avoid double-counting, then rebuild from ALL entries in this block
    if let Some(current) = &mut stats.current {
        current.tokens = 0;
    }
    for past in &mut stats.past {
        past.tokens = 0;
    }
    
    // Process ALL entries for the current day to get accurate count  
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let all_today_entries = load_entries_since(&base_path, Some(today_start));
    
    // Update stats with blocks and token counts
    update_block_tokens(&mut stats, &all_today_entries);
    stats.last_processed = Some(now);
    
    // Save silently
    let _ = save_stats(&stats);
}

pub fn run(config: &ConfigInfo) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);
    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    // Load existing stats
    let mut stats = load_stats();
    let now = Utc::now();
    
    // Detect current block status
    let block_status = detect_block_status(now, &stats.current);
    
    // Load entries since last processing
    let since = stats.last_processed;
    let mut all_entries = load_entries_since(&base_path, since);
    
    if all_entries.is_empty() && since.is_some() {
        println!("✅ No new entries since last run.");
        // Still need to show existing stats
        display_simple_blocks(&stats);
        return;
    }
    
    // Process new entries if we have them
    if !all_entries.is_empty() {
        all_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        stats.last_processed = Some(all_entries.first().unwrap().timestamp);
    }
    
    let limit_entries: Vec<ClaudeBarUsageEntry> = all_entries
        .iter()
        .cloned()
        .filter(|e| e.is_limit_reached)
        .collect();
    
    if all_entries.is_empty() {
        println!("❌ No usage entries found!");
        return;
    }

    // Process new limit entries and update stats
    if !limit_entries.is_empty() {
        // Build GuessBlocks from new limit entries
        let new_guess_blocks: Vec<GuessBlock> = build_guess_blocks_from_entries(&limit_entries);
        
        // Convert new completed blocks to SimpleBlocks and add to past
        for guess in &new_guess_blocks {
            if guess.reset != "projected" {
                let simple_block = SimpleBlock {
                    start: guess.start,
                    end: guess.end,
                    tokens: 0, // Will be filled in by aggregation
                };
                
                // Check if this block already exists in past
                if !stats.past.iter().any(|b| b.start == simple_block.start && b.end == simple_block.end) {
                    stats.past.push(simple_block);
                }
            }
        }
        
        // Sort past blocks by start time (ascending)
        stats.past.sort_by_key(|b| b.start);
    }
    
    // Update current block based on block status and now time
    match block_status {
        BlockStatus::NoCurrentBlock => {
            // Create new current block starting from the current hour boundary
            let current_start = round_to_hour_boundary(now);
            let current_end = current_start + Duration::hours(5);
            stats.current = Some(SimpleBlock {
                start: current_start,
                end: current_end,
                tokens: 0,
            });
        }
        BlockStatus::NeedNewBlock => {
            // Move current block to past (if it has tokens) and create projected block
            if let Some(current) = stats.current.take() {
                if current.tokens > 0 {
                    // Add completed block to past
                    if !stats.past.iter().any(|b| b.start == current.start && b.end == current.end) {
                        stats.past.push(current);
                        stats.past.sort_by_key(|b| b.start);
                    }
                }
            }
            
            // Try to find reset time from recent limit entries
            if let Some(reset_time) = find_reset_time_from_entries(&limit_entries) {
                // Calculate next block start from reset time
                if let Some(next_start) = calculate_unlock_time(now, &reset_time) {
                    let next_end = next_start + Duration::hours(5);
                    stats.current = Some(SimpleBlock {
                        start: next_start,
                        end: next_end,
                        tokens: 0,
                    });
                } else {
                    // Fallback: create block starting in 5 hours
                    let next_start = now + Duration::hours(5);
                    let next_end = next_start + Duration::hours(5);
                    stats.current = Some(SimpleBlock {
                        start: next_start,
                        end: next_end,
                        tokens: 0,
                    });
                }
            } else {
                // No reset time found, create block starting in 5 hours
                let next_start = now + Duration::hours(5);
                let next_end = next_start + Duration::hours(5);
                stats.current = Some(SimpleBlock {
                    start: next_start,
                    end: next_end,
                    tokens: 0,
                });
            }
        }
        BlockStatus::InCurrentBlock => {
            // Keep existing current block
        }
        BlockStatus::BeforeCurrentBlock => {
            // We're before the scheduled reset time - show limit status
        }
    }
    
    // Aggregate tokens for all blocks (including current)
    if !all_entries.is_empty() {
        update_block_tokens(&mut stats, &all_entries);
    }
    
    // Save updated stats
    if let Err(e) = save_stats(&stats) {
        eprintln!("Warning: Could not save stats: {}", e);
    }
    
    // Display the results
    display_simple_blocks(&stats);
}

/// Display the simple blocks stats
fn display_simple_blocks(stats: &StatsFile) {
    let now = Utc::now();
    let block_status = detect_block_status(now, &stats.current);
    
    println!("📊 Simple Blocks Stats");
    println!();
    
    // Show status based on current situation
    match block_status {
        BlockStatus::NeedNewBlock => {
            if let Some(current) = &stats.current {
                println!("🔴 LIMIT REACHED - Resets at {}", current.start.format("%Y-%m-%d %H:%M UTC"));
                println!();
            }
        }
        BlockStatus::BeforeCurrentBlock => {
            if let Some(current) = &stats.current {
                println!("🔴 LIMIT - Resets at {}", current.start.format("%Y-%m-%d %H:%M UTC"));
                println!();
            }
        }
        BlockStatus::InCurrentBlock => {
            if let Some(current) = &stats.current {
                let remaining = current.end.signed_duration_since(now);
                let remaining_str = if remaining.num_hours() > 0 {
                    format!("{}h {}m", remaining.num_hours(), remaining.num_minutes() % 60)
                } else {
                    format!("{}m", remaining.num_minutes())
                };
                println!("🟢 ACTIVE - {} remaining | {} tokens", remaining_str, current.tokens);
                println!();
            }
        }
        BlockStatus::NoCurrentBlock => {
            println!("🟡 NO ACTIVE BLOCK");
            println!();
        }
    }
    
    if stats.past.is_empty() && stats.current.is_none() {
        println!("No blocks found.");
        return;
    }
    
    // Show past blocks (last 10, ascending order)
    let mut display_past = stats.past.clone();
    if display_past.len() > 10 {
        let start = display_past.len() - 10;
        display_past = display_past[start..].to_vec();
    }
    
    if !display_past.is_empty() {
        println!("Past Blocks:");
        for (i, block) in display_past.iter().enumerate() {
            println!("  Block {}: {} - {} | {} tokens", 
                     i + 1,
                     block.start.format("%m-%d %H:%M"),
                     block.end.format("%m-%d %H:%M"),
                     block.tokens);
        }
    }
    
    // Show current block details
    if let Some(current) = &stats.current {
        println!();
        println!("Current Block:");
        println!("  {} - {} | {} tokens", 
                 current.start.format("%Y-%m-%d %H:%M UTC"),
                 current.end.format("%Y-%m-%d %H:%M UTC"),
                 current.tokens);
    }
    
    if let Some(last_processed) = stats.last_processed {
        println!();
        println!("Last processed: {}", last_processed.format("%Y-%m-%d %H:%M UTC"));
    }
}

/// Find reset time from limit entries
fn find_reset_time_from_entries(entries: &[ClaudeBarUsageEntry]) -> Option<String> {
    // Find the most recent limit entry with reset time
    for entry in entries.iter() {
        if let Some(content) = &entry.content_text {
            if let Some(reset_time) = parse_reset_time(content) {
                return Some(reset_time);
            }
        }
    }
    None
}

/// Update token counts for blocks based on entries
fn update_block_tokens(stats: &mut StatsFile, entries: &[ClaudeBarUsageEntry]) {
    
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
