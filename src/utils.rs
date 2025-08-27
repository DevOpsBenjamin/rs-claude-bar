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

/// Simplify common model names for status line display
pub fn format_model_name(display_name: &str) -> String {
    // Simplify common model names for status line
    if display_name.contains("Sonnet") {
        if display_name.contains("3.5") {
            "Sonnet 3.5".to_string()
        } else if display_name.contains("4") {
            "Sonnet 4".to_string()
        } else {
            "Sonnet".to_string()
        }
    } else if display_name.contains("Opus") {
        if display_name.contains("4") {
            "Opus 4".to_string()
        } else {
            "Opus".to_string()
        }
    } else if display_name.contains("Haiku") {
        "Haiku".to_string()
    } else {
        display_name.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::seconds(0)), "0m");
        assert_eq!(format_duration(Duration::seconds(30)), "0m");
        assert_eq!(format_duration(Duration::seconds(60)), "1m");
        assert_eq!(format_duration(Duration::seconds(3600)), "1h0m");
        assert_eq!(format_duration(Duration::seconds(3690)), "1h1m");
    }

    #[test]
    fn test_format_model_name() {
        assert_eq!(format_model_name("Claude 3.5 Sonnet"), "Sonnet 3.5");
        assert_eq!(format_model_name("Claude 4 Sonnet"), "Sonnet 4");
        assert_eq!(format_model_name("Claude Opus"), "Opus");
        assert_eq!(format_model_name("Claude Haiku"), "Haiku");
        assert_eq!(format_model_name("Unknown Model"), "Unknown Model");
    }

    #[test]
    fn test_format_token_count() {
        assert_eq!(format_token_count(123), "123");
        assert_eq!(format_token_count(1500), "1k");
        assert_eq!(format_token_count(1_500_000), "1.5M");
    }
}