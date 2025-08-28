use chrono::{DateTime, Utc};
use rs_claude_bar::claude_types::TranscriptEntry;
use rs_claude_bar::colors::*;
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

pub fn run(config: &rs_claude_bar::ConfigInfo, parse_mode: bool) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);

    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    if parse_mode {
        run_parse_debug(&base_path);
    } else {
        // Keep original debug functionality as fallback
        run_original_debug(&base_path);
    }
}

fn run_parse_debug(base_path: &str) {
    println!(
        "{bold}{cyan}🔍 DEBUG: JSONL Parse Analysis{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        cyan = if should_use_colors() { CYAN } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
    );
    println!();

    let path = Path::new(base_path);
    let mut all_file_stats = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();

                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name().to_string_lossy().to_string();
                            let file_path = format!("{}/{}", folder_name, file_name);
                            
                            if let Ok(content) = fs::read_to_string(file.path()) {
                                let stats = parse_file_content(&content);
                                all_file_stats.push((file_path, stats));
                            }
                        }
                    }
                }
            }
        }
    }

    if all_file_stats.is_empty() {
        println!("❌ No JSONL files found in {}", base_path);
        return;
    }

    // Print ANSI table header
    print_table_header();

    // Print each file's stats
    for (file_path, stats) in &all_file_stats {
        print_file_stats(file_path, stats);
    }

    print_table_footer();

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

        match serde_json::from_str::<TranscriptEntry>(line) {
            Ok(entry) => {
                stats.successful_parses += 1;
                
                // Parse timestamp and update bounds
                if let Ok(timestamp) = DateTime::parse_from_rfc3339(&entry.timestamp) {
                    let timestamp_utc = timestamp.with_timezone(&Utc);
                    if stats.min_timestamp.is_none() || timestamp_utc < stats.min_timestamp.unwrap() {
                        stats.min_timestamp = Some(timestamp_utc);
                    }
                    if stats.max_timestamp.is_none() || timestamp_utc > stats.max_timestamp.unwrap() {
                        stats.max_timestamp = Some(timestamp_utc);
                    }
                }

                // Add output tokens if available
                if let Some(ref usage) = entry.message.usage {
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

fn print_table_header() {
    let header_sep = "┌──────────────────────────────────────────────────────────────────────────────────┬───────┬─────────┬────────┬─────────┬──────────────┬──────────────┬──────────┐";
    let header_row = "│ File Path                                                                        │ Total │ Parsed  │ Empty  │ Errors  │ Min Timestamp│ Max Timestamp│ Tokens   │";
    let header_div = "├──────────────────────────────────────────────────────────────────────────────────┼───────┼─────────┼────────┼─────────┼──────────────┼──────────────┼──────────┤";

    println!(
        "{bold}{header_sep}{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        header_sep = header_sep,
        reset = if should_use_colors() { RESET } else { "" },
    );
    
    println!(
        "{bold}{header_row}{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        header_row = header_row,
        reset = if should_use_colors() { RESET } else { "" },
    );
    
    println!(
        "{bold}{header_div}{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        header_div = header_div,
        reset = if should_use_colors() { RESET } else { "" },
    );
}

fn print_file_stats(file_path: &str, stats: &FileParseStats) {
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
    let (color, reset) = if should_use_colors() {
        if success_rate >= 95.0 {
            (GREEN, RESET)
        } else if success_rate >= 80.0 {
            (YELLOW, RESET)
        } else {
            (RED, RESET)
        }
    } else {
        ("", "")
    };

    println!(
        "│ {color}{:<80}{reset} │ {:>5} │ {:>7} │ {:>6} │ {:>7} │ {:>12} │ {:>12} │ {:>8} │",
        truncated_path,
        stats.total_lines,
        stats.successful_parses,
        stats.empty_lines,
        stats.parse_errors,
        min_ts,
        max_ts,
        format_number_with_separators(stats.total_output_tokens),
        color = color,
        reset = reset,
    );
}

fn print_table_footer() {
    let footer = "└──────────────────────────────────────────────────────────────────────────────────┴───────┴─────────┴────────┴─────────┴──────────────┴──────────────┴──────────┘";
    
    println!(
        "{bold}{footer}{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        footer = footer,
        reset = if should_use_colors() { RESET } else { "" },
    );
}

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
        bold = if should_use_colors() { BOLD } else { "" },
        green = if should_use_colors() { GREEN } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
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
            bold = if should_use_colors() { BOLD } else { "" },
            yellow = if should_use_colors() { YELLOW } else { "" },
            reset = if should_use_colors() { RESET } else { "" },
        );
        
        for (file_path, stats) in problematic_files {
            let parseable_lines = stats.total_lines - stats.empty_lines;
            let success_rate = (stats.successful_parses as f64 / parseable_lines as f64) * 100.0;
            println!("   {} - {:.1}% success rate ({} errors)", file_path, success_rate, stats.parse_errors);
        }
    }
}

fn run_original_debug(base_path: &str) {
    println!("=== DEBUG: Original parsing debug ===");
    println!("Base path: {}", base_path);
    
    let path = Path::new(base_path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name();
                println!("\n📁 FOLDER: {:?}", folder_name);

                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name();
                            println!("  📄 FILE: {:?}", file_name);

                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for (line_num, line) in content.lines().enumerate() {
                                    if line.trim().is_empty() {
                                        continue;
                                    }

                                    // Try to parse as TranscriptEntry
                                    match serde_json::from_str::<TranscriptEntry>(line) {
                                        Ok(entry) => {
                                            println!(
                                                "    Line {}: ✅ TranscriptEntry",
                                                line_num + 1
                                            );
                                            println!("      -> Full Object: {:#?}", entry);
                                        }
                                        Err(parse_error) => {
                                            println!("    Line {}: ❌ Failed to parse as TranscriptEntry", line_num + 1);
                                            println!("      -> Parse Error: {}", parse_error);
                                            println!(
                                                "      -> Line content (first 200 chars): {}",
                                                line.chars().take(200).collect::<String>()
                                            );
                                        }
                                    }
                                }
                            } else {
                                println!("  ❌ Could not read file: {:?}", file_name);
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Could not read directory: {}", base_path);
    }
}

fn format_number_with_separators(num: u32) -> String {
    let num_str = num.to_string();
    let mut result = String::new();
    let chars: Vec<char> = num_str.chars().collect();
    
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    
    result
}

