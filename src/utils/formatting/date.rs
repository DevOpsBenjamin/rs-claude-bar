use chrono::{DateTime, Duration, Utc};

/// Format a UTC datetime using "%m-%d %H:%M", right-aligned to `size` width
pub fn format_date(datetime: DateTime<Utc>, size: usize) -> String {
    let formatted = datetime.format("%m-%d %H:%M").to_string();
    format!("{:>width$}", formatted, width = size)
}