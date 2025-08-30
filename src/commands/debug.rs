use crate::{
    cache::CacheManager,
    common::colors::*,
    claudebar_types::{
        config::ConfigInfo,
        display::HeaderInfo,
    },
    display::table::TableCreator,
    formatting::{
        format_date,
        format_text,
    }
};
use std::path::Path;

pub fn run(config: &ConfigInfo, cache_manager: &mut CacheManager, limits: bool) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);

    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    if limits {
        run_limits_debug_cache(cache_manager);
    }
}

/// Debug limits using only cache data (no filesystem access)
fn run_limits_debug_cache(cache_manager: &CacheManager) {
    println!(
        "{bold}{cyan}🚫 DEBUG: Limits Analysis (Cache-only){reset}",
        bold = BOLD,
        cyan = CYAN,
        reset = RESET,
    );
    println!();

    let cache_info = cache_manager.get_cache();
    let mut all_block_lines = Vec::new();

    // Collect all block lines from all files (now stored in a HashMap keyed by timestamp)
    for (folder_name, cached_folder) in &cache_info.folders {
        for (file_name, cached_file) in &cached_folder.files {
            for (ts, block_line) in &cached_file.blocks {
                all_block_lines.push((folder_name.as_str(), file_name.as_str(), ts.clone(), block_line));
            }
        }
    }

    if all_block_lines.is_empty() {
        println!("✅ No limit events found in cache");
        return;
    }

    // Sort by timestamp
    all_block_lines.sort_by_key(|(_, _, ts, _)| ts.clone());

    // Calculate dynamic column widths based on cache data
    let max_folder_width = all_block_lines.iter()
        .map(|(folder_name, _, _, _)| folder_name.len())
        .max()
        .unwrap_or(10)
        .max(8); // Minimum width for "📁 Folder" header
    
    let max_file_width = all_block_lines.iter()
        .map(|(_, file_name, _, _)| file_name.len())
        .max()
        .unwrap_or(10)
        .max(6); // Minimum width for "📄 File" header

    // Display table of limit events with dynamic widths
    let headers = vec![
        HeaderInfo::new("Folder", max_folder_width),
        HeaderInfo::new("File", max_file_width),
        HeaderInfo::new("Date", 11),
        HeaderInfo::new("Unlock", 11),
        HeaderInfo::new("When", 4),
    ];
    let mut tc = TableCreator::new(headers);

    for (folder_name, file_name, ts, block_line) in &all_block_lines {
        let unlock_time = if let Some(unlock) = &block_line.unlock_timestamp {
            format_date(*unlock, 11)
        } else {
            "Unknown".to_string()
        };

        tc.add_row(vec![
            format_text(folder_name, max_folder_width),
            format_text(file_name, max_file_width),
            format_date(*ts, 11),
            unlock_time,
            format_text(&block_line.reset_text, 4),
        ]);
    }

    tc.display(false);

    println!();
    println!(
        "{bold}📊 Summary: {} limit events from cache{reset}",
        all_block_lines.len(),
        bold = BOLD,
        reset = RESET
    );
}
