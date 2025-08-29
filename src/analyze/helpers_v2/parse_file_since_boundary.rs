use chrono::{DateTime, Utc};
use std::fs;

use crate::{
    claudebar_types::{
        file_info::FileSystemInfo,
        usage_entry::ClaudeBarUsageEntry,
    },
};

use super::parse_line::parse_line;

/// Parse file entries since boundary timestamp (reverse optimization)
pub fn parse_file_since_boundary(
    file: &FileSystemInfo, 
    boundary: Option<DateTime<Utc>>
) -> Vec<ClaudeBarUsageEntry> {
    let mut entries = Vec::new();
    
    let Ok(content) = fs::read_to_string(&file.file_path) else {
        return entries;
    };
    
    match boundary {
        None => {
            // No cache - parse everything normally
            for line in content.lines() {
                if let Some(entry) = parse_line(line, file) {
                    entries.push(entry);
                }
            }
        }
        Some(since) => {
            // Has cache - reverse parse until we hit cached hour boundary
            let mut temp_entries = Vec::new();
            
            for line in content.lines().rev() {
                if let Some(entry) = parse_line(line, file) {
                    if entry.timestamp <= since {
                        break; // Stop - this hour is already cached
                    }
                    temp_entries.push(entry);
                }
            }
            
            // Reverse back to chronological order
            temp_entries.reverse();
            entries = temp_entries;
        }
    }
    
    entries
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
    fn test_parse_nonexistent_file() {
        let file = create_test_file_info();
        let result = parse_file_since_boundary(&file, None);
        assert!(result.is_empty());
    }
}