use chrono::{DateTime, Duration, Utc};
use chrono::TimeZone;
use rs_claude_bar::{
    claudebar_types::{CurrentBlock, GuessBlock, ClaudeBarUsageEntry},
    analyze::{
        load_all_entries,
        build_guess_blocks_from_entries,
        build_current_blocks_from_guess,
        aggregate_events_into_blocks,
    },
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

    // Use shared loader to get all entries and then filter limit ones
    let mut all_entries = load_all_entries(&base_path);
    let limit_entries: Vec<ClaudeBarUsageEntry> = all_entries
        .iter()
        .cloned()
        .filter(|e| e.is_limit_reached)
        .collect();

    if limit_entries.is_empty() {
        println!("No limit messages found.");
        return;
    }

    // Build GuessBlocks via shared helper (handles dedup, sort, projected block)
    let guess_blocks: Vec<GuessBlock> = build_guess_blocks_from_entries(&limit_entries);

    // Print GuessBlocks via debug print (readable)
    print_guessblocks_debug(&guess_blocks);

    // Build CurrentBlocks: one per guess block, plus gaps
    let mut current_blocks: Vec<CurrentBlock> = build_current_blocks_from_guess(&guess_blocks, Utc::now());

    // Aggregate all events into blocks
    all_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    aggregate_events_into_blocks(&mut current_blocks, &guess_blocks, &all_entries);

    // Remove blocks with zero totals (no assistant tokens and no user content and no assistant content)
    let filtered: Vec<CurrentBlock> = current_blocks
        .into_iter()
        .filter(|b| b.assistant.total_tokens > 0 || b.assistant.content > 0 || b.user.content > 0)
        .collect();

    // Print CurrentBlocks via debug print (readable)
    print_currentblocks_debug(&filtered);
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
