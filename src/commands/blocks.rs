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

    // Get typed limit blocks, sorted desc by end; take the 10 most recent
    let blocks = analyzer.limit_blocks_typed_all();
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
    for b in top10 {
        let start = b.start;
        let end = b.end;
        let duration = end.signed_duration_since(start);
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() - hours * 60;
        let duration_str = format!("{}h {:02}m", hours, minutes);

        let tokens = b.stats.total_tokens;
        let messages = b.stats.assistant_messages + b.stats.user_messages;
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
