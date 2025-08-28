use chrono::{DateTime, Duration, Utc};
use chrono::TimeZone;
use regex::Regex;
use rs_claude_bar::{
    claude_types::TranscriptEntry,
    claudebar_types::{AssistantInfo, CurrentBlock, GuessBlock, UserInfo, ClaudeBarUsageEntry, UserRole},
};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

// Read JSONL files, find only limit-reached entries, and print simple list lines:
// "<end UTC> | <start UTC>" where start = end - 5h

pub fn run(config: &rs_claude_bar::ConfigInfo) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);

    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    let mut limit_entries: Vec<ClaudeBarUsageEntry> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();

                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name().to_string_lossy().to_string();

                            // file modification date (optional)
                            let file_date = file
                                .metadata()
                                .ok()
                                .and_then(|meta| meta.modified().ok())
                                .map(DateTime::<Utc>::from);

                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for line in content.lines() {
                                    let line = line.trim();
                                    if line.is_empty() {
                                        continue;
                                    }
                                    // Fast path: only consider lines that likely contain limit text
                                    if !line.contains("5-hour limit reached") {
                                        continue;
                                    }
                                    if let Ok(transcript) = serde_json::from_str::<TranscriptEntry>(line) {
                                        let entry = ClaudeBarUsageEntry::from_transcript(
                                            &transcript,
                                            folder_name.clone(),
                                            file_name.clone(),
                                            file_date,
                                        );
                                        if entry.is_limit_reached {
                                            limit_entries.push(entry);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if limit_entries.is_empty() {
        println!("No limit messages found.");
        return;
    }

    // Sort by timestamp descending (most recent first)
    limit_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    // Build blocks from entries
    let mut blocks: Vec<GuessBlock> = Vec::new();
    for e in limit_entries.into_iter() {
        let ts = e.timestamp;
        let content = e.content_text.as_deref().unwrap_or("");
        if let Some(reset_time) = parse_reset_time(content) {
            if let Some(unlock) = calculate_unlock_time(ts, &reset_time) {
                let start = unlock - Duration::hours(5);
                blocks.push(GuessBlock {
                    msg_timestamp: ts,
                    reset: reset_time,
                    end: unlock,
                    start,
                });
            }
        }
    }

    // Deduplicate by timing (start,end) to remove retries before limit end
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    let mut unique: Vec<GuessBlock> = Vec::new();
    for b in blocks.into_iter() {
        let key = (b.start.timestamp(), b.end.timestamp());
        if seen.insert(key) {
            unique.push(b);
        }
    }

    // Keep newest first by end time
    unique.sort_by(|a, b| b.end.cmp(&a.end));

    // Print GuessBlocks table first (for reference)
    print_guessblocks_table(&unique);

    // Build CurrentBlocks: one per guess block, plus gaps
    let mut current_blocks = build_current_blocks(&unique);

    // Load all events and aggregate into blocks
    let mut all = load_all_entries(&base_path);
    all.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    aggregate_events_into_blocks(&mut current_blocks, &unique, &all);

    // Print CurrentBlocks as a debug table
    print_currentblocks_table(&current_blocks);
}

/// Parse reset time like "10pm" or "10:30 pm" from content
fn parse_reset_time(content: &str) -> Option<String> {
    // Pattern: "Reset time: 10pm" or "resets 10pm"
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

/// Calculate unlock time based on limit timestamp and reset time string
fn calculate_unlock_time(limit_timestamp: DateTime<Utc>, reset_time: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"(\d{1,2})(?::(\d{2}))?\s*(am|pm)").ok()?;
    let caps = re.captures(reset_time)?;

    let hour: u32 = caps.get(1)?.as_str().parse().ok()?;
    let minute: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let is_pm = caps.get(3)?.as_str().eq_ignore_ascii_case("pm");

    let hour_24 = match (hour, is_pm) {
        (12, false) => 0,    // 12am -> 0
        (12, true) => 12,    // 12pm -> 12
        (h, false) => h,     // am hours
        (h, true) => h + 12, // pm hours
    };
    if hour_24 >= 24 || minute >= 60 { return None; }

    let limit_date = limit_timestamp.date_naive();
    let same_day = limit_date
        .and_hms_opt(hour_24, minute, 0)?
        .and_local_timezone(Utc)
        .single()?;

    // If reset time already passed at/ before limit timestamp, use next day
    let unlock = if same_day > limit_timestamp { same_day } else {
        (limit_date + chrono::Days::new(1))
            .and_hms_opt(hour_24, minute, 0)?
            .and_local_timezone(Utc)
            .single()?
    };
    Some(unlock)
}

fn print_guessblocks_table(rows: &Vec<GuessBlock>) {
    println!("timestamp|reset|end|start");
    for b in rows.iter() {
        println!(
            "{}|{}|{}|{}",
            b.msg_timestamp.format("%Y-%m-%d %H:%M UTC"),
            b.reset,
            b.end.format("%Y-%m-%d %H:%M UTC"),
            b.start.format("%Y-%m-%d %H:%M UTC"),
        );
    }
}

fn build_current_blocks(guess: &Vec<GuessBlock>) -> Vec<CurrentBlock> {
    let now = Utc::now();
    let mut blocks: Vec<CurrentBlock> = Vec::new();

    // Helper to make an empty block with placeholder min/max
    let empty_block = |reset: &str, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>| CurrentBlock {
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

    // First gap: now -> first guess end (stored as gap without explicit bounds)
    let _first_gap_start = guess[0].end; // informational
    let _first_gap_end = now;
    blocks.push(empty_block("gap", None, None));

    // For each guess block, add the block then the gap to next
    for (i, g) in guess.iter().enumerate() {
        blocks.push(empty_block(&g.reset, Some(g.start), Some(g.end)));
        if i + 1 < guess.len() {
            // Gap between this.start and next.end
            blocks.push(empty_block("gap", None, None));
        }
    }

    // Last gap: last guess start -> forever (no explicit bounds)
    blocks.push(empty_block("gap", None, None));

    blocks
}

fn aggregate_events_into_blocks(blocks: &mut Vec<CurrentBlock>, guess: &Vec<GuessBlock>, all: &Vec<ClaudeBarUsageEntry>) {
    if guess.is_empty() {
        // Everything maps to the single gap block at index 0
        for e in all.iter() {
            update_block(&mut blocks[0], e);
        }
        return;
    }

    // Iterate events in descending time and place them
    for e in all.iter() {
        let ts = e.timestamp;

        // If inside any guess block
        let mut placed = false;
        for (i, g) in guess.iter().enumerate() {
            if ts >= g.start && ts <= g.end {
                let idx = 1 + i * 2; // position of guess block inside blocks vector
                update_block(&mut blocks[idx], e);
                placed = true;
                break;
            }
        }
        if placed { continue; }

        // Otherwise map to the appropriate gap
        let first_end = guess[0].end;
        if ts > first_end {
            // First gap: index 0
            update_block(&mut blocks[0], e);
            continue;
        }
        // Middle gaps
        let mut assigned_middle = false;
        for i in 0..(guess.len() - 1) {
            let newer = &guess[i];
            let older = &guess[i + 1];
            if ts <= newer.start && ts > older.end {
                let idx = 1 + i * 2 + 1; // gap after block i
                update_block(&mut blocks[idx], e);
                assigned_middle = true;
                break;
            }
        }
        if assigned_middle { continue; }

        // Last gap: after the last guess start back in time
        let last_idx = blocks.len() - 1;
        update_block(&mut blocks[last_idx], e);
    }
}

fn update_block(block: &mut CurrentBlock, e: &ClaudeBarUsageEntry) {
    // Update min/max
    if e.timestamp < block.min_timestamp { block.min_timestamp = e.timestamp; }
    if e.timestamp > block.max_timestamp { block.max_timestamp = e.timestamp; }

    match e.role {
        UserRole::Assistant => {
            // Count assistant content and tokens (separately)
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

fn print_currentblocks_table(blocks: &Vec<CurrentBlock>) {
    println!("type|reset|start|end|min|max|assistant_content|assistant_in|assistant_out|assistant_cache_create|assistant_cache_read|assistant_total|user_content");
    for b in blocks.iter() {
        let t = if b.start.is_some() && b.end.is_some() { "block" } else { "gap" };
        let start = b.start.map(|d| d.format("%Y-%m-%d %H:%M UTC").to_string()).unwrap_or_default();
        let end = b.end.map(|d| d.format("%Y-%m-%d %H:%M UTC").to_string()).unwrap_or_default();
        let has_events = b.max_timestamp >= b.min_timestamp;
        let min = if has_events { b.min_timestamp.format("%Y-%m-%d %H:%M UTC").to_string() } else { String::new() };
        let max = if has_events { b.max_timestamp.format("%Y-%m-%d %H:%M UTC").to_string() } else { String::new() };
        println!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            t,
            b.reset,
            start,
            end,
            min,
            max,
            b.assistant.content,
            b.assistant.input_tokens,
            b.assistant.output_tokens,
            b.assistant.cache_creation_tokens,
            b.assistant.cache_read_tokens,
            b.assistant.total_tokens,
            b.user.content,
        );
    }
}

// Local reader to get every entry from ~/.claude/projects as ClaudeBarUsageEntry
fn load_all_entries(base_path: &str) -> Vec<ClaudeBarUsageEntry> {
    let mut usage_entries = Vec::new();
    let projects = Path::new(base_path);
    if !projects.exists() { return usage_entries; }

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
                                    if line.is_empty() { continue; }
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
