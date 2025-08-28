use chrono::{DateTime, Utc};
use chrono::TimeZone;
use regex::Regex;
use std::{fs, path::Path, collections::HashMap};

use crate::{
    claude_types::TranscriptEntry,
    claudebar_types::{AssistantInfo, CurrentBlock, GuessBlock, UserInfo, ClaudeBarUsageEntry},
};

/// Public: load every entry from `~/.claude/projects`-style path
pub fn load_all_entries(base_path: &str) -> Vec<ClaudeBarUsageEntry> {
    let mut usage_entries = Vec::new();
    let projects = Path::new(base_path);
    if !projects.exists() {
        return usage_entries;
    }

    if let Ok(entries) = fs::read_dir(projects) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name().to_string_lossy().to_string();
                            let file_date = file
                                .metadata()
                                .ok()
                                .and_then(|m| m.modified().ok())
                                .map(DateTime::<Utc>::from);
                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for line in content.lines() {
                                    let line = line.trim();
                                    if line.is_empty() {
                                        continue;
                                    }
                                    if let Ok(transcript) = serde_json::from_str::<TranscriptEntry>(line) {
                                        let entry = ClaudeBarUsageEntry::from_transcript(
                                            &transcript,
                                            folder_name.clone(),
                                            file_name.clone(),
                                            file_date,
                                        );
                                        usage_entries.push(entry);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    usage_entries
}

/// Public: parse reset time (e.g. "10pm")
pub fn parse_reset_time(content: &str) -> Option<String> {
    let patterns = [
        r"(?i)reset\s*time:\s*(\d{1,2}(?::\d{2})?\s*(?:am|pm))",
        r"(?i)resets?\s+(?:at\s+)?(\d{1,2}(?::\d{2})?\s*(?:am|pm))",
        r"(?i)(?:until|at)\s+(\d{1,2}(?::\d{2})?\s*(?:am|pm))",
    ];
    for pat in patterns {
        if let Ok(re) = Regex::new(pat) {
            if let Some(caps) = re.captures(content) {
                return Some(caps[1].to_lowercase());
            }
        }
    }
    None
}

/// Public: compute unlock time from a message timestamp and reset time string
pub fn calculate_unlock_time(limit_timestamp: DateTime<Utc>, reset_time: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"(\d{1,2})(?::(\d{2}))?\s*(am|pm)").ok()?;
    let caps = re.captures(reset_time)?;

    let hour: u32 = caps.get(1)?.as_str().parse().ok()?;
    let minute: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let is_pm = caps.get(3)?.as_str().eq_ignore_ascii_case("pm");

    let hour_24 = match (hour, is_pm) {
        (12, false) => 0,
        (12, true) => 12,
        (h, false) => h,
        (h, true) => h + 12,
    };
    if hour_24 >= 24 || minute >= 60 {
        return None;
    }

    let limit_date = limit_timestamp.date_naive();
    let same_day = limit_date
        .and_hms_opt(hour_24, minute, 0)?
        .and_local_timezone(Utc)
        .single()?;

    let unlock = if same_day > limit_timestamp {
        same_day
    } else {
        (limit_date + chrono::Days::new(1))
            .and_hms_opt(hour_24, minute, 0)?
            .and_local_timezone(Utc)
            .single()?
    };
    Some(unlock)
}

/// Build GuessBlocks from limit-reached entries (dedup by (start,end), sort desc by end)
pub fn build_guess_blocks_from_entries(entries: &[ClaudeBarUsageEntry]) -> Vec<GuessBlock> {
    let mut blocks: Vec<GuessBlock> = Vec::new();

    for e in entries {
        if !e.is_limit_reached {
            continue;
        }
        let content = e.content_text.as_deref().unwrap_or("");
        if let Some(reset_time) = parse_reset_time(content) {
            if let Some(unlock) = calculate_unlock_time(e.timestamp, &reset_time) {
                let start = unlock - chrono::Duration::hours(5);
                blocks.push(GuessBlock {
                    msg_timestamp: e.timestamp,
                    reset: reset_time,
                    end: unlock,
                    start,
                });
            }
        }
    }

    // Deduplicate by (start,end)
    use std::collections::HashSet;
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    blocks.retain(|b| seen.insert((b.start.timestamp(), b.end.timestamp())));

    // Sort newest first by end time
    blocks.sort_by(|a, b| b.end.cmp(&a.end));

    // Add projected block (latest end -> +5h) as a real block at front
    if let Some(latest) = blocks.first().cloned() {
        let start = latest.end;
        let end = start + chrono::Duration::hours(5);
        if !blocks.iter().any(|b| b.start == start && b.end == end) {
            let projected = GuessBlock {
                msg_timestamp: start,
                reset: "projected".to_string(),
                start,
                end,
            };
            blocks.insert(0, projected);
        }
    }

    blocks
}

/// Build CurrentBlocks timeline from GuessBlocks, with gap rules:
/// - First gap: now -> first guess end (no explicit bounds)
/// - No middle gap when current.start == next.end
/// - Last gap: last guess start -> forever
pub fn build_current_blocks_from_guess(guess: &Vec<GuessBlock>, now: DateTime<Utc>) -> Vec<CurrentBlock> {
    let mut blocks: Vec<CurrentBlock> = Vec::new();

    // Helper to make an empty block with placeholder min/max
    let mut empty_block = |reset: &str, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>| CurrentBlock {
        reset: reset.to_string(),
        start,
        end,
        min_timestamp: Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap(),
        max_timestamp: Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(),
        assistant: AssistantInfo { content: 0, input_tokens: 0, output_tokens: 0, cache_creation_tokens: 0, cache_read_tokens: 0, total_tokens: 0 },
        user: UserInfo { content: 0 },
    };

    if guess.is_empty() {
        blocks.push(empty_block("gap", None, None));
        return blocks;
    }

    // First gap (always present)
    let _first_gap = (now, guess[0].end);
    blocks.push(empty_block("gap", None, None));

    for i in 0..guess.len() {
        let g = &guess[i];
        // The real block
        blocks.push(empty_block(&g.reset, Some(g.start), Some(g.end)));

        // Middle gap to next if there is a strict gap
        if i + 1 < guess.len() {
            let next = &guess[i + 1];
            if g.start > next.end {
                blocks.push(empty_block("gap", None, None));
            }
        }
    }

    // Last gap (always present)
    blocks.push(empty_block("gap", None, None));

    blocks
}

/// Aggregate events into blocks.
/// Blocks vector must contain: first gap, blocks/gaps in order, last gap.
pub fn aggregate_events_into_blocks(blocks: &mut Vec<CurrentBlock>, guess: &Vec<GuessBlock>, all: &Vec<ClaudeBarUsageEntry>) {
    if guess.is_empty() {
        for e in all.iter() {
            update_block(blocks.get_mut(0).unwrap(), e);
        }
        return;
    }

    // Precompute block indices by (start,end) timestamps and gap indices
    let mut block_index_map: HashMap<(i64, i64), usize> = HashMap::new();
    let mut first_gap_index: usize = 0;
    let mut last_gap_index: usize = 0;
    for (idx, b) in blocks.iter().enumerate() {
        match (b.start, b.end) {
            (Some(s), Some(e)) => {
                block_index_map.insert((s.timestamp(), e.timestamp()), idx);
            }
            (None, None) => {
                if idx == 0 { first_gap_index = idx; }
                last_gap_index = idx;
            }
            _ => {}
        }
    }

    for e in all.iter() {
        let ts = e.timestamp;
        let mut placed = false;

        // Inside any guess block?
        for g in guess.iter() {
            if ts >= g.start && ts <= g.end {
                let key = (g.start.timestamp(), g.end.timestamp());
                if let Some(&idx) = block_index_map.get(&key) {
                    update_block(blocks.get_mut(idx).unwrap(), e);
                    placed = true;
                    break;
                }
            }
        }
        if placed { continue; }

        // First gap
        if ts > guess[0].end {
            update_block(blocks.get_mut(first_gap_index).unwrap(), e);
            continue;
        }

        // Middle gaps
        let mut assigned = false;
        for i in 0..guess.len()-1 {
            let newer = &guess[i];
            let older = &guess[i+1];
            if ts <= newer.start && ts > older.end {
                let key_new = (newer.start.timestamp(), newer.end.timestamp());
                let key_old = (older.start.timestamp(), older.end.timestamp());
                if let (Some(&nidx), Some(&oidx)) = (block_index_map.get(&key_new), block_index_map.get(&key_old)) {
                    if nidx + 1 < oidx {
                        if let Some(rel) = blocks[nidx + 1..oidx].iter().position(|b| b.start.is_none() && b.end.is_none()) {
                            let gidx = nidx + 1 + rel;
                            update_block(blocks.get_mut(gidx).unwrap(), e);
                            assigned = true;
                            break;
                        }
                    }
                }
            }
        }
        if assigned { continue; }

        // Last gap
        update_block(blocks.get_mut(last_gap_index).unwrap(), e);
    }
}

fn update_block(block: &mut CurrentBlock, e: &ClaudeBarUsageEntry) {
    use crate::claudebar_types::UserRole;
    if e.timestamp < block.min_timestamp { block.min_timestamp = e.timestamp; }
    if e.timestamp > block.max_timestamp { block.max_timestamp = e.timestamp; }

    match e.role {
        UserRole::Assistant => {
            block.assistant.content += e.content_length as i32;
            block.assistant.input_tokens += e.usage.input_tokens as i64;
            block.assistant.output_tokens += e.usage.output_tokens as i64;
            block.assistant.cache_creation_tokens += e.usage.cache_creation_tokens as i64;
            block.assistant.cache_read_tokens += e.usage.cache_read_tokens as i64;
            block.assistant.total_tokens += e.usage.total_tokens as i64;
        }
        UserRole::User => {
            block.user.content += e.content_length as i32;
        }
        UserRole::Unknown => {}
    }
}
