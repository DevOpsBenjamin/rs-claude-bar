use chrono::{DateTime, Utc, Duration};
use std::collections::{HashMap, hash_map::Entry};

use crate::{
    analyze::{DataBlock, DataStats, BlockKind, LimitBlock},
    cache::{CacheInfo, PerHourBlock}
};

// Internal state for building Gap blocks; kept at module scope (no inner structs)
struct GapState {
    start: DateTime<Utc>,
    last: DateTime<Utc>,
    hours_count: i32,
    min: DateTime<Utc>,
    max: DateTime<Utc>,
    stats: DataStats,
}

// STEP 1: Find FIXED 5-hour windows from limit messages
/// Flatten all per-hour usage across all files into a single map keyed by hour start.
fn build_per_hour_agg(cache: &CacheInfo) -> (HashMap<DateTime<Utc>, LimitBlock>, HashMap<DateTime<Utc>, PerHourBlock>) {
    let mut limit_blocks: HashMap<DateTime<Utc>, LimitBlock> = HashMap::new();
    let mut per_hour_block: HashMap<DateTime<Utc>, PerHourBlock> = HashMap::new();

    for (_folder, folder) in &cache.folders {
        for (_file, file) in &folder.files {
            for (hour_start, ph) in &file.per_hour {

                // Use HashMap entry API to distinguish new vs existing files
                match per_hour_block.entry(*hour_start) {
                    Entry::Vacant(entry) => {
                        // First time we see this hour: start with the actual values
                        entry.insert(ph.clone());
                    },
                    Entry::Occupied(mut entry) => {
                        // Existing block adding infos  compare dates to give good min max
                        let block = entry.get_mut();
                        if ph.min_timestamp < block.min_timestamp {
                            block.min_timestamp = ph.min_timestamp;
                        }
                        if ph.max_timestamp > block.max_timestamp {
                            block.max_timestamp = ph.max_timestamp;
                        }
                        block.input_tokens += ph.input_tokens;
                        block.output_tokens += ph.output_tokens;
                        block.cache_creation_tokens += ph.cache_creation_tokens;
                        block.cache_read_tokens += ph.cache_read_tokens;
                        block.assistant_messages += ph.assistant_messages;
                        block.user_messages += ph.user_messages;
                        block.total_content_length += ph.total_content_length;
                        block.entry_count += ph.entry_count;
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

    // Prepare sorted lists
    let mut limit_windows: Vec<(DateTime<Utc>, DateTime<Utc>)> = limit_blocks
        .iter()
        .map(|(s, lb)| (*s, lb.unlock_timestamp))
        .collect();
    limit_windows.sort_by_key(|(s, _)| *s);

    let mut hour_keys: Vec<DateTime<Utc>> = per_hour.keys().cloned().collect();
    hour_keys.sort();

    // 1) Create Limit blocks directly from windows
    let mut result: HashMap<DateTime<Utc>, DataBlock> = HashMap::new();
    for (start, end) in &limit_windows {
        let mut stats = DataStats::default();
        let mut seen = false;
        let mut min_timestamp = *start;
        let mut max_timestamp = *start;
        let mut t = *start;
        while t < *end {
            if let Some(ph) = per_hour.get(&t) {
                stats.input_tokens += ph.input_tokens as i64;
                stats.output_tokens += ph.output_tokens as i64;
                stats.cache_creation_tokens += ph.cache_creation_tokens as i64;
                stats.cache_read_tokens += ph.cache_read_tokens as i64;
                stats.total_tokens += (ph.input_tokens as i64
                    + ph.output_tokens as i64
                    + ph.cache_creation_tokens as i64
                    + ph.cache_read_tokens as i64);
                stats.assistant_messages += ph.assistant_messages as i64;
                stats.user_messages += ph.user_messages as i64;
                stats.total_content_length += ph.total_content_length as i64;
                stats.entry_count += ph.entry_count as i64;
                if !seen {
                    min_timestamp = ph.min_timestamp;
                    max_timestamp = ph.max_timestamp;
                    seen = true;
                } else {
                    if ph.min_timestamp < min_timestamp { min_timestamp = ph.min_timestamp; }
                    if ph.max_timestamp > max_timestamp { max_timestamp = ph.max_timestamp; }
                }
            }
            t = t + Duration::hours(1);
        }
        result.insert(*start, DataBlock {
            kind: BlockKind::Limit,
            start: *start,
            end: *end,
            unlock_timestamp: Some(*end),
            min_timestamp,
            max_timestamp,
            stats,
        });
    }

    // 2) Build Gap blocks from consecutive per-hour hours outside any limit window
    let mut win_idx = 0usize;
    let mut gap: Option<GapState> = None;

    for &h in &hour_keys {
        // advance window index past windows that ended before this hour
        while win_idx < limit_windows.len() && limit_windows[win_idx].1 <= h {
            win_idx += 1;
        }
        let inside_limit = win_idx < limit_windows.len()
            && h >= limit_windows[win_idx].0
            && h < limit_windows[win_idx].1;

        if inside_limit {
            // close any ongoing gap
            if let Some(gs) = gap.take() {
                let end = gs.last + Duration::hours(1);
                result.insert(gs.start, DataBlock {
                    kind: BlockKind::Gap,
                    start: gs.start,
                    end,
                    unlock_timestamp: None,
                    min_timestamp: gs.min,
                    max_timestamp: gs.max,
                    stats: gs.stats,
                });
            }
            continue;
        }

        // free hour (activity outside limits): extend or start gap
        let ph = &per_hour[&h];
        if let Some(mut gs) = gap.take() {
            // If consecutive hour, extend; otherwise flush previous gap and start new one
            if h == gs.last + Duration::hours(1) {
                // If adding this hour exceeds 5h window, flush current gap and start new one at h
                if gs.hours_count >= 5 {
                    let end = gs.last + Duration::hours(1);
                    result.insert(gs.start, DataBlock {
                        kind: BlockKind::Gap,
                        start: gs.start,
                        end,
                        unlock_timestamp: None,
                        min_timestamp: gs.min,
                        max_timestamp: gs.max,
                        stats: gs.stats,
                    });
                    // Start a fresh gap at current hour h
                    let mut stats = DataStats::default();
                    stats.input_tokens = ph.input_tokens as i64;
                    stats.output_tokens = ph.output_tokens as i64;
                    stats.cache_creation_tokens = ph.cache_creation_tokens as i64;
                    stats.cache_read_tokens = ph.cache_read_tokens as i64;
                    stats.total_tokens = (ph.input_tokens as i64
                        + ph.output_tokens as i64
                        + ph.cache_creation_tokens as i64
                        + ph.cache_read_tokens as i64);
                    stats.assistant_messages = ph.assistant_messages as i64;
                    stats.user_messages = ph.user_messages as i64;
                    stats.total_content_length = ph.total_content_length as i64;
                    stats.entry_count = ph.entry_count as i64;
                    gap = Some(GapState { start: h, last: h, hours_count: 1, min: ph.min_timestamp, max: ph.max_timestamp, stats });
                } else {
                    // Extend existing gap
                    gs.last = h;
                    gs.hours_count += 1;
                    if ph.min_timestamp < gs.min { gs.min = ph.min_timestamp; }
                    if ph.max_timestamp > gs.max { gs.max = ph.max_timestamp; }
                    gs.stats.input_tokens += ph.input_tokens as i64;
                    gs.stats.output_tokens += ph.output_tokens as i64;
                    gs.stats.cache_creation_tokens += ph.cache_creation_tokens as i64;
                    gs.stats.cache_read_tokens += ph.cache_read_tokens as i64;
                    gs.stats.total_tokens += (ph.input_tokens as i64
                        + ph.output_tokens as i64
                        + ph.cache_creation_tokens as i64
                        + ph.cache_read_tokens as i64);
                    gs.stats.assistant_messages += ph.assistant_messages as i64;
                    gs.stats.user_messages += ph.user_messages as i64;
                    gs.stats.total_content_length += ph.total_content_length as i64;
                    gs.stats.entry_count += ph.entry_count as i64;
                    gap = Some(gs);
                }
            } else {
                // flush previous
                let end = gs.last + Duration::hours(1);
                result.insert(gs.start, DataBlock {
                    kind: BlockKind::Gap,
                    start: gs.start,
                    end,
                    unlock_timestamp: None,
                    min_timestamp: gs.min,
                    max_timestamp: gs.max,
                    stats: gs.stats,
                });
                // start new
                let mut stats = DataStats::default();
                stats.input_tokens = ph.input_tokens as i64;
                stats.output_tokens = ph.output_tokens as i64;
                stats.cache_creation_tokens = ph.cache_creation_tokens as i64;
                stats.cache_read_tokens = ph.cache_read_tokens as i64;
                stats.total_tokens = (ph.input_tokens as i64
                    + ph.output_tokens as i64
                    + ph.cache_creation_tokens as i64
                    + ph.cache_read_tokens as i64);
                stats.assistant_messages = ph.assistant_messages as i64;
                stats.user_messages = ph.user_messages as i64;
                stats.total_content_length = ph.total_content_length as i64;
                stats.entry_count = ph.entry_count as i64;
                gap = Some(GapState { start: h, last: h, hours_count: 1, min: ph.min_timestamp, max: ph.max_timestamp, stats });
            }
        } else {
            // start new gap from this hour
            let mut stats = DataStats::default();
            stats.input_tokens = ph.input_tokens as i64;
            stats.output_tokens = ph.output_tokens as i64;
            stats.cache_creation_tokens = ph.cache_creation_tokens as i64;
            stats.cache_read_tokens = ph.cache_read_tokens as i64;
            stats.total_tokens = (ph.input_tokens as i64
                + ph.output_tokens as i64
                + ph.cache_creation_tokens as i64
                + ph.cache_read_tokens as i64);
            stats.assistant_messages = ph.assistant_messages as i64;
            stats.user_messages = ph.user_messages as i64;
            stats.total_content_length = ph.total_content_length as i64;
            stats.entry_count = ph.entry_count as i64;
            gap = Some(GapState { start: h, last: h, hours_count: 1, min: ph.min_timestamp, max: ph.max_timestamp, stats });
        }
    }

    // Flush trailing gap
    if let Some(gs) = gap.take() {
        let end = gs.last + Duration::hours(1);
        result.insert(gs.start, DataBlock {
            kind: BlockKind::Gap,
            start: gs.start,
            end,
            unlock_timestamp: None,
            min_timestamp: gs.min,
            max_timestamp: gs.max,
            stats: gs.stats,
        });
    }

    result
}
