use crate::{
    claude_types::transcript_entry::TranscriptEntry,
    claudebar_types::{
        file_info::FileSystemInfo,
        usage_entry::ClaudeBarUsageEntry,
    },
};

/* 
/// Parse single JSONL line into ClaudeBarUsageEntry
pub fn parse_line(line: &str, file: &FileSystemInfo) -> Option<ClaudeBarUsageEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    
    if let Ok(transcript) = serde_json::from_str::<TranscriptEntry>(line) {
        Some(ClaudeBarUsageEntry::from_transcript(
            &transcript,
            file.folder_name.clone(),
            file.file_name.clone(),
            Some(file.modified_time),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    fn create_test_file_info() -> FileSystemInfo {
        FileSystemInfo {
            folder_name: "test-folder".to_string(),
            file_name: "test-file.jsonl".to_string(),
            file_path: "/test/path".to_string(),
            size_bytes: 1024,
            modified_time: Utc::now(),
            created_time: Utc::now(),
            exists: true,
        }
    }

    #[test]
    fn test_parse_empty_line() {
        let file = create_test_file_info();
        assert!(parse_line("", &file).is_none());
        assert!(parse_line("   ", &file).is_none());
    }
    
    #[test]
    fn test_parse_invalid_json() {
        let file = create_test_file_info();
        assert!(parse_line("invalid json", &file).is_none());
    }
}*/