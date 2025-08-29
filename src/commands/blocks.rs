
use std::path::Path;
use chrono::{DateTime, Duration, Utc};

use crate::{
    analyze::{
        parse_reset_time, 
        calculate_unlock_time,
        load_entries_with_cache
    },
    utils::formatting::{
        format_date,
        format_duration,
        format_token_count
    },
    display::table::TableCreator,
    commands::shared_types::UsageBlock,
    claudebar_types::{
        usage_entry::{ClaudeBarUsageEntry, UserRole},
        config::ConfigInfo,
        display::{
            HeaderInfo,
        }
    },
    common::colors::*,
};

pub fn run(config: &ConfigInfo) {
    let mut updated_config = config.clone();
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);

    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    println!(
        "{bold}{cyan}📊 5-Hour Usage Blocks Analysis{reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
    );
    println!();

    // Load ALL entries with caching (blocks analysis needs historical data)
    let (mut all_entries, load_duration) = load_entries_with_cache(&base_path);
    
    if all_entries.is_empty() {
        println!("❌ No usage entries found in {}!", base_path);
        return;
    }

    // Sort by timestamp (descending for analysis)
    all_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    println!(
        "📈 Loaded {} entries from {} to {} (took {:.1}ms)",
        all_entries.len(),
        all_entries
            .last()
            .unwrap()
            .timestamp
            .format("%Y-%m-%d %H:%M UTC"),
        all_entries
            .first()
            .unwrap()
            .timestamp
            .format("%Y-%m-%d %H:%M UTC"),
        load_duration.as_secs_f64() * 1000.0
    );
    println!();

    // Find usage blocks
    let analysis_start = std::time::Instant::now();
    let mut blocks = analyze_usage_blocks(&all_entries);
    let analysis_duration = analysis_start.elapsed();
    
    // Sort blocks by start time descending (most recent first)
    blocks.sort_by(|a, b| b.start_time.cmp(&a.start_time));


    // Get last 10 fixed blocks (limit reached, not guessed)
    let fixed_blocks: Vec<&UsageBlock> = blocks.iter()
        .filter(|b| b.limit_reached && !b.guessed)
        .collect();
    // Get current ongoing block (not limit reached, no end time)
    let current_block = blocks.iter()
        .find(|b| !b.limit_reached && b.end_time.is_none());

    println!("🔍 Found {} fixed blocks, showing last 10 + current (analysis took {:.1}ms):", 
            fixed_blocks.len(), 
            analysis_duration.as_secs_f64() * 1000.0);
    println!();
    
    //Table using Helper
    let headers = vec![
        HeaderInfo { label: "Type", width: 6 },
        HeaderInfo { label: "Start", width: 13 },
        HeaderInfo { label: "End", width: 13 },
        HeaderInfo { label: "Duration", width: 10 },
        HeaderInfo { label: "Tokens", width: 9 },
        HeaderInfo { label: "Messages", width: 12 },
        HeaderInfo { label: "Status", width: 9 },
    ];
    let mut tc = TableCreator::new(headers);

    // Fixed blocks
    let mut display_blocks: Vec<&UsageBlock> = fixed_blocks.iter().take(10).copied().collect();
    display_blocks.reverse();
    
    for block in display_blocks.iter() {
        let end_time = block
            .end_time
            .unwrap_or_else(Utc::now);
        let duration = end_time.signed_duration_since(block.start_time);
        let total_tokens: u32 = block.entries.iter().map(|e| e.usage.output_tokens).sum();
        
        // reset format
        let reset_time = block.reset_time.as_deref().unwrap_or("?");
        let mut type_display = format!("{:>6}", "PAST");
        let mut status = format!("🔴 {:>4}", reset_time);
        if end_time> Utc::now() {
            type_display = format!("{:>6}", "NOW");
            status = format!("🟢 {:>4}", reset_time);
        }

        tc.add_row(vec![
            type_display,
            format_date(block.start_time, 11),
            format_date(end_time, 11),
            format_duration(duration, 8),
            format_token_count(total_tokens, 7),
            format!("{:>10}", block.assistant_count.to_string()),
            status
        ]);
    }
    tc.display(false);

    // Show current session estimate first
    if let Some(current) = current_block {
        let total_tokens: u32 = current.entries.iter()
            .map(|e| e.usage.output_tokens)
            .sum();
        let session_duration = chrono::Utc::now() - current.start_time;
        let duration_str = format_duration(session_duration, 8);
        
        // Calculate estimated end time (5 hours from start)
        let estimated_end = current.start_time + Duration::hours(5);
        let time_remaining = estimated_end - chrono::Utc::now();
        let remaining_str = if time_remaining.num_seconds() > 0 {
            format!("{} left", format_duration(time_remaining, 8))
        } else {
            "overtime".to_string()
        };
        
        let est_limit = 25000;
        let percentage = (total_tokens as f64 / est_limit as f64) * 100.0;
        let status = if percentage > 70.0 {
            format!("⚠️ {:>3.0}%", percentage)
        } else {
            format!("🟢 {:>3.0}%", percentage)
        };

        println!("│ {:<4} │ {:<11} │ {:<11} │ {:>8} │ {:>7} │ {:>10} │ {:>6} │",
            "NOW",
            current.start_time.format("%m-%d %H:%M"),
            remaining_str,
            duration_str,
            total_tokens,
            current.assistant_count,
            { &status }
        );
    }
    
    // Update config with the latest block date for next run
    // This needs to be implemented differently as we're not using CurrentBlock here
    // For now, we'll find the latest non-projected block from our analysis
    if let Some(latest_real_block) = blocks.iter()
        .filter(|b| !b.guessed && b.limit_reached)
        .max_by_key(|b| b.end_time.unwrap_or(Utc::now())) {
        updated_config.last_limit_date = latest_real_block.end_time;
        
        // Save updated config
        if let Err(e) = crate::config_manager::save_config(&updated_config) {
            eprintln!("Warning: Could not save updated config: {}", e);
        }
    }
}

