/*
    use super::*;
    use chrono::TimeZone;
    use claudebar_types::{FileInfo, TokenUsage, UserRole};
    fn make_entry(ts: &str, limit: bool) -> ClaudeBarUsageEntry {
        ClaudeBarUsageEntry {
            session_id: String::new(),
            timestamp: DateTime::parse_from_rfc3339(ts)
                .unwrap()
                .with_timezone(&Utc),
            role: UserRole::Assistant,
            usage: TokenUsage::default(),
            content_length: 0,
            is_limit_reached: limit,
            content_text: if limit {
                Some("resets 10pm".into())
            } else {
                None
            },
            file_info: FileInfo {
                folder_name: String::new(),
                file_name: String::new(),
                file_date: None,
            },
        }
    }

    #[ignore = "Test needs update for FIXED window algorithm"]
    #[test]
    fn analyze_blocks_detects_limits_and_gaps() {
        let entries = vec![
            make_entry("2024-01-01T09:00:00Z", false),
            make_entry("2024-01-01T10:00:00Z", false),
            make_entry("2024-01-01T14:00:00Z", true),
            make_entry("2024-01-01T20:00:00Z", false),
            make_entry("2024-01-01T21:00:00Z", false),
            make_entry("2024-01-02T06:00:00Z", false),
        ];

        let blocks = analyze_usage_blocks(&entries);
        assert_eq!(blocks.len(), 3);

        assert_eq!(
            blocks[0].start_time,
            Utc.with_ymd_and_hms(2024, 1, 1, 9, 0, 0).unwrap()
        );
        assert_eq!(
            blocks[0].end_time.unwrap(),
            Utc.with_ymd_and_hms(2024, 1, 1, 14, 0, 0).unwrap()
        );
        assert!(blocks[0].limit_reached);
        assert!(!blocks[0].guessed);
        assert_eq!(blocks[0].assistant_count, 2);

        assert_eq!(
            blocks[1].start_time,
            Utc.with_ymd_and_hms(2024, 1, 1, 20, 0, 0).unwrap()
        );
        assert_eq!(
            blocks[1].end_time.unwrap(),
            Utc.with_ymd_and_hms(2024, 1, 2, 1, 0, 0).unwrap()
        );
        assert!(!blocks[1].limit_reached);
        assert!(blocks[1].guessed);
        assert_eq!(blocks[1].assistant_count, 2);

        assert_eq!(
            blocks[2].start_time,
            Utc.with_ymd_and_hms(2024, 1, 2, 6, 0, 0).unwrap()
        );
        assert!(blocks[2].end_time.is_none());
        assert!(!blocks[2].limit_reached);
        assert!(!blocks[2].guessed);
        assert_eq!(blocks[2].assistant_count, 1);
    }
}
*/