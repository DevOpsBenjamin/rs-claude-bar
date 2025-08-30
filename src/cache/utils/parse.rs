use std::{collections::{HashMap, hash_map::Entry}, fs, path::{Path, PathBuf}};
use chrono::{DateTime, Utc};

use crate::{cache::{CacheInfo, CacheStatus, CachedFile, CachedFolder}, claude_types::transcript_entry::ClaudeEntry};

/// Refresh a single file by updating its cache status to Fresh
/// This simulates processing the file content and updating cache data
fn refresh_single_file(file: &mut CachedFile, file_path: &PathBuf) {
        /*
        // Verify file exists and is readable
        if !std::path::Path::new(file_path).exists() {
            return Err(format!("File does not exist: {}", file_path).into());
        }
        
        if let Some(cached_folder) = self.cache.folders.get_mut(folder_name) {
            if let Some(cached_file) = cached_folder.files.get_mut(file_name) {
                // Get current file metadata for accurate timestamp
                let metadata = fs::metadata(file_path)?;
                let modified_time = metadata.modified()?;
                let modified_time_utc = chrono::DateTime::<chrono::Utc>::from(modified_time);
                
                // Update cache entry - mark as Fresh (processed)
                cached_file.modified_time = modified_time_utc;
                cached_file.cache_status = CacheStatus::Fresh;
                
                // TODO: In real implementation, this is where we would:
                // 1. Parse the JSONL file content
                // 2. Extract usage entries, blocks, per-hour data 
                // 3. Populate cached_file.data with processed information
                // 4. Store limit information and session data
                // For now, we just mark it as refreshed
                
                Ok(())
            } else {
                Err(format!("File {} not found in cache for folder {}", file_name, folder_name).into())
            }
        } else {
            Err(format!("Folder {} not found in cache", folder_name).into())
        }
    }
        */
}

/// Parse entries strictly newer than `boundary`.
/// - If the file doesn't exist → returns an empty Vec (silent).
/// - Otherwise: reverse-parse and stop when `timestamp <= boundary`,
///   then restore chronological order.
pub fn parse_file_since_boundary(
    file_path: &str,
    boundary: DateTime<Utc>,
) -> Vec<ClaudeEntry> {
    let content = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(_)  => return Vec::new(),
    };

    let mut entries: Vec<_> = content
        .lines()
        .rev()
        .filter_map(|line| parse_line(line, file_path))
        .take_while(|entry| 
            {
                let timestamp_str = entry.timestamp();
                let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
                    .unwrap_or_else(|_| DateTime::from(Utc::now()));
                timestamp > boundary
            })
        .collect();

    entries.reverse();
    entries
}

/// Parse single JSONL line into ClaudeBarUsageEntry
pub fn parse_line(line: &str, file_path: &str) -> Option<ClaudeEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    serde_json::from_str::<ClaudeEntry>(line);
}