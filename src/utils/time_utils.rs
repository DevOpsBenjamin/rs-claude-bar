use chrono::{DateTime, Utc};

/// Find entries within a time window
pub fn entries_within_window(
    entries: &[crate::claudebar_types::usage_entry::ClaudeBarUsageEntry],
    start: DateTime<Utc>,
    end: DateTime<Utc>
) -> Vec<&crate::claudebar_types::usage_entry::ClaudeBarUsageEntry> {
    entries
        .iter()
        .filter(|entry| entry.timestamp >= start && entry.timestamp <= end)
        .collect()
}

/// Calculate the time gap between two entries in hours
pub fn gap_hours(
    earlier: &crate::claudebar_types::usage_entry::ClaudeBarUsageEntry,
    later: &crate::claudebar_types::usage_entry::ClaudeBarUsageEntry
) -> i64 {
    later.timestamp.signed_duration_since(earlier.timestamp).num_hours()
}