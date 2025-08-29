use crate::{
    claudebar_types::{
        usage_entry::ClaudeBarUsageEntry,
        limit_info::LimitInfo,
    },
};

/// Extract limit information from usage entries
pub fn extract_limit_info(entries: &[ClaudeBarUsageEntry]) -> Vec<LimitInfo> {
    let mut limit_infos = Vec::new();
    
    for entry in entries {
        if entry.is_limit_reached {
            // Only process entries that have content
            if let Some(content) = &entry.content_text {
                // Check if this looks like a limit message
                if is_limit_message(content) {
                    let limit_info = LimitInfo::new(
                        entry.timestamp,
                        entry.session_id.clone(),
                        entry.role.to_string(),
                        content.clone(),
                        entry.file_info.folder_name.clone(),
                        entry.file_info.file_name.clone(),
                    );
                    
                    limit_infos.push(limit_info);
                }
            }
        }
    }
    
    limit_infos
}

/// Check if message content contains limit-related keywords
fn is_limit_message(content: &str) -> bool {
    let content_lower = content.to_lowercase();
    
    // Common limit message patterns
    let limit_keywords = [
        "usage limit",
        "rate limit",
        "limit reached",
        "limit exceeded",
        "too many requests",
        "quota exceeded",
        "usage quota",
        "reset",
        "try again",
    ];
    
    limit_keywords.iter().any(|keyword| content_lower.contains(keyword))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::claudebar_types::usage_entry::{UserRole, Usage, FileInfo};
    
    fn create_test_entry(content: &str, is_limit: bool) -> ClaudeBarUsageEntry {
        ClaudeBarUsageEntry {
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            role: UserRole::Assistant,
            usage: Usage {
                input_tokens: 100,
                output_tokens: 50,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                total_tokens: 150,
            },
            content_length: content.len(),
            content_text: Some(content.to_string()),
            is_limit_reached: is_limit,
            file_info: FileInfo {
                folder_name: "test-folder".to_string(),
                file_name: "test-file.jsonl".to_string(),
                file_date: Some(Utc::now()),
            },
        }
    }

    #[test]
    fn test_extract_limit_info() {
        let entries = vec![
            create_test_entry("Normal message", false),
            create_test_entry("Usage limit reached. Please try again at 10pm EST.", true),
            create_test_entry("Another normal message", false),
            create_test_entry("Rate limit exceeded", true),
        ];
        
        let limit_infos = extract_limit_info(&entries);
        assert_eq!(limit_infos.len(), 2);
        assert!(limit_infos[0].message_content.contains("Usage limit reached"));
        assert!(limit_infos[1].message_content.contains("Rate limit exceeded"));
    }
    
    #[test]
    fn test_is_limit_message() {
        assert!(is_limit_message("Usage limit reached"));
        assert!(is_limit_message("Rate limit exceeded"));
        assert!(is_limit_message("Too many requests, try again later"));
        assert!(is_limit_message("Your usage quota has been exceeded"));
        
        assert!(!is_limit_message("Normal conversation message"));
        assert!(!is_limit_message("Hello, how can I help you?"));
    }
    
    #[test]
    fn test_no_limit_entries() {
        let entries = vec![
            create_test_entry("Normal message 1", false),
            create_test_entry("Normal message 2", false),
        ];
        
        let limit_infos = extract_limit_info(&entries);
        assert!(limit_infos.is_empty());
    }
}