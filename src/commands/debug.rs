use chrono::{DateTime, Utc, Timelike};
use crate::{
    analyzer::{
        Analyzer
    },
    cache::CacheManager,
    claude_types::transcript_entry::{ClaudeEntry, TranscriptEntry},
    commands::shared_types::UsageBlock,
    common::colors::*,
    claudebar_types::{
        config::ConfigInfo,
        usage_entry::ClaudeBarUsageEntry,
        display::HeaderInfo,
    },
    display::table::TableCreator,
    utils::formatting::{
        format_file_size,
        format_duration,
        format_date,
        format_number_with_separators,
        format_cache_status,
        format_text,
    }
};
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
struct FileParseStats {
    total_lines: usize,
    successful_parses: usize,
    empty_lines: usize,
    parse_errors: usize,
    min_timestamp: Option<DateTime<Utc>>,
    max_timestamp: Option<DateTime<Utc>>,
    total_output_tokens: u32,
}

// Removed tabled struct - using cool custom TableCreator instead!

pub fn run(config: &ConfigInfo, cache_manager: &mut CacheManager, parse: bool, cache: bool, file: Option<String>, blocks: bool, gaps: bool, limits: bool, files: bool) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);

    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    if let Some(filepath) = file {
        run_single_file_debug(cache_manager, &base_path, &filepath);
    } else if parse {
        //run_parse_debug(cache_manager, &base_path);
    } else if limits {
        run_limits_debug_cache(cache_manager);
    } else if blocks || gaps {
        run_blocks_debug(config, cache_manager, gaps, limits);
    } else if files {
        run_files_debug(cache_manager, &base_path);
    } else {
        // Default behavior - show table view
        if parse {
            //run_parse_debug(cache_manager, &base_path); // V1: Now cache-based
        } else if cache {
            //run_parse_debug_v2(config, &base_path, no_cache); // V2: New cached system (--no-cache forces full reparse)
        } else {
            //run_parse_debug(cache_manager, &base_path); // Default: Now cache-based
        }
    }
}