/// Print all limit messages with their timestamps and file paths
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

    // STEP 1: Find FIXED 5-hour windows from limit messages
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

/// Get content text from a ClaudeBarUsageEntry
fn get_entry_content_text(entry: &ClaudeBarUsageEntry) -> String {
    entry.content_text.clone().unwrap_or_default()
}

/// Calculate FIXED 5-hour window from reset time
/// Reset time (e.g., "5pm") defines the END of the 5-hour window
/// So "reset 5pm" means window was 12pm-5pm (12:00-17:00)
fn calculate_fixed_window_from_reset(limit_timestamp: DateTime<Utc>, reset_time: &str) -> (DateTime<Utc>, DateTime<Utc>) {
    // Parse reset time to get the hour
    let reset_hour = match reset_time.to_lowercase().as_str() {
        "12am" | "midnight" => 0,
        "1am" => 1, "2am" => 2, "3am" => 3, "4am" => 4, "5am" => 5,
        "6am" => 6, "7am" => 7, "8am" => 8, "9am" => 9, "10am" => 10, "11am" => 11,
        "12pm" | "noon" => 12,
        "1pm" => 13, "2pm" => 14, "3pm" => 15, "4pm" => 16, "5pm" => 17,
        "6pm" => 18, "7pm" => 19, "8pm" => 20, "9pm" => 21, "10pm" => 22, "11pm" => 23,
        _ => {
            // Try to extract number + am/pm pattern
            if let Some(hour_str) = reset_time.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse::<u32>().ok() {
                if reset_time.to_lowercase().contains("pm") && hour_str != 12 {
                    (hour_str + 12) % 24
                } else if reset_time.to_lowercase().contains("am") && hour_str == 12 {
                    0
                } else {
                    hour_str % 24
                }
            } else {
                17 // Default to 5pm
            }
        }
    };

    // The key insight: when we hit a limit and get "reset 5pm", 
    // it means the window that JUST ENDED was ending at 5pm
    // So we want the window that ended at the most recent reset_hour that's <= limit_timestamp
    
    let limit_date = limit_timestamp.date_naive();
    
    // Try reset time on the same day first
    let same_day_reset = limit_date.and_hms_opt(reset_hour as u32, 0, 0)
        .unwrap_or_else(|| limit_date.and_hms_opt(17, 0, 0).unwrap())
        .and_utc();
    
    let window_end = if limit_timestamp >= same_day_reset {
        // Limit was hit on or after today's reset time, so yesterday's window ended at today's reset
        same_day_reset - Duration::days(1)
    } else {
        // Limit was hit before today's reset time, so today's window is ending at today's reset
        same_day_reset
    };
    
    // Window START is 5 hours before window end
    let window_start = window_end - Duration::hours(5);
    
    (window_start, window_end)
}

/// Group entries into sessions based on gaps > 1 hour
fn group_entries_into_sessions(entries: &[ClaudeBarUsageEntry]) -> Vec<Vec<ClaudeBarUsageEntry>> {
    if entries.is_empty() {
        return Vec::new();
    }
    
    let mut sessions = Vec::new();
    let mut current_session = Vec::new();
    let mut last_timestamp: Option<DateTime<Utc>> = None;
    
    for entry in entries {
        if let Some(last) = last_timestamp {
            let gap = entry.timestamp - last;
            
            // If gap > 1 hour, start new session
            if gap > Duration::hours(1) {
                if !current_session.is_empty() {
                    sessions.push(current_session.clone());
                    current_session.clear();
                }
            }
        }
        
        current_session.push(entry.clone());
        last_timestamp = Some(entry.timestamp);
    }
    
    // Add final session
    if !current_session.is_empty() {
        sessions.push(current_session);
    }
    
    sessions
}

