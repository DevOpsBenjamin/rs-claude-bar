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
        assistant_input_tokens: i64,
        projected: bool,
    }

    let mut simple: Vec<SimpleBlock> = Vec::new();
    for b in &current_blocks {
        if let (Some(s), Some(e)) = (b.start, b.end) {
            let is_projected = b.reset == "projected";
            let out = b.assistant.output_tokens;
            let inn = b.assistant.input_tokens;
            if out == 0 && inn == 0 && !is_projected {
                continue; // skip zero-token (in+out) non-projected blocks
            }
            simple.push(SimpleBlock { start: s, end: e, assistant_output_tokens: out, assistant_input_tokens: inn, projected: is_projected });
        }
    }

    // Keep only the last 10 blocks, sorted old -> recent (projected likely last)
    let mut simple_sorted = simple.clone();
    simple_sorted.sort_by_key(|b| b.start);
    if simple_sorted.len() > 10 {
        let start = simple_sorted.len() - 10;
        simple_sorted = simple_sorted[start..].to_vec();
    }
    println!("SimpleBlocks: {:#?}", simple_sorted);

    // Slot stats: 0-6, 6-12, 12-18, 18-24; if a 5h block crosses a boundary, count it in both slots
    #[derive(Debug, Default, Clone)]
    struct MetricStat { count: usize, mean: f64, min: i64, max: i64 }
    #[derive(Debug, Default, Clone)]
    struct BandStats { input: MetricStat, output: MetricStat, total: MetricStat }

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
    let mut slot_values_in: HashMap<&'static str, Vec<i64>> = HashMap::new();
    let mut slot_values_out: HashMap<&'static str, Vec<i64>> = HashMap::new();
    let mut slot_values_total: HashMap<&'static str, Vec<i64>> = HashMap::new();
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
            slot_values_in.entry(lab).or_default().push(sb.assistant_input_tokens);
            slot_values_out.entry(lab).or_default().push(sb.assistant_output_tokens);
            slot_values_total.entry(lab).or_default().push(sb.assistant_input_tokens + sb.assistant_output_tokens);
        }
    }
    
    // Collect union of labels
    let labels = ["0-6","6-12","12-18","18-24"];
    let mut stats: HashMap<&'static str, BandStats> = HashMap::new();
    for &lab in &labels {
        let ins = slot_values_in.remove(lab).unwrap_or_default();
        let outs = slot_values_out.remove(lab).unwrap_or_default();
        let tots = slot_values_total.remove(lab).unwrap_or_default();
        let in_count = ins.len();
        let out_count = outs.len();
        let tot_count = tots.len();
        let in_mean = if in_count>0 { ins.iter().map(|v| *v as i128).sum::<i128>() as f64 / in_count as f64 } else {0.0};
        let out_mean = if out_count>0 { outs.iter().map(|v| *v as i128).sum::<i128>() as f64 / out_count as f64 } else {0.0};
        let tot_mean = if tot_count>0 { tots.iter().map(|v| *v as i128).sum::<i128>() as f64 / tot_count as f64 } else {0.0};
        let in_min = ins.iter().min().copied().unwrap_or(0);
        let in_max = ins.iter().max().copied().unwrap_or(0);
        let out_min = outs.iter().min().copied().unwrap_or(0);
        let out_max = outs.iter().max().copied().unwrap_or(0);
        let tot_min = tots.iter().min().copied().unwrap_or(0);
        let tot_max = tots.iter().max().copied().unwrap_or(0);
        stats.insert(lab, BandStats {
            input:  MetricStat { count: in_count,  mean: in_mean,  min: in_min,  max: in_max },
            output: MetricStat { count: out_count, mean: out_mean, min: out_min, max: out_max },
            total:  MetricStat { count: tot_count, mean: tot_mean, min: tot_min, max: tot_max },
        });
    }
    println!("SlotStats: {:#?}", stats);
}
fn print_guessblocks_debug(rows: &Vec<GuessBlock>) {
    println!("GuessBlocks: {:#?}", rows);
}

fn print_currentblocks_debug(blocks: &Vec<CurrentBlock>) {
    println!("CurrentBlocks: {:#?}", blocks);
}

// shared helpers are imported from rs_claude_bar::analyze