/* V2: New cached system
fn run_parse_debug_v2(config: &ConfigInfo, base_path: &str, no_cache: bool) {
    let cache_status = if no_cache { "No Cache" } else { "Cached" };
    println!(
        "{bold}{cyan}🔍 DEBUG: JSONL Parse Analysis V2 ({cache_status}){reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
        cache_status = cache_status,
    );
    println!();

    // Create analyzer with cache and config
    let mut analyzer = Analyzer::new(config.clone());
    
    // Analyze files and determine which need parsing
    let (needs_parsing, cached_files) = if no_cache {
        // Force all files to be reparsed
        let all_files = analyzer.scan_files(base_path);
        (all_files, Vec::new())
    } else {
        //analyzer.analyze_files(base_path)
    };
    
    println!("📊 File Analysis Results:");
    println!("  📝 Files needing parsing: {}", needs_parsing.len());
    println!("  ✅ Files up-to-date in cache: {}", cached_files.len());
    println!();
    
    if !needs_parsing.is_empty() {
        println!("🔄 Parsing new/modified files:");
        for file in &needs_parsing {
            println!("   📄 {}/{}", file.folder_name, file.file_name);
        }
        
        // Parse and cache files with per-hour info
        analyzer.parse_and_cache_files(needs_parsing, no_cache);
        
        // TODO: Uncomment when feature is ready
        // Save updated cache
        // if let Err(e) = analyzer.save_cache() {
        //     eprintln!("⚠️  Warning: Failed to save cache: {}", e);
        // }
        println!();
    }
    
    if !cached_files.is_empty() {
        println!("✅ Using cached data for {} files", cached_files.len());
        println!();
    }
    
    // TODO: Generate analysis table from cached per-hour data
    println!("📈 Analysis complete! (Per-hour cache implementation pending)");
}
 */

 /*
fn run_parse_debug(cache_manager: &CacheManager, base_path: &str) {
    println!(
        "{bold}{cyan}🔍 DEBUG: JSONL Parse Analysis (Cache-based){reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
    );
    println!();
    let analysis_start = std::time::Instant::now();

    let cache_info = cache_manager.get_cache();
    let mut all_file_stats = Vec::new();

    for (folder_name, cached_folder) in &cache_info.folders {
        for (file_name, cached_file) in &cached_folder.files {
            let file_path = format!("{}/{}", folder_name, file_name);
            let full_path = format!("{}/{}/{}", base_path, folder_name, file_name);
            
            if let Ok(content) = fs::read_to_string(full_path) {
                let stats = parse_file_content(&content);
                all_file_stats.push((file_path, stats));
            }
        }
    }

    if all_file_stats.is_empty() {
        println!("❌ No JSONL files found in cache for {}", base_path);
        return;
    }
    let analysis_duration = analysis_start.elapsed();
    println!("🔍 Parsed {} files, (analysis took {:.1}ms):", 
            all_file_stats.len(), 
            analysis_duration.as_secs_f64() * 1000.0);
    println!();

    // Create table using TableCreator
    let headers = vec![
        HeaderInfo { label: "File Path", width: 80 },
        HeaderInfo { label: "Total", width: 5 },
        HeaderInfo { label: "Parsed", width: 7 },
        HeaderInfo { label: "Empty", width: 6 },
        HeaderInfo { label: "Errors", width: 7 },
        HeaderInfo { label: "Min Timestamp", width: 12 },
        HeaderInfo { label: "Max Timestamp", width: 12 },
        HeaderInfo { label: "Tokens", width: 8 },
    ];
    let mut tc = TableCreator::new(headers);

    for (file_path, stats) in &all_file_stats {
        let truncated_path = if file_path.len() > 80 {
            format!("...{}", &file_path[file_path.len() - 77..])
        } else {
            file_path.to_string()
        };

        let min_ts = stats.min_timestamp
            .map(|ts| ts.format("%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let max_ts = stats.max_timestamp
            .map(|ts| ts.format("%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let success_rate = if stats.total_lines > stats.empty_lines {
            let parseable_lines = stats.total_lines - stats.empty_lines;
            (stats.successful_parses as f64 / parseable_lines as f64) * 100.0
        } else {
            0.0
        };

        // Color coding based on success rate
        let colored_path = if success_rate >= 95.0 {
            format!("{green}{:<80}{reset}", truncated_path, green = GREEN, reset = RESET)
        } else if success_rate >= 80.0 {
            format!("{yellow}{:<80}{reset}", truncated_path, yellow = YELLOW, reset = RESET)
        } else {
            format!("{red}{:<80}{reset}", truncated_path, red = RED, reset = RESET)
        };

        tc.add_row(vec![
            colored_path,
            format!("{:>5}", stats.total_lines),
            format!("{:>7}", stats.successful_parses),
            format!("{:>6}", stats.empty_lines),
            format!("{:>7}", stats.parse_errors),
            format!("{:>12}", min_ts),
            format!("{:>12}", max_ts),
            format!("{:>8}", format_number_with_separators(stats.total_output_tokens)),
        ]);
    }

    tc.display(false);

    // Print summary
    print_summary(&all_file_stats);
}

fn parse_file_content(content: &str) -> FileParseStats {
    let mut stats = FileParseStats::default();

    for line in content.lines() {
        stats.total_lines += 1;

        if line.trim().is_empty() {
            stats.empty_lines += 1;
            continue;
        }

        match serde_json::from_str::<ClaudeEntry>(line) {
            Ok(ClaudeEntry::Transcript(entry)) => {
                stats.successful_parses += 1;
                
                // Parse timestamp and update bounds if available
                let timestamp_str = &entry.timestamp;
                if let Ok(timestamp) = DateTime::parse_from_rfc3339(timestamp_str) {
                        let timestamp_utc = timestamp.with_timezone(&Utc);
                        if stats.min_timestamp.is_none() || timestamp_utc < stats.min_timestamp.unwrap() {
                            stats.min_timestamp = Some(timestamp_utc);
                        }
                        if stats.max_timestamp.is_none() || timestamp_utc > stats.max_timestamp.unwrap() {
                            stats.max_timestamp = Some(timestamp_utc);
                        }
                    }
                }

                // Add output tokens if available
                if let Some(usage) = &entry.message.usage {
                    stats.total_output_tokens += usage.output_tokens;
                }
            }
            Err(_) => {
                stats.parse_errors += 1;
            }
        }
    }

    stats
}
*/

