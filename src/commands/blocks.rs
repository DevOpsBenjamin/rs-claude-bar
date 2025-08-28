use chrono::{DateTime, Duration, Utc};
use rs_claude_bar::analyze::{parse_reset_time, calculate_unlock_time};
use rs_claude_bar::{
    claude_types::TranscriptEntry, claudebar_types::ClaudeBarUsageEntry, colors::*,
};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct UsageBlock {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub entries: Vec<ClaudeBarUsageEntry>,
    pub assistant_count: usize,
    pub limit_reached: bool,
    pub reset_time: Option<String>,         // e.g., "10pm", "11pm"
    pub unlock_time: Option<DateTime<Utc>>, // calculated unlock timestamp
    pub guessed: bool,
}

pub fn run(config: &rs_claude_bar::ConfigInfo) {
    let mut updated_config = config.clone();
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);

    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    println!(
        "{bold}{cyan}📊 5-Hour Usage Blocks Analysis{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        cyan = if should_use_colors() { CYAN } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
    );
    println!();

    // Load entries using caching mechanism  
    // If this is first run (no last_limit_date), load all entries
    let use_incremental = config.last_limit_date.is_some();
    let mut all_entries = if use_incremental {
        rs_claude_bar::analyze::load_entries_since(&base_path, config.last_limit_date)
    } else {
        rs_claude_bar::analyze::load_all_entries(&base_path)
    };
    
    if all_entries.is_empty() {
        if use_incremental {
            println!("✅ No new entries since last run.");
            return;
        } else {
            println!("❌ No usage entries found!");
            return;
        }
    }

    // Sort by timestamp (descending for analysis)
    all_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    println!(
        "📈 Loaded {} entries from {} to {}",
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
            .format("%Y-%m-%d %H:%M UTC")
    );
    println!();

    // Find usage blocks
    let mut blocks = analyze_usage_blocks(&all_entries);

    // Sort blocks by start time descending (most recent first)
    blocks.sort_by(|a, b| b.start_time.cmp(&a.start_time));

    println!("🔍 Found {} usage blocks:", blocks.len());
    println!();

    // Display blocks
    for (i, block) in blocks.iter().enumerate() {
        let limit_indicator = if block.limit_reached {
            format!("{red}🔴 LIMIT HIT{reset}", red = RED, reset = RESET)
        } else {
            format!("{green}🟢 ACTIVE{reset}", green = GREEN, reset = RESET)
        };
        let status = if block.guessed {
            format!("{} (guess)", limit_indicator)
        } else {
            limit_indicator
        };

        let end_display = block
            .end_time
            .map(|t| t.format("%Y-%m-%d %H:%M UTC").to_string())
            .unwrap_or_else(|| "ongoing".to_string());

        let duration = block
            .end_time
            .unwrap_or_else(Utc::now)
            .signed_duration_since(block.start_time);
        let duration_str = format_duration_hours(duration);

        println!(
            "Block {}: {} - {}",
            i + 1,
            block.start_time.format("%Y-%m-%d %H:%M UTC"),
            end_display
        );
        println!(
            "  Duration: {} | Assistant messages: {} | Status: {}",
            duration_str, block.assistant_count, status
        );

        // Show reset time and unlock time for limit-reached blocks
        if block.limit_reached {
            if let Some(reset_time) = &block.reset_time {
                print!("  Reset time: {}", reset_time);
                if let Some(unlock_time) = &block.unlock_time {
                    println!(
                        " | Unlocks at: {}",
                        unlock_time.format("%Y-%m-%d %H:%M UTC")
                    );
                } else {
                    println!(" | Unlock time: could not calculate");
                }
            } else {
                println!("  Reset time: not found in limit message");
            }
        }

        println!("  Total entries: {}", block.entries.len());
        println!();
    }
    
    // Update config with the latest block date for next run
    // This needs to be implemented differently as we're not using CurrentBlock here
    // For now, we'll find the latest non-projected block from our analysis
    if let Some(latest_real_block) = blocks.iter()
        .filter(|b| !b.guessed && b.limit_reached)
        .max_by_key(|b| b.end_time.unwrap_or(Utc::now())) {
        updated_config.last_limit_date = latest_real_block.end_time;
        
        // Save updated config
        if let Err(e) = rs_claude_bar::config_manager::save_config(&updated_config) {
            eprintln!("Warning: Could not save updated config: {}", e);
        }
    }
}

fn load_all_entries(base_path: &str) -> Vec<ClaudeBarUsageEntry> {
    let mut usage_entries = Vec::new();
    let path = Path::new(base_path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();

                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name().to_string_lossy().to_string();

                            // Get file modification date
                            let file_date = file
                                .metadata()
                                .ok()
                                .and_then(|meta| meta.modified().ok())
                                .and_then(|time| DateTime::<Utc>::from(time).into());

                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for line in content.lines() {
                                    if line.trim().is_empty() {
                                        continue;
                                    }

                                    if let Ok(transcript) =
                                        serde_json::from_str::<TranscriptEntry>(line)
                                    {
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
                }
            }
        }
    }

    usage_entries
}

