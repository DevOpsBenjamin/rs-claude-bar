use chrono::Duration;
use rs_claude_bar::utils::formatting::{
    format_duration,
    format_token_count,
    format_file_size,
    format_number_with_separators
};

#[test]
fn test_format_duration() {
    assert_eq!(format_duration(Duration::seconds(0)), "0m");
    assert_eq!(format_duration(Duration::seconds(30)), "0m");
    assert_eq!(format_duration(Duration::seconds(60)), "1m");
    assert_eq!(format_duration(Duration::seconds(3600)), "1h00m");
    assert_eq!(format_duration(Duration::seconds(3690)), "1h01m");
    assert_eq!(format_duration(Duration::minutes(90)), "1h30m");
    assert_eq!(format_duration(Duration::minutes(125)), "2h05m");
}

#[test]
fn test_format_token_count() {
    assert_eq!(format_token_count(123), "123");
    assert_eq!(format_token_count(1500), "1k");
    assert_eq!(format_token_count(1_500_000), "1.5M");
}

#[test]
fn test_format_file_size() {
    assert_eq!(format_file_size(123), "123 B");
    assert_eq!(format_file_size(1500), "1.5 KB");
    assert_eq!(format_file_size(1547), "1.5 KB");
    assert_eq!(format_file_size(1_500_000), "1.4 MB");
    assert_eq!(format_file_size(1_500_000_000), "1.4 GB");
}

#[test]
fn test_format_number_with_separators() {
    assert_eq!(format_number_with_separators(1234), "1,234");
    assert_eq!(format_number_with_separators(1234567), "1,234,567");
    assert_eq!(format_number_with_separators(123), "123");
}