fn print_summary(all_file_stats: &[(String, FileParseStats)]) {
    let total_files = all_file_stats.len();
    let total_lines: usize = all_file_stats.iter().map(|(_, stats)| stats.total_lines).sum();
    let total_parsed: usize = all_file_stats.iter().map(|(_, stats)| stats.successful_parses).sum();
    let total_empty: usize = all_file_stats.iter().map(|(_, stats)| stats.empty_lines).sum();
    let total_errors: usize = all_file_stats.iter().map(|(_, stats)| stats.parse_errors).sum();
    let total_tokens: u32 = all_file_stats.iter().map(|(_, stats)| stats.total_output_tokens).sum();

    let global_min = all_file_stats.iter()
        .filter_map(|(_, stats)| stats.min_timestamp)
        .min();
    
    let global_max = all_file_stats.iter()
        .filter_map(|(_, stats)| stats.max_timestamp)
        .max();

    let overall_success_rate = if total_lines > total_empty {
        let parseable_lines = total_lines - total_empty;
        (total_parsed as f64 / parseable_lines as f64) * 100.0
    } else {
        0.0
    };

    println!();
    println!(
        "{bold}{green}📊 Summary:{reset}",
        bold = { BOLD },
        green = { GREEN },
        reset = { RESET },
    );
    
    println!("   Files processed: {}", total_files);
    println!("   Total lines: {}", format_number_with_separators(total_lines as u32));
    println!("   Successfully parsed: {} ({:.1}%)", format_number_with_separators(total_parsed as u32), overall_success_rate);
    println!("   Empty lines: {}", format_number_with_separators(total_empty as u32));
    println!("   Parse errors: {}", format_number_with_separators(total_errors as u32));
    println!("   Total output tokens: {}", format_number_with_separators(total_tokens));
    
    if let Some(min_ts) = global_min {
        println!("   Date range: {} to {}", 
                min_ts.format("%Y-%m-%d %H:%M UTC"), 
                global_max.unwrap().format("%Y-%m-%d %H:%M UTC"));
    }

    // Show files with parsing issues
    let problematic_files: Vec<_> = all_file_stats.iter()
        .filter(|(_, stats)| {
            let parseable_lines = stats.total_lines - stats.empty_lines;
            if parseable_lines > 0 {
                let success_rate = (stats.successful_parses as f64 / parseable_lines as f64) * 100.0;
                success_rate < 95.0
            } else {
                false
            }
        })
        .collect();

    if !problematic_files.is_empty() {
        println!();
        println!(
            "{bold}{yellow}⚠️  Files with parsing issues:{reset}",
            bold = { BOLD },
            yellow = { YELLOW },
            reset = { RESET },
        );
        
        for (file_path, stats) in problematic_files {
            let parseable_lines = stats.total_lines - stats.empty_lines;
            let success_rate = (stats.successful_parses as f64 / parseable_lines as f64) * 100.0;
            println!("   {} - {:.1}% success rate ({} errors)", file_path, success_rate, stats.parse_errors);
        }
    }
}


fn run_single_file_debug(cache_manager: &CacheManager, base_path: &str, target_file: &str) {
    println!(
        "{bold}{cyan}🔍 DEBUG: Single File Parse Analysis (Cache-based){reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
    );
    println!("Target file: {}", target_file);
    println!();

    let cache_info = cache_manager.get_cache();
    let mut file_found = false;

    for (folder_name, cached_folder) in &cache_info.folders {
        for (file_name, _cached_file) in &cached_folder.files {
            let file_path = format!("{}/{}", folder_name, file_name);
            
            // Check if this is the target file (partial match)
            if file_path.contains(target_file) || file_name.contains(target_file) {
                file_found = true;
                println!(
                    "{bold}📄 Analyzing: {}{reset}",
                    file_path,
                    bold = { BOLD },
                    reset = { RESET },
                );
                println!();
                
                let full_path = format!("{}/{}/{}", base_path, folder_name, file_name);
                if let Ok(content) = fs::read_to_string(full_path) {
                    analyze_single_file_with_errors(&content, &file_path);
                } else {
                    println!("❌ Could not read file: {}", file_path);
                }
                return;
            }
        }
    }

    if !file_found {
        println!("❌ File not found: {}", target_file);
        println!();
        println!("Available files:");
        list_available_files_from_cache(cache_manager);
    }
}

