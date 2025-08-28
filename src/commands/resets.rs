use chrono::Utc;
use rs_claude_bar::{
    claudebar_types::{CurrentBlock, GuessBlock, ClaudeBarUsageEntry},
    analyze::{
        load_all_entries,
        build_guess_blocks_from_entries,
        build_current_blocks_from_guess,
        aggregate_events_into_blocks,
    },
};
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

fn print_currentblocks_debug(blocks: &Vec<CurrentBlock>) {
    println!("CurrentBlocks: {:#?}", blocks);
}

// shared helpers are imported from rs_claude_bar::analyze