fn analyze_usage_blocks(entries: &[ClaudeBarUsageEntry]) -> Vec<UsageBlock> {
    use rs_claude_bar::claudebar_types::UserRole;

    // Consider only assistant messages
    let mut assistant_entries: Vec<ClaudeBarUsageEntry> = entries
        .iter()
        .filter(|e| matches!(e.role, UserRole::Assistant))
        .cloned()
        .collect();
    assistant_entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut blocks = Vec::new();
    let mut current_block_entries: Vec<ClaudeBarUsageEntry> = Vec::new();
    let mut block_start: Option<DateTime<Utc>> = None;
    let mut last_timestamp: Option<DateTime<Utc>> = None;

    for entry in assistant_entries {
        let timestamp = entry.timestamp;

        if let (Some(last), Some(start)) = (last_timestamp, block_start) {
            if timestamp - last > Duration::hours(5) {
                let assistant_count = current_block_entries
                    .iter()
                    .filter(|e| !e.is_limit_reached)
                    .count();
                let end = start + Duration::hours(5);
                blocks.push(UsageBlock {
                    start_time: start,
                    end_time: Some(end),
                    entries: current_block_entries.clone(),
                    assistant_count,
                    limit_reached: false,
                    reset_time: None,
                    unlock_time: None,
                    guessed: true,
                });
                current_block_entries.clear();
                block_start = None;
            }
        }

        if entry.is_limit_reached {
            if block_start.is_none() {
                last_timestamp = Some(timestamp);
                continue;
            }

            current_block_entries.push(entry.clone());
            let assistant_count = current_block_entries
                .iter()
                .filter(|e| !e.is_limit_reached)
                .count();
            let start = entry.timestamp - Duration::hours(5);
            let content_text = get_entry_content_text(&entry);
            let reset_time = parse_reset_time(&content_text);
            let unlock_time = reset_time
                .as_ref()
                .and_then(|rt| calculate_unlock_time(entry.timestamp, rt));

            blocks.push(UsageBlock {
                start_time: start,
                end_time: Some(entry.timestamp),
                entries: current_block_entries.clone(),
                assistant_count,
                limit_reached: true,
                reset_time,
                unlock_time,
                guessed: false,
            });

            current_block_entries.clear();
            block_start = None;
            last_timestamp = None;
            continue;
        }

        if block_start.is_none() {
            block_start = Some(timestamp);
        }

        current_block_entries.push(entry.clone());
        last_timestamp = Some(timestamp);
    }

    if let Some(start) = block_start {
        let assistant_count = current_block_entries
            .iter()
            .filter(|e| !e.is_limit_reached)
            .count();
        blocks.push(UsageBlock {
            start_time: start,
            end_time: None,
            entries: current_block_entries,
            assistant_count,
            limit_reached: false,
            reset_time: None,
            unlock_time: None,
            guessed: false,
        });
    }

    blocks
}

/// Get content text from a ClaudeBarUsageEntry
fn get_entry_content_text(entry: &ClaudeBarUsageEntry) -> String {
    entry.content_text.clone().unwrap_or_default()
}

/// Parse reset time from limit message content
// parse_reset_time and calculate_unlock_time now reused from rs_claude_bar::analyze

fn format_duration_hours(duration: Duration) -> String {
    let total_hours = duration.num_hours();
    let minutes = (duration.num_minutes() % 60).abs();

    if total_hours == 0 {
        format!("{}m", minutes)
    } else {
        format!("{}h {}m", total_hours, minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rs_claude_bar::claudebar_types::{FileInfo, TokenUsage, UserRole};

    fn make_entry(ts: &str, limit: bool) -> ClaudeBarUsageEntry {
        ClaudeBarUsageEntry {
            session_id: String::new(),
            timestamp: DateTime::parse_from_rfc3339(ts)
                .unwrap()
                .with_timezone(&Utc),
            role: UserRole::Assistant,
            usage: TokenUsage::default(),
            content_length: 0,
            is_limit_reached: limit,
            content_text: if limit {
                Some("resets 10pm".into())
            } else {
                None
            },
            file_info: FileInfo {
                folder_name: String::new(),
                file_name: String::new(),
                file_date: None,
            },
        }
    }

    #[test]
    fn analyze_blocks_detects_limits_and_gaps() {
        let entries = vec![
            make_entry("2024-01-01T09:00:00Z", false),
            make_entry("2024-01-01T10:00:00Z", false),
            make_entry("2024-01-01T14:00:00Z", true),
            make_entry("2024-01-01T20:00:00Z", false),
            make_entry("2024-01-01T21:00:00Z", false),
            make_entry("2024-01-02T06:00:00Z", false),
        ];

        let blocks = analyze_usage_blocks(&entries);
        assert_eq!(blocks.len(), 3);

        assert_eq!(
            blocks[0].start_time,
            Utc.with_ymd_and_hms(2024, 1, 1, 9, 0, 0).unwrap()
        );
        assert_eq!(
            blocks[0].end_time.unwrap(),
            Utc.with_ymd_and_hms(2024, 1, 1, 14, 0, 0).unwrap()
        );
        assert!(blocks[0].limit_reached);
        assert!(!blocks[0].guessed);
        assert_eq!(blocks[0].assistant_count, 2);

        assert_eq!(
            blocks[1].start_time,
            Utc.with_ymd_and_hms(2024, 1, 1, 20, 0, 0).unwrap()
        );
        assert_eq!(
            blocks[1].end_time.unwrap(),
            Utc.with_ymd_and_hms(2024, 1, 2, 1, 0, 0).unwrap()
        );
        assert!(!blocks[1].limit_reached);
        assert!(blocks[1].guessed);
        assert_eq!(blocks[1].assistant_count, 2);

        assert_eq!(
            blocks[2].start_time,
            Utc.with_ymd_and_hms(2024, 1, 2, 6, 0, 0).unwrap()
        );
        assert!(blocks[2].end_time.is_none());
        assert!(!blocks[2].limit_reached);
        assert!(!blocks[2].guessed);
        assert_eq!(blocks[2].assistant_count, 1);
    }
}