fn analyze_single_file_with_errors(content: &str, _file_path: &str) {
    let mut stats = FileParseStats::default();
    let mut parse_errors = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        stats.total_lines += 1;

        if line.trim().is_empty() {
            stats.empty_lines += 1;
            continue;
        }

        match serde_json::from_str::<ClaudeEntry>(line) {
            Ok(ClaudeEntry::Transcript(entry)) => {
                stats.successful_parses += 1;
                
                // Parse timestamp and update bounds if available
                let timestamp_str = &entry.timestamp;
                if let Ok(timestamp) = DateTime::parse_from_rfc3339(timestamp_str) {
                        let timestamp_utc = timestamp.with_timezone(&Utc);
                        if stats.min_timestamp.is_none() || timestamp_utc < stats.min_timestamp.unwrap() {
                            stats.min_timestamp = Some(timestamp_utc);
                        }
                        if stats.max_timestamp.is_none() || timestamp_utc > stats.max_timestamp.unwrap() {
                            stats.max_timestamp = Some(timestamp_utc);
                        }
                    
                }

                // Add output tokens if available
                if let Some(usage) = &entry.message.usage {
                    stats.total_output_tokens += usage.output_tokens;
                }
            }
            Ok(ClaudeEntry::Summary { .. }) | Ok(ClaudeEntry::Unknown(_)) => {
                // Skip summary and unknown entries for now
            },
            Err(parse_error) => {
                stats.parse_errors += 1;
                parse_errors.push((line_num + 1, parse_error.to_string(), line.to_string()));
            }
        }
    }

    // Print file stats
    println!(
        "{bold}📊 File Statistics:{reset}",
        bold = { BOLD },
        reset = { RESET },
    );
    println!("   Total lines: {}", stats.total_lines);
    println!("   Successfully parsed: {}", stats.successful_parses);
    println!("   Empty lines: {}", stats.empty_lines);
    println!("   Parse errors: {}", stats.parse_errors);
    println!("   Total output tokens: {}", format_number_with_separators(stats.total_output_tokens));

    if let Some(min_ts) = stats.min_timestamp {
        println!(
            "   Date range: {} to {}",
            min_ts.format("%Y-%m-%d %H:%M UTC"),
            stats.max_timestamp.unwrap().format("%Y-%m-%d %H:%M UTC")
        );
    }

    let success_rate = if stats.total_lines > stats.empty_lines {
        let parseable_lines = stats.total_lines - stats.empty_lines;
        (stats.successful_parses as f64 / parseable_lines as f64) * 100.0
    } else {
        0.0
    };
    println!("   Success rate: {:.1}%", success_rate);

    if !parse_errors.is_empty() {
        println!();
        println!(
            "{bold}{red}❌ Parse Errors ({} total):{reset}",
            parse_errors.len(),
            bold = { BOLD },
            red = { RED },
            reset = { RESET },
        );
        println!();

        for (line_num, error, line_content) in &parse_errors {
            println!(
                "{bold}Line {}:{reset}",
                line_num,
                bold = { BOLD },
                reset = { RESET },
            );
            println!("  Error: {}", error);
            
            // Show first 200 chars of problematic line
            let truncated_line = if line_content.len() > 200 {
                format!("{}...", &line_content[..200])
            } else {
                line_content.clone()
            };
            println!("  Content: {}", truncated_line);
            
            // Try to parse as generic JSON and show structure
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(line_content) {
                if let Some(obj) = json_value.as_object() {
                    let keys: Vec<&String> = obj.keys().collect();
                    println!("  Available keys: {:?}", keys);
                    
                    // Check for common problematic fields
                    if let Some(timestamp) = obj.get("timestamp") {
                        println!("  Timestamp field: {:?}", timestamp);
                    }
                    if let Some(message) = obj.get("message") {
                        if let Some(msg_obj) = message.as_object() {
                            let msg_keys: Vec<&String> = msg_obj.keys().collect();
                            println!("  Message keys: {:?}", msg_keys);
                        }
                    }
                }
            } else {
                println!("  ⚠️  Not valid JSON");
            }
            
            println!();
        }

        // Provide suggestions
        println!(
            "{bold}{yellow}💡 Parsing Suggestions:{reset}",
            bold = { BOLD },
            yellow = { YELLOW },
            reset = { RESET },
        );
        
        // Analyze common error patterns
        let missing_field_errors = parse_errors.iter()
            .filter(|(_, error, _)| error.contains("missing field"))
            .count();
        
        let type_mismatch_errors = parse_errors.iter()
            .filter(|(_, error, _)| error.contains("invalid type"))
            .count();
            
        let unknown_field_errors = parse_errors.iter()
            .filter(|(_, error, _)| error.contains("unknown field"))
            .count();

        if missing_field_errors > 0 {
            println!("   • {} errors due to missing required fields", missing_field_errors);
            println!("     Consider making more fields optional in TranscriptEntry");
        }
        
        if type_mismatch_errors > 0 {
            println!("   • {} errors due to type mismatches", type_mismatch_errors);
            println!("     Check field types in TranscriptEntry struct");
        }
        
        if unknown_field_errors > 0 {
            println!("   • {} errors due to unknown fields", unknown_field_errors);
            println!("     Consider adding #[serde(flatten)] or #[serde(other)]");
        }
    } else {
        println!();
        println!(
            "{bold}{green}✅ All lines parsed successfully!{reset}",
            bold = { BOLD },
            green = { GREEN },
            reset = { RESET },
        );
    }
}

