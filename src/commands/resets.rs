use chrono::{DateTime, Duration, Utc};
use chrono::TimeZone;
use rs_claude_bar::{
    claude_types::TranscriptEntry,
    claudebar_types::{AssistantInfo, CurrentBlock, GuessBlock, UserInfo, ClaudeBarUsageEntry, UserRole},
    analyze::{load_all_entries, parse_reset_time, calculate_unlock_time},
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

    // Add one real projected block from latest end -> +5h
    if let Some(latest) = unique.first().cloned() {
        let start = latest.end;
        let end = start + Duration::hours(5);
        let projected = GuessBlock {
            msg_timestamp: start,
            reset: "projected".to_string(),
            start,
            end,
        };
        // Insert as newest (front)
        if unique.first().map(|b| b.start != start || b.end != end).unwrap_or(true) {
            unique.insert(0, projected);
        }
    }

    // Print GuessBlocks via debug print (readable)
    print_guessblocks_debug(&unique);

    // Build CurrentBlocks: one per guess block, plus gaps
    let mut current_blocks = build_current_blocks(&unique);

    // Load all events and aggregate into blocks (shared helper)
    let mut all = load_all_entries(&base_path);
    all.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    aggregate_events_into_blocks(&mut current_blocks, &unique, &all);

    // Print CurrentBlocks via debug print (readable)
    print_currentblocks_debug(&current_blocks);
}


fn print_guessblocks_debug(rows: &Vec<GuessBlock>) {
    println!("GuessBlocks: {:#?}", rows);
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

fn print_currentblocks_debug(blocks: &Vec<CurrentBlock>) {
    println!("CurrentBlocks: {:#?}", blocks);
}

// shared helpers are imported from rs_claude_bar::analyze
