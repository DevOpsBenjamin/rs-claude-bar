/// Format numbers with thousands separators
pub fn format_number_with_separators(num: u32) -> String {
    let num_str = num.to_string();
    let mut result = String::new();
    let chars: Vec<char> = num_str.chars().collect();
    
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    
    result
}

/// Format duration in hours and minutes
pub fn format_duration_hours(duration: chrono::Duration) -> String {
    let total_minutes = duration.num_minutes();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    
    if hours > 0 {
        format!("{}h{:02}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_format_number_with_separators() {
        assert_eq!(format_number_with_separators(1234), "1,234");
        assert_eq!(format_number_with_separators(1234567), "1,234,567");
        assert_eq!(format_number_with_separators(123), "123");
    }

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration_hours(Duration::minutes(30)), "30m");
        assert_eq!(format_duration_hours(Duration::minutes(90)), "1h30m");
        assert_eq!(format_duration_hours(Duration::minutes(125)), "2h05m");
    }
}