fn list_available_files_from_cache(cache_manager: &CacheManager) {
    let cache_info = cache_manager.get_cache();
    
    for (folder_name, cached_folder) in &cache_info.folders {
        for (file_name, _cached_file) in &cached_folder.files {
            println!("   {}/{}", folder_name, file_name);
        }
    }
}


fn run_blocks_debug(config: &ConfigInfo, cache_manager: &CacheManager, gaps: bool, limits: bool) {
    if gaps {
        run_gaps_debug_cache(cache_manager);
    } else {
        run_blocks_debug_cache(cache_manager);
    }
}

/// Debug 5-hour blocks using only cache data
fn run_blocks_debug_cache(cache_manager: &CacheManager) {
    println!(
        "{bold}{cyan}🟦 DEBUG: 5-Hour Blocks Analysis (Cache-only){reset}",
        bold = BOLD,
        cyan = CYAN,
        reset = RESET,
    );
    println!();

    let cache_info = cache_manager.get_cache();
    let mut all_per_hour_blocks = Vec::new();

    // Collect all per-hour blocks from all files
    for (folder_name, cached_folder) in &cache_info.folders {
        for (file_name, cached_file) in &cached_folder.files {
            for (hour_start, per_hour_block) in &cached_file.per_hour {
                all_per_hour_blocks.push((folder_name.as_str(), file_name.as_str(), per_hour_block));
            }
        }
    }

    if all_per_hour_blocks.is_empty() {
        println!("✅ No hourly usage blocks found in cache");
        return;
    }

    // Sort by hour start
    all_per_hour_blocks.sort_by_key(|(_, _, block)| block.hour_start);

    // Group into 5-hour windows and display
    let headers = vec![
        HeaderInfo::new("5-Hour Window",15),
        HeaderInfo::new("Total Tokens",12),
        HeaderInfo::new("Assistant",10),
        HeaderInfo::new("User",8),
        HeaderInfo::new("Entries",8),
        HeaderInfo::new("Content",10),
    ];
    let mut tc = TableCreator::new(headers);

    // Group by 5-hour windows (00:00-05:00, 05:00-10:00, etc.)
    let mut five_hour_groups = std::collections::HashMap::new();
    
    for (folder_name, file_name, block) in &all_per_hour_blocks {
        let five_hour_start = (block.hour_start.hour() / 5) * 5;
        let five_hour_key = block.hour_start.with_hour(five_hour_start).unwrap();
        
        let group = five_hour_groups.entry(five_hour_key).or_insert_with(|| {
            (0u32, 0u32, 0u32, 0u32, 0u64) // (total_tokens, assistant_msgs, user_msgs, entries, content_len)
        });
        
        group.0 += block.input_tokens + block.output_tokens + block.cache_creation_tokens + block.cache_read_tokens;
        group.1 += block.assistant_messages;
        group.2 += block.user_messages;
        group.3 += block.entry_count;
        group.4 += block.total_content_length;
    }

    // Display sorted 5-hour blocks
    let mut sorted_groups: Vec<_> = five_hour_groups.iter().collect();
    sorted_groups.sort_by_key(|(time, _)| *time);

    let groups_count = sorted_groups.len();
    for (start_time, (total_tokens, assistant_msgs, user_msgs, entries, content_len)) in &sorted_groups {
        let end_time = **start_time + chrono::Duration::hours(5);
        let window_str = format!("{}-{}", 
            start_time.format("%H:%M"), 
            end_time.format("%H:%M")
        );
        
        tc.add_row(vec![
            format_text(&window_str, 15),
            format_number_with_separators(*total_tokens),
            format_number_with_separators(*assistant_msgs),
            format_number_with_separators(*user_msgs), 
            format_number_with_separators(*entries),
            format_file_size(*content_len),
        ]);
    }

    tc.display(false);

    println!();
    println!(
        "{bold}📊 Summary: {} 5-hour blocks from {} hourly blocks{reset}",
        groups_count,
        all_per_hour_blocks.len(),
        bold = BOLD,
        reset = RESET
    );
}

