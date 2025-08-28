use chrono::{DateTime, Duration, Utc};
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

    // Print simple list: "<end> | <start>"
    for e in limit_entries {
        let end = e.timestamp;
        let start = end - Duration::hours(5);
        println!(
            "{} | {}",
            end.format("%Y-%m-%d %H:%M UTC"),
            start.format("%Y-%m-%d %H:%M UTC")
        );
    }
}

