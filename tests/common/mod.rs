use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

use rs_claude_bar::{
    claude_types::transcript_entry::TranscriptEntry,
    claudebar_types::usage_entry::ClaudeBarUsageEntry
};

/// Load entries from a test data directory
pub fn load_test_entries(data_path: &str) -> Vec<ClaudeBarUsageEntry> {
    let path = Path::new(data_path);
    if !path.exists() {
        return Vec::new();
    }

    let mut usage_entries = Vec::new();
    let Ok(dir_entries) = fs::read_dir(path) else {
        return usage_entries;
    };

    for entry in dir_entries.flatten().filter(|e| e.path().is_dir()) {
        let folder_name = entry.file_name().to_string_lossy().to_string();
        process_folder(&entry.path(), &folder_name, &mut usage_entries);
    }

    usage_entries
}

fn process_folder(
    folder_path: &Path,
    folder_name: &str,
    usage_entries: &mut Vec<ClaudeBarUsageEntry>,
) {
    let Ok(files) = fs::read_dir(folder_path) else {
        return;
    };

    for file in files.flatten().filter(is_jsonl_file) {
        let file_name = file.file_name().to_string_lossy().to_string();
        let file_date = get_file_date(&file);

        process_jsonl_file(
            &file.path(),
            folder_name,
            &file_name,
            file_date,
            usage_entries,
        );
    }
}

fn is_jsonl_file(file: &fs::DirEntry) -> bool {
    file.path().extension().and_then(|s| s.to_str()) == Some("jsonl")
}

fn get_file_date(file: &fs::DirEntry) -> Option<DateTime<Utc>> {
    Some(file.metadata().ok()?.modified().ok()?.into())
}

fn process_jsonl_file(
    file_path: &Path,
    folder_name: &str,
    file_name: &str,
    file_date: Option<DateTime<Utc>>,
    usage_entries: &mut Vec<ClaudeBarUsageEntry>,
) {
    let Ok(content) = fs::read_to_string(file_path) else {
        return;
    };

    for line in content.lines().filter(|line| !line.trim().is_empty()) {
        if let Ok(transcript) = serde_json::from_str::<TranscriptEntry>(line) {
            let usage_entry = ClaudeBarUsageEntry::from_transcript(
                &transcript,
                folder_name.to_string(),
                file_name.to_string(),
                file_date,
            );
            usage_entries.push(usage_entry);
        }
    }
}
