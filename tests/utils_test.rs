use chrono::Duration;
use rs_claude_bar::utils::{format_duration, format_model_name, format_token_count};

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