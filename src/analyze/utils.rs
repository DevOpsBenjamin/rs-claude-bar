use chrono::{DateTime, Utc, Duration};
use std::collections::{HashMap, HashSet};

use crate::{analyze::{BlockData, LimitBlock}, cache::CacheInfo};

// STEP 1: Find FIXED 5-hour windows from limit messages
pub fn build_limit_blocks(cache: &CacheInfo) -> HashMap<DateTime<Utc>, LimitBlock> {    
    let mut limit_blocks: HashMap<DateTime<Utc>, LimitBlock> = HashMap::new();

    for (_folder, folder) in &cache.folders {
        for (_file, file) in &folder.files {
            for (_ts, block) in &file.blocks {
                if let Some(unlock) = block.unlock_timestamp {
                    let start = unlock - Duration::hours(5);
                    limit_blocks.insert(start, 
                        LimitBlock 
                        {
                            unlock_timestamp: unlock,
                            datas: BlockData::default()
                        });
                }
            }
        }
    }
    limit_blocks
}

/*
fn analyze_usage_blocks(entries: &[ClaudeBarUsageEntry]) -> Vec<UsageBlock> {
    
    // Consider only assistant messages
    let mut assistant_entries: Vec<ClaudeBarUsageEntry> = entries
        .iter()
        .filter(|e| matches!(e.role, UserRole::Assistant))
        .cloned()
        .collect();

    if assistant_entries.is_empty() {
        return Vec::new();
    }

    assistant_entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut blocks = Vec::new();

    let mut fixed_windows = Vec::new();
    let mut seen_windows = std::collections::HashSet::new();
    
    for entry in &assistant_entries {
        if entry.is_limit_reached {
            let content_text = get_entry_content_text(entry);
            if let Some(reset_time) = parse_reset_time(&content_text) {
                // Calculate FIXED window: reset time defines the END of 5-hour window
                let window = calculate_fixed_window_from_reset(entry.timestamp, &reset_time);
                let window_key = (window.0, window.1); // (start, end) as key
                
                // Only add if we haven't seen this exact window before
                if seen_windows.insert(window_key) {
                    fixed_windows.push((window, entry.clone()));
                }
            }
        }
    }

    // STEP 2: Create blocks from FIXED windows
    for (fixed_window, limit_entry) in fixed_windows {
        let (window_start, window_end) = fixed_window;
        
        // Find all entries within this FIXED window
        let window_entries: Vec<ClaudeBarUsageEntry> = assistant_entries
            .iter()
            .filter(|e| e.timestamp >= window_start && e.timestamp <= window_end)
            .cloned()
            .collect();

        let assistant_count = window_entries.iter()
            .filter(|e| !e.is_limit_reached)
            .count();

        let content_text = get_entry_content_text(&limit_entry);
        let reset_time = parse_reset_time(&content_text);
        let unlock_time = reset_time.as_ref()
            .and_then(|rt| calculate_unlock_time(limit_entry.timestamp, rt));

        // Note: Could track actual usage bounds within the fixed window if needed

        blocks.push(UsageBlock {
            start_time: window_start,  // FIXED window start
            end_time: Some(window_end), // FIXED window end
            entries: window_entries,
            assistant_count,
            limit_reached: true,
            reset_time,
            unlock_time,
            guessed: false,
            total_tokens: 0, // Could calculate actual tokens here
        });
    }

    // STEP 3: Fill gaps with estimated usage sessions (>1 hour gaps)
    let mut remaining_entries = assistant_entries.clone();
    
    // Remove entries already assigned to fixed windows
    for block in &blocks {
        remaining_entries.retain(|e| !block.entries.iter().any(|be| be.timestamp == e.timestamp));
    }

    // Group remaining entries into sessions (gaps > 1 hour = new session)
    if !remaining_entries.is_empty() {
        let sessions = group_entries_into_sessions(&remaining_entries);
        
        for session in sessions {
            if session.is_empty() { continue; }
            
            let session_start = session.iter().map(|e| e.timestamp).min().unwrap();
            let session_end = session.iter().map(|e| e.timestamp).max().unwrap();
            let assistant_count = session.iter().filter(|e| !e.is_limit_reached).count();
            
            // Determine if this is an ongoing session (no end time if within last hour)
            let now = Utc::now();
            let is_ongoing = (now - session_end) < Duration::hours(1);
            
            blocks.push(UsageBlock {
                start_time: session_start,
                end_time: if is_ongoing { None } else { Some(session_end) },
                entries: session.clone(),
                assistant_count,
                limit_reached: false,
                reset_time: None,
                unlock_time: None,
                guessed: true, // These are estimated sessions
                total_tokens: session.iter().map(|e| e.usage.output_tokens).sum(),
            });
        }
    }

    // Sort blocks chronologically
    blocks.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    blocks
} 
*/