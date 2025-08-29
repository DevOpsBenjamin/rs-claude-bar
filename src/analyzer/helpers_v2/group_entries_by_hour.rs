use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::claudebar_types::{
    per_hour_log::PerHourLog,
    usage_entry::{ClaudeBarUsageEntry, UserRole},
};

use super::hour_boundary::round_to_hour_boundary;

/// Group entries by hour boundaries and create PerHourLog summaries
pub fn group_entries_by_hour(entries: Vec<ClaudeBarUsageEntry>) -> Vec<PerHourLog> {
    let mut hour_map: HashMap<DateTime<Utc>, PerHourLog> = HashMap::new();
    
    for entry in entries {
        let hour_start = round_to_hour_boundary(entry.timestamp);
        
        let hour_log = hour_map.entry(hour_start).or_insert_with(|| PerHourLog {
            hour_start,
            first_entry: entry.timestamp,
            last_entry: entry.timestamp,
            entry_count: 0,
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_cache_creation_tokens: 0,
            total_cache_read_tokens: 0,
            total_tokens: 0,
            total_content_length: 0,
            user_entries: 0,
            assistant_entries: 0,
            limit_hits: 0,
        });
        
        // Update hour log with this entry
        hour_log.last_entry = hour_log.last_entry.max(entry.timestamp);
        hour_log.first_entry = hour_log.first_entry.min(entry.timestamp);
        hour_log.entry_count += 1;
        hour_log.total_input_tokens += entry.usage.input_tokens;
        hour_log.total_output_tokens += entry.usage.output_tokens;
        hour_log.total_cache_creation_tokens += entry.usage.cache_creation_tokens;
        hour_log.total_cache_read_tokens += entry.usage.cache_read_tokens;
        hour_log.total_tokens += entry.usage.total_tokens;
        hour_log.total_content_length += entry.content_length as u32;
        
        match entry.role {
            UserRole::User => hour_log.user_entries += 1,
            UserRole::Assistant => hour_log.assistant_entries += 1,
            UserRole::Unknown => {}
        }
        
        if entry.is_limit_reached {
            hour_log.limit_hits += 1;
        }
    }
    
    // Convert to sorted vector (by hour_start)
    let mut hour_logs: Vec<PerHourLog> = hour_map.into_values().collect();
    hour_logs.sort_by_key(|log| log.hour_start);
    
    hour_logs
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    // Helper to create test entry
    fn create_test_entry(timestamp: DateTime<Utc>, tokens: u32) -> ClaudeBarUsageEntry {
        ClaudeBarUsageEntry {
            session_id: "test".to_string(),
            timestamp,
            role: UserRole::Assistant,
            usage: crate::claudebar_types::usage_entry::Usage {
                input_tokens: tokens,
                output_tokens: tokens,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                total_tokens: tokens * 2,
            },
            content_length: 100,
            content_text: Some("test".to_string()),
            is_limit_reached: false,
            file_info: crate::claudebar_types::usage_entry::FileInfo {
                folder_name: "test".to_string(),
                file_name: "test.jsonl".to_string(),
                file_date: Some(timestamp),
            },
        }
    }

    #[test]
    fn test_empty_entries() {
        let result = group_entries_by_hour(vec![]);
        assert!(result.is_empty());
    }
    
    #[test]
    fn test_single_hour_multiple_entries() {
        let base_time = Utc.with_ymd_and_hms(2025, 8, 29, 15, 30, 0).unwrap();
        let entries = vec![
            create_test_entry(base_time, 100),
            create_test_entry(base_time + chrono::Duration::minutes(15), 200),
        ];
        
        let result = group_entries_by_hour(entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry_count, 2);
        assert_eq!(result[0].total_tokens, 600); // (100 + 200) * 2
    }
    
    #[test]
    fn test_multiple_hours() {
        let entries = vec![
            create_test_entry(Utc.with_ymd_and_hms(2025, 8, 29, 15, 30, 0).unwrap(), 100),
            create_test_entry(Utc.with_ymd_and_hms(2025, 8, 29, 16, 30, 0).unwrap(), 200),
        ];
        
        let result = group_entries_by_hour(entries);
        assert_eq!(result.len(), 2);
        assert!(result[0].hour_start < result[1].hour_start); // Should be sorted
    }
}