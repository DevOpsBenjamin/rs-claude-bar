use chrono::Duration;

/// Format a duration into human-readable format (e.g., "1h30m", "45m")
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();
    if total_seconds < 0 {
        return "0m".to_string();
    }
    
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    
    if hours > 0 {
        format!("{}h{}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

/// Format token count in human-readable format (e.g., "1.2M", "500k", "123")
pub fn format_token_count(tokens: u32) -> String {
    if tokens >= 1_000_000 {
        format!("{:.1}M", tokens as f64 / 1_000_000.0)
    } else if tokens >= 1_000 {
        format!("{}k", tokens / 1_000)
    } else {
        tokens.to_string()
    }
}

