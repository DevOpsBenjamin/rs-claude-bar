use std::{collections::HashMap, fs, path::PathBuf};
use chrono::{DateTime, Utc, Timelike};

use crate::{
    cache::{BlockLine, CachedFile, PerHourBlock},
    claude_types::transcript_entry::ClaudeEntry, common::duration::round_to_hour_boundary,
};

/// Refresh a single file by parsing JSONL content and populating cache data
pub fn refresh_single_file(file: &mut CachedFile, file_path: &PathBuf) {
    // Parse entries since the hour boundary (not just cache_time) to get complete hours
    let boundary = round_to_hour_boundary(file.cache_time);
    let new_entries = parse_file_since_boundary(
        file_path.to_string_lossy().as_ref(), 
        boundary
    );
    
    if new_entries.is_empty() {
        // Mark as Fresh even if no new entries (file was checked)
        file.cache_status = crate::cache::CacheStatus::Fresh;
        return;
    }
    
    // Process entries into per-hour blocks and limit events
    let new_per_hour_blocks = generate_per_hour_blocks(&new_entries);
    let new_block_lines = generate_block_lines(&new_entries);
    
    // Merge per-hour blocks (replace existing hours with new data)
    for (hour_start, new_block) in new_per_hour_blocks {
        file.per_hour.insert(hour_start, new_block); // Replace if exists
    }
    
    // Merge/replace block lines by timestamp to avoid duplicates
    for (ts, block) in new_block_lines {
        file.blocks.insert(ts, block);
    }
    
    // Update cache timestamp to current time
    file.cache_time = Utc::now();
    file.cache_status = crate::cache::CacheStatus::Fresh;
}

/// Parse entries strictly newer than `boundary`.
/// - If the file doesn't exist → returns an empty Vec (silent).
/// - Otherwise: reverse-parse and stop when `timestamp <= boundary`,
///   then restore chronological order.
pub fn parse_file_since_boundary(
    file_path: &str,
    boundary: DateTime<Utc>,
) -> Vec<ClaudeEntry> {
    let content = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(_)  => return Vec::new(),
    };

    let mut entries: Vec<_> = content
        .lines()
        .rev()
        .filter_map(|line| parse_line(line))
        .take_while(|entry| 
            {
                let timestamp_str = entry.timestamp();
                let timestamp = DateTime::parse_from_rfc3339(timestamp_str.unwrap_or(""))
                    .unwrap_or_else(|_| DateTime::from(Utc::now()));
                timestamp > boundary
            })
        .collect();

    entries.reverse();
    entries
}

/// Parse single JSONL line into ClaudeBarUsageEntry
pub fn parse_line(line: &str) -> Option<ClaudeEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    serde_json::from_str::<ClaudeEntry>(line).ok()
}

/// Generate per-hour usage blocks from ClaudeEntry list  
fn generate_per_hour_blocks(entries: &[ClaudeEntry]) -> HashMap<DateTime<Utc>, PerHourBlock> {
    let mut hour_blocks: HashMap<DateTime<Utc>, PerHourBlock> = HashMap::new();
    
    for entry in entries {
        if let ClaudeEntry::Transcript(transcript) = entry {
            // Parse timestamp
            let timestamp_dt = match DateTime::parse_from_rfc3339(&transcript.timestamp) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => continue,
            };
            
            // Round down to hour boundary (e.g., 14:32:15 -> 14:00:00)
            let hour_start = round_to_hour_boundary(timestamp_dt);
            let hour_end = hour_start + chrono::Duration::hours(1) - chrono::Duration::seconds(1);
            
            // Get or create hour block
            let hour_block = hour_blocks.entry(hour_start).or_insert_with(|| PerHourBlock {
                hour_start,
                hour_end,
                min_timestamp: timestamp_dt,
                max_timestamp: timestamp_dt,
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                assistant_messages: 0,
                user_messages: 0,
                total_content_length: 0,
                entry_count: 0,
            });
            
            // Update min/max timestamps
            if timestamp_dt < hour_block.min_timestamp {
                hour_block.min_timestamp = timestamp_dt;
            }
            if timestamp_dt > hour_block.max_timestamp {
                hour_block.max_timestamp = timestamp_dt;
            }
            
            // Count message types and content length
            if let Some(role) = &transcript.message.role {
                match role.as_str() {
                    "assistant" => hour_block.assistant_messages += 1,
                    "user" => hour_block.user_messages += 1,
                    _ => {}
                }
            }
            
            // Add content length (from content field)
            match &transcript.message.content {
                crate::claude_types::message::MessageContent::String(text) => {
                    hour_block.total_content_length += text.len() as u64;
                }
                crate::claude_types::message::MessageContent::Blocks(content_items) => {
                    for content_item in content_items {
                        if let Some(text) = content_item.extract_text() {
                            hour_block.total_content_length += text.len() as u64;
                        }
                    }
                }
            }
            
            // Add token usage if available
            if let Some(usage) = &transcript.message.usage {
                hour_block.input_tokens += usage.input_tokens;
                hour_block.output_tokens += usage.output_tokens;
                hour_block.cache_creation_tokens += usage.cache_creation_input_tokens;
                hour_block.cache_read_tokens += usage.cache_read_input_tokens;
            }
            
            hour_block.entry_count += 1;
        }
    }
    
    hour_blocks
}

