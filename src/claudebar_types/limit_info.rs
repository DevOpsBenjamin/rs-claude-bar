use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Essential limit detection data for cache storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitInfo {
    /// When the limit was hit
    pub timestamp: DateTime<Utc>,
    
    /// Session ID where limit occurred
    pub session_id: String,
    
    /// Reset time mentioned in the message (e.g., "10pm", "11pm")
    pub reset_time: Option<String>,
    
    /// The actual limit message content (truncated for cache efficiency)
    pub message_content: String,
    
    /// Calculated unlock time based on reset_time
    pub unlock_time: Option<DateTime<Utc>>,
    
    /// Role who hit the limit (usually Assistant)
    pub role: String,
    
    /// File info where this limit was found
    pub file_folder: String,
    pub file_name: String,
}

impl LimitInfo {
    /// Create a new LimitInfo from parsed data
    pub fn new(
        timestamp: DateTime<Utc>,
        session_id: String,
        role: String,
        message_content: String,
        file_folder: String,
        file_name: String,
    ) -> Self {
        // Extract reset time from message content (simplified for now)
        let reset_time = Self::extract_reset_time(&message_content);
        let unlock_time = Self::calculate_unlock_time(timestamp, reset_time.as_deref());
        
        // Truncate message for cache efficiency (first 200 chars)
        let truncated_message = if message_content.len() > 200 {
            format!("{}...", &message_content[..197])
        } else {
            message_content
        };
        
        Self {
            timestamp,
            session_id,
            reset_time,
            message_content: truncated_message,
            unlock_time,
            role,
            file_folder,
            file_name,
        }
    }
    
    /// Extract reset time from limit message content
    /// Look for patterns like "10pm", "11pm", "reset at 10pm", etc.
    fn extract_reset_time(content: &str) -> Option<String> {
        // Simple regex-like matching for common reset time patterns
        let content_lower = content.to_lowercase();
        
        // Look for "10pm", "11pm" patterns
        for hour in 1..=12 {
            let pattern_pm = format!("{}pm", hour);
            let pattern_am = format!("{}am", hour);
            
            if content_lower.contains(&pattern_pm) {
                return Some(pattern_pm);
            }
            if content_lower.contains(&pattern_am) {
                return Some(pattern_am);
            }
        }
        
        None
    }
    
    /// Calculate unlock time based on reset time and current timestamp
    fn calculate_unlock_time(
        limit_timestamp: DateTime<Utc>,
        reset_time: Option<&str>,
    ) -> Option<DateTime<Utc>> {
        // TODO: Implement proper unlock time calculation
        // This is a placeholder - would need timezone logic and proper time parsing
        reset_time?; // For now, return None if no reset time
        
        // Placeholder: assume unlock is next day at reset time
        // Real implementation would need timezone handling
        Some(limit_timestamp + chrono::Duration::hours(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_extract_reset_time() {
        assert_eq!(
            LimitInfo::extract_reset_time("Usage limit reached. Resets at 10pm EST."),
            Some("10pm".to_string())
        );
        
        assert_eq!(
            LimitInfo::extract_reset_time("Limit hit, try again at 11pm"),
            Some("11pm".to_string())
        );
        
        assert_eq!(
            LimitInfo::extract_reset_time("Usage limit exceeded"),
            None
        );
    }
    
    #[test]
    fn test_message_truncation() {
        let long_message = "a".repeat(300);
        let timestamp = Utc::now();
        
        let limit_info = LimitInfo::new(
            timestamp,
            "session123".to_string(),
            "assistant".to_string(),
            long_message,
            "folder".to_string(),
            "file.jsonl".to_string(),
        );
        
        assert!(limit_info.message_content.len() <= 200);
        assert!(limit_info.message_content.ends_with("..."));
    }
}