/// Debug gaps between usage blocks using cache data  
fn run_gaps_debug_cache(cache_manager: &CacheManager) {
    println!(
        "{bold}{cyan}⛳ DEBUG: Usage Gaps Analysis (Cache-only){reset}",
        bold = BOLD,
        cyan = CYAN,
        reset = RESET,
    );
    println!();

    let cache_info = cache_manager.get_cache();
    let mut all_per_hour_blocks = Vec::new();

    // Collect all per-hour blocks from all files
    for (_folder_name, cached_folder) in &cache_info.folders {
        for (_file_name, cached_file) in &cached_folder.files {
            for (hour_start, per_hour_block) in &cached_file.per_hour {
                all_per_hour_blocks.push(per_hour_block);
            }
        }
    }

    if all_per_hour_blocks.len() < 2 {
        println!("✅ Need at least 2 hourly blocks to detect gaps");
        return;
    }

    // Sort by hour start
    all_per_hour_blocks.sort_by_key(|block| block.hour_start);

    // Find gaps between consecutive blocks
    let mut gaps = Vec::new();
    for i in 1..all_per_hour_blocks.len() {
        let prev_block = &all_per_hour_blocks[i-1];
        let curr_block = &all_per_hour_blocks[i];
        
        let expected_next = prev_block.hour_end + chrono::Duration::seconds(1);
        let gap_duration = curr_block.hour_start - expected_next;
        
        if gap_duration > chrono::Duration::hours(1) {
            gaps.push((prev_block.hour_end, curr_block.hour_start, gap_duration));
        }
    }

    if gaps.is_empty() {
        println!("✅ No significant gaps found between usage blocks");
        return;
    }

    // Display gaps
    let headers = vec![
        HeaderInfo::new("Gap Start", 16 ),
        HeaderInfo::new("Gap End",16 ),
        HeaderInfo::new("Duration", 12 ),
        HeaderInfo::new("Gap Size", 10 ),
    ];
    let mut tc = TableCreator::new(headers);

    for (gap_start, gap_end, duration) in &gaps {
        tc.add_row(vec![
            format_date(*gap_start, 16),
            format_date(*gap_end, 16),
            format_duration(*duration, 12),
            format!("{:.1}h", duration.num_minutes() as f64 / 60.0),
        ]);
    }

    tc.display(false);

    println!();
    println!(
        "{bold}📊 Found {} gaps in usage timeline{reset}",
        gaps.len(),
        bold = BOLD,
        reset = RESET
    );
}

fn load_usage_entries(cache_manager: &CacheManager, base_path: &str) -> Vec<ClaudeBarUsageEntry> {
    let mut usage_entries = Vec::new();
    let cache_info = cache_manager.get_cache();

    for (folder_name, cached_folder) in &cache_info.folders {
        for (file_name, cached_file) in &cached_folder.files {
            let file_date = Some(cached_file.modified_time);
            let full_path = format!("{}/{}/{}", base_path, folder_name, file_name);

            if let Ok(content) = fs::read_to_string(full_path) {
                for line in content.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }

                    if let Ok(transcript) = serde_json::from_str::<TranscriptEntry>(line) {
                        let usage_entry = ClaudeBarUsageEntry::from_transcript(
                            &transcript,
                            folder_name.clone(),
                            file_name.clone(),
                            file_date,
                        );
                        usage_entries.push(usage_entry);
                    }
                }
            }
        }
    }

    usage_entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    usage_entries
}