/// Print debug information for FIXED blocks (assured time gaps with limits)
fn print_blocks_debug(blocks: &[UsageBlock], all_entries: &[ClaudeBarUsageEntry]) {    
    println!("{bold}{cyan}🔍 DEBUG: FIXED 5-Hour Windows Analysis{reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
    );
    println!();

    // Filter only FIXED blocks (limit reached, not guessed)
    let fixed_blocks: Vec<&UsageBlock> = blocks.iter()
        .filter(|b| b.limit_reached && !b.guessed)
        .collect();

    if fixed_blocks.is_empty() {
        println!("❌ No FIXED windows found (no limit messages with reset times)");
        return;
    }

    // Create table using TableCreator
    let headers = vec![
        HeaderInfo { label: "Window Start", width: 14 },
        HeaderInfo { label: "Window End", width: 14 },
        HeaderInfo { label: "Reset", width: 7 },
        HeaderInfo { label: "First Activity", width: 14 },
        HeaderInfo { label: "Last Activity", width: 14 },
        HeaderInfo { label: "Count", width: 5 },
        HeaderInfo { label: "Tokens", width: 9 },
    ];
    let mut tc = TableCreator::new(headers);

    for block in &fixed_blocks {
        // Find actual activity bounds within the window
        let activity_entries: Vec<&ClaudeBarUsageEntry> = all_entries.iter()
            .filter(|e| matches!(e.role, UserRole::Assistant))
            .filter(|e| e.timestamp >= block.start_time && e.timestamp <= block.end_time.unwrap_or(block.start_time))
            .collect();

        let first_activity = activity_entries.iter()
            .map(|e| e.timestamp)
            .min()
            .map(|t| format_date(t, 14))
            .unwrap_or_else(|| "No activity".to_string());

        let last_activity = activity_entries.iter()
            .map(|e| e.timestamp)
            .max()
            .map(|t| format_date(t, 14))
            .unwrap_or_else(|| "No activity".to_string());

        let reset_time = block.reset_time.as_deref().unwrap_or("Unknown");
        let count = activity_entries.len();
        let total_tokens: u32 = activity_entries.iter()
            .map(|e| e.usage.output_tokens)
            .sum();

        tc.add_row(vec![
            format_date(block.start_time, 14),
            format_date(block.end_time.unwrap(), 14),
            format!("{:>7}", reset_time),
            first_activity,
            last_activity,
            format!("{:>5}", count),
            format_token_count(total_tokens, 9),
        ]);
    }

    tc.display(false);
    println!();
    println!("{green}✅ Found {} FIXED windows with confirmed limits{reset}",
        fixed_blocks.len(),
        green = { GREEN },
        reset = { RESET },
    );
}

/// Print debug information for gap analysis (sessions)
fn print_gaps_debug(blocks: &[UsageBlock]) {
    println!("{bold}{yellow}🕳️  DEBUG: Gap Analysis (Sessions){reset}",
        bold = { BOLD },
        yellow = { YELLOW },
        reset = { RESET },
    );
    println!();

    // Filter only guessed blocks (sessions with gaps > 1 hour)
    let session_blocks: Vec<&UsageBlock> = blocks.iter()
        .filter(|b| b.guessed && !b.limit_reached)
        .collect();

    if session_blocks.is_empty() {
        println!("❌ No session gaps found (no entries with >1 hour gaps)");
        return;
    }

    // Create table using TableCreator  
    let headers = vec![
        HeaderInfo { label: "Session Start", width: 19 },
        HeaderInfo { label: "Session End", width: 19 },
        HeaderInfo { label: "Duration", width: 8 },
        HeaderInfo { label: "Entries", width: 7 },
        HeaderInfo { label: "Status", width: 10 },
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
            format_duration(dur, 8)
        } else {
            let dur = chrono::Utc::now() - block.start_time;
            format_duration(dur, 8)
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
            format!("{:<7}", block.entries.len()),
            status_colored,
        ]);
    }

    tc.display(false);
    println!();
    println!("{yellow}🔍 Found {} usage sessions (gaps >1 hour detected){reset}",
        session_blocks.len(),
        yellow = { YELLOW },
        reset = { RESET },
    );
}