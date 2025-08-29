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

/// Round timestamp down to previous hour boundary (e.g., 15:44 → 15:00)
pub fn round_to_hour_boundary(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_minute(0)
      .unwrap()
      .with_second(0)
      .unwrap()
      .with_nanosecond(0)
      .unwrap()
}