fn compute_blocks(entries: &[ClaudeBarUsageEntry]) -> Vec<UsageBlock> {
    // Simple 5-hour block computation
    let mut blocks = Vec::new();
    if entries.is_empty() {
        return blocks;
    }

    let mut current_block_start = entries[0].timestamp;
    let mut current_entries = Vec::new();
    let mut total_tokens = 0;

    for entry in entries {
        let hours_diff = (entry.timestamp - current_block_start).num_hours();
        
        if hours_diff >= 5 {
            // Close current block
            if !current_entries.is_empty() {
                let end_time = current_entries.last().map(|e: &ClaudeBarUsageEntry| e.timestamp);
                let limit_reached = current_entries.iter().any(|e| e.is_limit_reached);
                blocks.push(UsageBlock {
                    start_time: current_block_start,
                    end_time,
                    entries: current_entries.clone(),
                    assistant_count: current_entries.len(),
                    guessed: false,
                    limit_reached,
                    reset_time: None,
                    total_tokens,
                    unlock_time: end_time.map(|t| t + chrono::Duration::hours(5)),
                });
            }
            
            // Start new block
            current_block_start = entry.timestamp;
            current_entries.clear();
            total_tokens = 0;
        }
        
        current_entries.push(entry.clone());
        total_tokens += entry.usage.output_tokens;
    }

    // Close final block
    if !current_entries.is_empty() {
        let end_time = current_entries.last().map(|e: &ClaudeBarUsageEntry| e.timestamp);
        let limit_reached = current_entries.iter().any(|e| e.is_limit_reached);
        blocks.push(UsageBlock {
            start_time: current_block_start,
            end_time,
            entries: current_entries.clone(),
            assistant_count: current_entries.len(),
            guessed: false,
            limit_reached,
            reset_time: None,
            total_tokens,
            unlock_time: end_time.map(|t| t + chrono::Duration::hours(5)),
        });
    }

    blocks
}


fn print_limits_debug(all_entries: &[ClaudeBarUsageEntry]) {
    println!(
        "{bold}{purple}🔍 DEBUG: Limit Messages{reset}",
        bold = { BOLD },
        purple = { PURPLE },
        reset = { RESET },
    );
    println!();

    let limit_entries: Vec<&ClaudeBarUsageEntry> =
        all_entries.iter().filter(|e| e.is_limit_reached).collect();

    if limit_entries.is_empty() {
        println!("❌ No limit messages found");
        return;
    }

    for entry in &limit_entries {
        let path = format!("{}/{}", entry.file_info.folder_name, entry.file_info.file_name);

        println!(
            "{} | {}",
            entry.timestamp.format("%Y-%m-%d %H:%M UTC"),
            path
        );
        if let Some(text) = &entry.content_text {
            println!("  {}", text.trim());
        }
    }

    println!(
        "{green}✅ Found {} limit messages{reset}",
        limit_entries.len(),
        green = { GREEN },
        reset = { RESET },
    );
}

fn print_blocks_debug(blocks: &[UsageBlock], _all_entries: &[ClaudeBarUsageEntry]) {
    println!("{bold}{cyan}🔍 DEBUG: FIXED 5-Hour Windows Analysis{reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
    );
    println!();
    
    let fixed_blocks: Vec<&UsageBlock> = blocks.iter()
        .filter(|b| b.limit_reached && !b.guessed)
        .collect();

    if fixed_blocks.is_empty() {
        println!("❌ No fixed 5-hour windows found (no limit reached entries)");
        return;
    }

    println!("Found {} FIXED 5-hour windows:", fixed_blocks.len());
    println!();

    for (i, block) in fixed_blocks.iter().enumerate() {
        println!("{bold}Window {} - {} to {}{reset}",
            i + 1,
            block.start_time.format("%m-%d %H:%M"),
            block.end_time.unwrap_or(chrono::Utc::now()).format("%m-%d %H:%M"),
            bold = { BOLD },
            reset = { RESET },
        );
        
        println!("  Total entries: {}", block.entries.len());
        println!("  Total tokens: {}", format_number_with_separators(block.total_tokens));
        println!();
    }

    println!("{green}✅ Found {} FIXED windows with confirmed limits{reset}",
        fixed_blocks.len(),
        green = { GREEN },
        reset = { RESET },
    );
}

