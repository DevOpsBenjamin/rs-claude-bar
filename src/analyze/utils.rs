use chrono::{DateTime, Utc, Duration};
use std::collections::{HashMap, hash_map::Entry};

use crate::{analyze::{DataBlock, LimitBlock}, cache::{CacheInfo, PerHourBlock}};

// STEP 1: Find FIXED 5-hour windows from limit messages
/// Flatten all per-hour usage across all files into a single map keyed by hour start.
pub fn build_per_hour_agg(cache: &CacheInfo) -> (HashMap<DateTime<Utc>, LimitBlock>, HashMap<DateTime<Utc>, PerHourBlock>) {
    let mut limit_blocks: HashMap<DateTime<Utc>, LimitBlock> = HashMap::new();
    let mut per_hour_block: HashMap<DateTime<Utc>, PerHourBlock> = HashMap::new();

    for (_folder, folder) in &cache.folders {
        for (_file, file) in &folder.files {
            for (hour_start, ph) in &file.per_hour {

                // Use HashMap entry API to distinguish new vs existing files
                match per_hour_block.entry(*hour_start) {
                    Entry::Vacant(entry) => {
                        // New block not in current map
                        entry.insert(PerHourBlock
                        {
                            hour_start: *hour_start,
                            hour_end: ph.hour_end,
                            min_timestamp: ph.min_timestamp,
                            max_timestamp: ph.max_timestamp,
                            input_tokens: 0,
                            output_tokens: 0,
                            cache_creation_tokens: 0,
                            cache_read_tokens: 0,
                            assistant_messages: 0,
                            user_messages: 0,
                            total_content_length: 0,
                            entry_count: 0,
                        });
                    },
                    Entry::Occupied(mut entry) => {
                        // Existing block adding infos  compare dates to give good min max
                        let block = entry.get_mut();
                        block.min_timestamp = DateTime::min(block.min_timestamp, ph.min_timestamp);
                        block.max_timestamp = DateTime::max(block.max_timestamp, ph.max_timestamp);
                        block.input_tokens += ph.input_tokens as u32;
                        block.output_tokens += ph.output_tokens as u32;
                        block.cache_creation_tokens += ph.cache_creation_tokens as u32;
                        block.cache_read_tokens += ph.cache_read_tokens as u32;
                        block.assistant_messages += ph.assistant_messages as u32;
                        block.user_messages += ph.user_messages as u32;
                        block.total_content_length += ph.total_content_length as u64;
                        block.entry_count += ph.entry_count as u32;
                    }
                }
            }
            for (_ts, block) in &file.blocks {
                if let Some(unlock) = block.unlock_timestamp {
                    let start = unlock - Duration::hours(5);
                    limit_blocks.entry(start).or_insert(LimitBlock {
                        unlock_timestamp: unlock,
                    });
                }
            }
        }
    }

    (limit_blocks, per_hour_block)
}

/// Build limit windows from cache limits and populate aggregates from pre-aggregated per-hour data.
pub fn analyze_blocks(cache: &CacheInfo) -> HashMap<DateTime<Utc>, DataBlock> {
    let (limit_blocks, per_hour) = build_per_hour_agg(cache);
    let all_blocks = HashMap::new();

    all_blocks
}

/* 
/// Same as build_limit_blocks but reuse precomputed per-hour aggregates.
pub fn build_limit_blocks_with_agg(
    cache: &CacheInfo,
    per_hour: &HashMap<DateTime<Utc>, BlockData>,
) -> HashMap<DateTime<Utc>, LimitBlock> {
    // Collect windows (start = unlock-5h)
    let mut limit_blocks: HashMap<DateTime<Utc>, LimitBlock> = HashMap::new();
    for (_folder, folder) in &cache.folders {
        for (_file, file) in &folder.files {
            for (_ts, block) in &file.blocks {
                if let Some(unlock) = block.unlock_timestamp {
                    let start = unlock - Duration::hours(5);
                    limit_blocks.entry(start).or_insert(LimitBlock {
                        unlock_timestamp: unlock,
                        datas: BlockData::default(),
                    });
                }
            }
        }
    }

    // Populate aggregates from per_hour for each window
    for (start, item) in limit_blocks.iter_mut() {
        let mut agg = BlockData::default();
        let mut h = *start;
        while h < item.unlock_timestamp {
            if let Some(b) = per_hour.get(&h) {
                agg.input_tokens += b.input_tokens;
                agg.output_tokens += b.output_tokens;
                agg.cache_creation_tokens += b.cache_creation_tokens;
                agg.cache_read_tokens += b.cache_read_tokens;
                agg.total_tokens += b.total_tokens;
                agg.assistant_messages += b.assistant_messages;
                agg.user_messages += b.user_messages;
                agg.total_content_length += b.total_content_length;
                agg.entry_count += b.entry_count;
            }
            h = h + Duration::hours(1);
        }
        item.datas = agg;
    }

    limit_blocks
}
*/
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