/// Generate block/limit events from ClaudeEntry list
fn generate_block_lines(entries: &[ClaudeEntry]) -> HashMap<DateTime<Utc>, BlockLine> {
    let mut block_lines: HashMap<DateTime<Utc>, BlockLine> = HashMap::new();
    
    for entry in entries {
        match entry {
            // Look for limit-related messages in Transcript entries
            ClaudeEntry::Transcript(transcript) => {
                // Parse timestamp
                let block_timestamp_utc = match DateTime::parse_from_rfc3339(&transcript.timestamp) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(_) => continue,
                };
                
                // Check if this is an API error message (indicates limit/rate limit failure)
                if transcript.is_api_error_message {
                    // Extract full message text
                    let full_text = match &transcript.message.content {
                        crate::claude_types::message::MessageContent::String(text) => text.clone(),
                        crate::claude_types::message::MessageContent::Blocks(content_items) => {
                            content_items.iter()
                                .filter_map(|item| item.extract_text())
                                .collect::<Vec<_>>()
                                .join(" ")
                        }
                    };
                    
                    let reset_text = extract_reset_time_text(&full_text);
                    let unlock_timestamp = calculate_unlock_time(block_timestamp_utc, &reset_text);
                    
                    block_lines.insert(
                        block_timestamp_utc,
                        BlockLine {
                            unlock_timestamp,
                            reset_text,
                        }
                    );
                }
            },
            // Summary and Unknown entries are kept for debug parsing stats but don't contain limits
            _ => {} // Skip Summary/Unknown entries - they don't contain limit events
        }
    }
    block_lines
}

/// Extract reset time from limit message (e.g., "5-hour limit reached ∙ resets 5pm" -> "5pm")
fn extract_reset_time_text(message: &str) -> String {
    // Look for the "resets " pattern and capture what comes after
    if let Some(pos) = message.find("resets ") {
        let after_resets = &message[pos + 7..]; // Skip "resets "
        
        // Take the first word/time after "resets " (until space or end of string)
        let reset_time = after_resets.split_whitespace().next()
            .unwrap_or("unknown")
            .to_string();
        
        return reset_time;
    }
    
    // Fallback: if no "resets " found, return unknown
    "unknown".to_string()
}

/// Calculate unlock timestamp from block time and reset text
/// e.g., blocked at "08-22 16:43" with reset "5pm" -> unlock at "08-22 17:00"
fn calculate_unlock_time(block_time: DateTime<Utc>, reset_text: &str) -> Option<DateTime<Utc>> {
    if reset_text == "unknown" {
        return None;
    }
    
    // Parse different time formats
    let target_hour = match reset_text.to_lowercase().as_str() {
        "12am" => 0,
        "1am" => 1, "2am" => 2, "3am" => 3, "4am" => 4, "5am" => 5,
        "6am" => 6, "7am" => 7, "8am" => 8, "9am" => 9, "10am" => 10, "11am" => 11,
        "12pm" => 12,
        "1pm" => 13, "2pm" => 14, "3pm" => 15, "4pm" => 16, "5pm" => 17,
        "6pm" => 18, "7pm" => 19, "8pm" => 20, "9pm" => 21, "10pm" => 22, "11pm" => 23,
        _ => return None, // Unknown format
    };
    
    // Start with the same date as the block
    let mut unlock_date = block_time.date_naive();
    
    // If the target hour has already passed today, move to tomorrow
    if target_hour <= block_time.hour() as u8 {
        unlock_date = unlock_date.succ_opt()?;
    }
    
    // Create the unlock timestamp
    let unlock_time = unlock_date.and_hms_opt(target_hour as u32, 0, 0)?;
    Some(DateTime::from_naive_utc_and_offset(unlock_time, Utc))
}