fn print_gaps_debug(blocks: &[UsageBlock]) {
    println!("{bold}{yellow}🕳️  DEBUG: Gap Analysis (Sessions){reset}",
        bold = { BOLD },
        yellow = { YELLOW },
        reset = { RESET },
    );
    println!();
    
    let session_blocks: Vec<&UsageBlock> = blocks.iter()
        .filter(|b| b.guessed && !b.limit_reached)
        .collect();
        
    if session_blocks.is_empty() {
        println!("❌ No session gaps found (no entries with >1 hour gaps)");
        return;
    }
    
    // Create table using TableCreator
    let headers = vec![
        HeaderInfo::new("Session Start", 19),
        HeaderInfo::new("Session End", 19),
        HeaderInfo::new("Duration", 10),
        HeaderInfo::new("Entries", 17),
        HeaderInfo::new("Status", 12),
    ];
    let mut tc = TableCreator::new(headers);
    
    for block in &session_blocks {
        let end_str = if let Some(end) = block.end_time {
            format_date(end, 19)
        } else {
            "Ongoing".to_string()
        };
        
        let duration = if let Some(end) = block.end_time {
            let dur = end - block.start_time;
            format_duration(dur, 10)
        } else {
            let dur = chrono::Utc::now() - block.start_time;
            format!("{}+", format_duration(dur, 9))
        };
        
        let status_colored = if block.end_time.is_none() {
            format!("{green}Active{reset}", green = GREEN, reset = RESET)
        } else {
            format!("{gray}Complete{reset}", gray = GRAY, reset = RESET)
        };
        
        tc.add_row(vec![
            format_date(block.start_time, 19),
            end_str,
            duration,
            format!("{:>7}", block.entries.len()),
            status_colored,
        ]);
    }
    
    tc.display(false);
    
    println!();
    println!("{yellow}📝 Note: These are estimated session boundaries based on gaps > 1 hour{reset}",
        yellow =  { YELLOW },
        reset = { RESET },
    );
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
        HeaderInfo::new("Block", 11),
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

fn run_files_debug(cache_manager: &mut CacheManager, base_path: &str) {
    println!(
        "{bold}{cyan}📊 Files Debug (Cache-based){reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
    );
    println!();

    // Use existing cache from main.rs - no refresh needed in CLI
    let cache_info = cache_manager.get_cache();
    
    if cache_info.folders.is_empty() {
        println!("❌ No folders found in cache for {}", base_path);
        return;
    }

    println!("📁 Found {} project folders in cache:", cache_info.folders.len());
    println!();

    // Print each folder as a separate table (the cool way!)
    let mut total_files = 0;
    let mut total_fresh = 0;
    let mut total_refresh = 0;
    let mut total_not_in_cache = 0;
    let mut total_size = 0u64;

    for (folder_name, cached_folder) in &cache_info.folders {
        println!("{bold}📁 {}{reset}", 
            folder_name, 
            bold = { BOLD }, 
            reset = { RESET }
        );

        let files_count = cached_folder.files.len();
        let folder_size: u64 = cached_folder.files.values().map(|f| f.size_bytes).sum();
        
        println!("   Files: {} files ({} total)", 
            files_count, 
            format_file_size(folder_size)
        );
        println!();

        // Create stylish table with your custom TableCreator - more compact!
        let headers = vec![
            HeaderInfo::new("File Name", 48),
            HeaderInfo::new("Size", 9),
            HeaderInfo::new("Modified", 19),
            HeaderInfo::new("Created", 19),
            HeaderInfo::new("Cache", 10),
        ];
        let mut tc = TableCreator::new(headers);
        let mut fresh_count = 0;
        let mut refresh_count = 0;
        let mut not_in_cache_count = 0;

        for (file_name, cached_file) in &cached_folder.files {
            // Use existing formatting utils - no manual formatting!
            match cached_file.cache_status {
                crate::cache::CacheStatus::Fresh => fresh_count += 1,
                crate::cache::CacheStatus::NeedsRefresh => refresh_count += 1,
                crate::cache::CacheStatus::NotInCache => not_in_cache_count += 1,
            }

            tc.add_row(vec![
                format_text(file_name, 48), // Right-aligned truncated text
                format_file_size(cached_file.size_bytes), // Already formatted  
                format_date(cached_file.modified_time, 19), // Compact format
                format_date(cached_file.created_time, 19), // Compact format
                format_cache_status(&cached_file.cache_status) // No clone needed!
            ]);
        }

        // Display the beautiful custom table
        tc.display(false);
        println!();

        // Update totals
        total_files += files_count;
        total_fresh += fresh_count;
        total_refresh += refresh_count;
        total_not_in_cache += not_in_cache_count;
        total_size += folder_size;
    }

    // Summary
    println!("{bold}📈 Cache Summary:{reset}", 
        bold = { BOLD },
        reset = { RESET }
    );
    println!("   Total Files: {}", format_number_with_separators(total_files as u32));
    println!("   🟢 Fresh: {}", total_fresh);
    println!("   🟡 Needs Refresh: {}", total_refresh);
    println!("   🔴 Not In Cache: {}", total_not_in_cache);
    println!("   💾 Total Size: {}", format_file_size(total_size));
}
