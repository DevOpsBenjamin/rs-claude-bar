use chrono::{Utc, DateTime, Timelike, Duration};
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

    // For debug: remove zero-activity GAP blocks only; keep all real blocks, including projected
    let filtered_debug: Vec<CurrentBlock> = current_blocks
        .iter()
        .cloned()
        .filter(|b| {
            let is_gap = b.start.is_none() && b.end.is_none();
            if is_gap {
                b.assistant.total_tokens > 0 || b.assistant.content > 0 || b.user.content > 0
            } else {
                true
            }
        })
        .collect();

    // Print CurrentBlocks via debug print (readable)
    print_currentblocks_debug(&filtered_debug);

    // Build a simple array of non-gap blocks: start, end, assistant.output_tokens
    #[derive(Debug, Clone)]
    struct SimpleBlock {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        assistant_output_tokens: i64,
        projected: bool,
    }

    let mut simple: Vec<SimpleBlock> = Vec::new();
    for b in &current_blocks {
        if let (Some(s), Some(e)) = (b.start, b.end) {
            let is_projected = b.reset == "projected";
            let out = b.assistant.output_tokens;
            if out == 0 && !is_projected {
                continue; // skip zero-token non-projected blocks
            }
            simple.push(SimpleBlock { start: s, end: e, assistant_output_tokens: out, projected: is_projected });
        }
    }

    println!("SimpleBlocks: {:#?}", simple);

    // Slot stats: 0-6, 6-12, 12-18, 18-24; if a 5h block crosses a boundary, count it in both slots
    #[derive(Debug, Default, Clone)]
    struct SlotStat { count: usize, sum: i128, mean: f64, median: f64, ema: f64 }

    fn slot_label(hour: u32) -> &'static str {
        match hour {
            0..=5 => "0-6",
            6..=11 => "6-12",
            12..=17 => "12-18",
            _ => "18-24",
        }
    }

    fn next_boundary_after(dt: DateTime<Utc>) -> DateTime<Utc> {
        let h = dt.hour();
        // boundaries at 6,12,18,24
        let next_h = if h < 6 { 6 } else if h < 12 { 12 } else if h < 18 { 18 } else { 24 };
        let day = dt.date_naive();
        let base = day.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).single().unwrap();
        let mut boundary = base + Duration::hours(next_h as i64);
        if boundary <= dt { boundary += Duration::hours(24); }
        boundary
    }

    use std::collections::HashMap;
    let mut slot_values: HashMap<&'static str, Vec<(DateTime<Utc>, i64)>> = HashMap::new();
    for sb in &simple {
        if sb.projected { continue; } // don't include projected in stats
        let start = sb.start;
        let end = sb.end;
        let mut labels = vec![slot_label(start.hour())];
        let boundary = next_boundary_after(start);
        if boundary < end {
            labels.push(slot_label((boundary.hour()) % 24));
        }
        for lab in labels {
            slot_values.entry(lab).or_default().push((end, sb.assistant_output_tokens));
        }
    }
    let mut slot_stats: HashMap<&'static str, SlotStat> = HashMap::new();
    
    fn compute_median(mut xs: Vec<i64>) -> f64 {
        if xs.is_empty() { return 0.0; }
        xs.sort_unstable();
        let n = xs.len();
        if n % 2 == 1 { xs[n/2] as f64 } else { (xs[n/2 - 1] as f64 + xs[n/2] as f64) / 2.0 }
    }
    fn compute_ema(mut pairs: Vec<(DateTime<Utc>, i64)>) -> f64 {
        if pairs.is_empty() { return 0.0; }
        pairs.sort_by_key(|p| p.0); // chronological
        let n = pairs.len() as f64;
        let alpha = 2.0 / (n + 1.0);
        let mut ema = pairs[0].1 as f64;
        for &(_, v) in pairs.iter().skip(1) {
            ema = alpha * (v as f64) + (1.0 - alpha) * ema;
        }
        ema
    }

    for (lab, vals) in slot_values.into_iter() {
        let count = vals.len();
        let sum: i128 = vals.iter().map(|(_, v)| *v as i128).sum();
        let mean = if count > 0 { sum as f64 / count as f64 } else { 0.0 };
        let median = compute_median(vals.iter().map(|(_, v)| *v).collect());
        let ema = compute_ema(vals);
        slot_stats.insert(lab, SlotStat { count, sum, mean, median, ema });
    }
    println!("SlotStats: {:#?}", slot_stats);
}
fn print_guessblocks_debug(rows: &Vec<GuessBlock>) {
    println!("GuessBlocks: {:#?}", rows);
}

fn print_currentblocks_debug(blocks: &Vec<CurrentBlock>) {
    println!("CurrentBlocks: {:#?}", blocks);
}

// shared helpers are imported from rs_claude_bar::analyze
