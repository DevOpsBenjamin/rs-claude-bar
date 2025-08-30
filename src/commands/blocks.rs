use crate::{
    analyze::Analyzer,
    display::table::TableCreator,
    claudebar_types::{
        config::ConfigInfo,
        display::HeaderInfo,
    },
    common::colors::*,
};

/// Minimal `blocks` implementation: use Analyzer’s limit blocks and display last 10.
pub fn run(_config: &ConfigInfo, analyzer: &Analyzer) {
    println!(
        "{bold}{cyan}📊 5-Hour Usage Blocks (limits){reset}",
        bold = { BOLD }, cyan = { CYAN }, reset = { RESET }
    );

    // Get all limit blocks, sorted desc by unlock; take the 10 most recent
    let blocks = analyzer.limit_blocks_all();
    let top10 = blocks.into_iter().take(10).collect::<Vec<_>>();

    // Table: Start | End | Duration | Tokens | Messages | Status (most recent first)
    let headers = vec![
        HeaderInfo::new("Start", 19),
        HeaderInfo::new("End", 19),
        HeaderInfo::new("Duration", 10),
        HeaderInfo::new("Tokens", 9),
        HeaderInfo::new("Messages", 12),
        HeaderInfo::new("Status", 9),
    ];
    let mut tc = TableCreator::new(headers);
    for (start, lb) in top10 {
        let end = lb.unlock_timestamp;
        let duration = end.signed_duration_since(start);
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() - hours * 60;
        let duration_str = format!("{}h {:02}m", hours, minutes);

        let tokens = lb.datas.total_tokens;
        let messages = lb.datas.assistant_messages + lb.datas.user_messages;
        let status = "Complete"; // Limit-based windows are completed

        tc.add_row(vec![
            format!("{}", start.format("%Y-%m-%d %H:%M UTC")),
            format!("{}", end.format("%Y-%m-%d %H:%M UTC")),
            format!("{:>10}", duration_str),
            format!("{:>9}", tokens),
            format!("{:>12}", messages),
            format!("{:>9}", status),
        ]);
    }
    tc.display(false);

    /*
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
    */
}


/*
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
        HeaderInfo::new("Window Start", 14),
        HeaderInfo::new("Window End", 14),
        HeaderInfo::new("Reset", 7),
        HeaderInfo::new("First Activity", 14),
        HeaderInfo::new("Last Activity", 14),
        HeaderInfo::new("Count", 5),
        HeaderInfo::new("Tokens", 9),
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
        HeaderInfo::new("Session Start", 19),
        HeaderInfo::new("Session End", 19),
        HeaderInfo::new("Duration", 8),
        HeaderInfo::new("Entries", 7),
        HeaderInfo::new("Status", 10),
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
*/
