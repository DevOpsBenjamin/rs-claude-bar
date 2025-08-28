use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use rs_claude_bar::{claude_types::TranscriptEntry, claudebar_types::ClaudeBarUsageEntry};
use std::fs;
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

    let mut limit_entries: Vec<ClaudeBarUsageEntry> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();

                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name().to_string_lossy().to_string();

                            // file modification date (optional)
                            let file_date = file
                                .metadata()
                                .ok()
                                .and_then(|meta| meta.modified().ok())
                                .map(DateTime::<Utc>::from);

                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for line in content.lines() {
                                    let line = line.trim();
                                    if line.is_empty() {
                                        continue;
                                    }
                                    // Fast path: only consider lines that likely contain limit text
                                    if !line.contains("5-hour limit reached") {
                                        continue;
                                    }
                                    if let Ok(transcript) = serde_json::from_str::<TranscriptEntry>(line) {
                                        let entry = ClaudeBarUsageEntry::from_transcript(
                                            &transcript,
                                            folder_name.clone(),
                                            file_name.clone(),
                                            file_date,
                                        );
                                        if entry.is_limit_reached {
                                            limit_entries.push(entry);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if limit_entries.is_empty() {
        println!("No limit messages found.");
        return;
    }

    // Sort by timestamp descending (most recent first)
    limit_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    for e in limit_entries {
        let ts = e.timestamp;
        let content = e.content_text.as_deref().unwrap_or("");
        if let Some(reset_time) = parse_reset_time(content) {
            if let Some(unlock) = calculate_unlock_time(ts, &reset_time) {
                let start = unlock - Duration::hours(5);
                println!(
                    "{}|{}|{}|{}",
                    ts.format("%Y-%m-%d %H:%M UTC"),
                    reset_time,
                    unlock.format("%Y-%m-%d %H:%M UTC"),
                    start.format("%Y-%m-%d %H:%M UTC"),
                );
            }
        }
    }
}

/// Parse reset time like "10pm" or "10:30 pm" from content
fn parse_reset_time(content: &str) -> Option<String> {
    // Pattern: "Reset time: 10pm" or "resets 10pm"
    let patterns = [
        r"(?i)reset\s*time:\s*(\d{1,2}(?::\d{2})?\s*(?:am|pm))",
        r"(?i)resets?\s+(?:at\s+)?(\d{1,2}(?::\d{2})?\s*(?:am|pm))",
        r"(?i)(?:until|at)\s+(\d{1,2}(?::\d{2})?\s*(?:am|pm))",
    ];
    for pat in patterns {
        if let Ok(re) = Regex::new(pat) {
            if let Some(caps) = re.captures(content) {
                return Some(caps[1].to_lowercase());
            }
        }
    }
    None
}

/// Calculate unlock time based on limit timestamp and reset time string
fn calculate_unlock_time(limit_timestamp: DateTime<Utc>, reset_time: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"(\d{1,2})(?::(\d{2}))?\s*(am|pm)").ok()?;
    let caps = re.captures(reset_time)?;

    let hour: u32 = caps.get(1)?.as_str().parse().ok()?;
    let minute: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let is_pm = caps.get(3)?.as_str().eq_ignore_ascii_case("pm");

    let hour_24 = match (hour, is_pm) {
        (12, false) => 0,    // 12am -> 0
        (12, true) => 12,    // 12pm -> 12
        (h, false) => h,     // am hours
        (h, true) => h + 12, // pm hours
    };
    if hour_24 >= 24 || minute >= 60 { return None; }

    let limit_date = limit_timestamp.date_naive();
    let same_day = limit_date
        .and_hms_opt(hour_24, minute, 0)?
        .and_local_timezone(Utc)
        .single()?;

    // If reset time already passed at/ before limit timestamp, use next day
    let unlock = if same_day > limit_timestamp { same_day } else {
        (limit_date + chrono::Days::new(1))
            .and_hms_opt(hour_24, minute, 0)?
            .and_local_timezone(Utc)
            .single()?
    };
    Some(unlock)
}
