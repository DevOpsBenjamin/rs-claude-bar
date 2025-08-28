use chrono::{DateTime, Utc};
use regex::Regex;
use std::{fs, path::Path};

use crate::{claude_types::TranscriptEntry, claudebar_types::ClaudeBarUsageEntry};

/// Public: load every entry from `~/.claude/projects`-style path
pub fn load_all_entries(base_path: &str) -> Vec<ClaudeBarUsageEntry> {
    let mut usage_entries = Vec::new();
    let projects = Path::new(base_path);
    if !projects.exists() {
        return usage_entries;
    }

    if let Ok(entries) = fs::read_dir(projects) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name().to_string_lossy().to_string();
                            let file_date = file
                                .metadata()
                                .ok()
                                .and_then(|m| m.modified().ok())
                                .map(DateTime::<Utc>::from);
                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for line in content.lines() {
                                    let line = line.trim();
                                    if line.is_empty() {
                                        continue;
                                    }
                                    if let Ok(transcript) = serde_json::from_str::<TranscriptEntry>(line) {
                                        let entry = ClaudeBarUsageEntry::from_transcript(
                                            &transcript,
                                            folder_name.clone(),
                                            file_name.clone(),
                                            file_date,
                                        );
                                        usage_entries.push(entry);
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

/// Public: parse reset time (e.g. "10pm")
pub fn parse_reset_time(content: &str) -> Option<String> {
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

/// Public: compute unlock time from a message timestamp and reset time string
pub fn calculate_unlock_time(limit_timestamp: DateTime<Utc>, reset_time: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"(\d{1,2})(?::(\d{2}))?\s*(am|pm)").ok()?;
    let caps = re.captures(reset_time)?;

    let hour: u32 = caps.get(1)?.as_str().parse().ok()?;
    let minute: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let is_pm = caps.get(3)?.as_str().eq_ignore_ascii_case("pm");

    let hour_24 = match (hour, is_pm) {
        (12, false) => 0,
        (12, true) => 12,
        (h, false) => h,
        (h, true) => h + 12,
    };
    if hour_24 >= 24 || minute >= 60 {
        return None;
    }

    let limit_date = limit_timestamp.date_naive();
    let same_day = limit_date
        .and_hms_opt(hour_24, minute, 0)?
        .and_local_timezone(Utc)
        .single()?;

    let unlock = if same_day > limit_timestamp {
        same_day
    } else {
        (limit_date + chrono::Days::new(1))
            .and_hms_opt(hour_24, minute, 0)?
            .and_local_timezone(Utc)
            .single()?
    };
    Some(unlock)
}

