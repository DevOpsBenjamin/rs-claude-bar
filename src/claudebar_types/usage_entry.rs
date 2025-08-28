use crate::claude_types::{MessageUsage, TranscriptEntry};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Simplified entry for Claude Bar usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeBarUsageEntry {
    /// Session ID for grouping entries
    pub session_id: String,

    /// Timestamp when the entry was created
    pub timestamp: DateTime<Utc>,

    /// Role (user, assistant, or unknown)
    pub role: UserRole,

    /// Token usage information
    pub usage: TokenUsage,

    /// Content message length (character count)
    pub content_length: usize,

    /// Whether this is a 5-hour limit reached message
    pub is_limit_reached: bool,

    /// Full content text (only stored for limit messages to save memory)
    pub content_text: Option<String>,

    /// File information
    pub file_info: FileInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Assistant,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_creation_tokens: u32,
    pub cache_read_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Folder name (e.g., "-workspace-git-rs-claude-bar")
    pub folder_name: String,

    /// File name (e.g., "b5a23264-fc0b-444f-a7b4-54caa0bfaf07.jsonl")
    pub file_name: String,

    /// File creation/modification date
    pub file_date: Option<DateTime<Utc>>,
}

impl ClaudeBarUsageEntry {
    /// Convert a TranscriptEntry to ClaudeBarUsageEntry
    pub fn from_transcript(
        transcript: &TranscriptEntry,
        folder_name: String,
        file_name: String,
        file_date: Option<DateTime<Utc>>,
    ) -> Self {
        // Determine role
        let role = match transcript.message.role.as_deref() {
            Some("user") => UserRole::User,
            Some("assistant") => UserRole::Assistant,
            _ => UserRole::Unknown,
        };

        // Extract token usage (0 when not present)
        let usage = if let Some(msg_usage) = &transcript.message.usage {
            TokenUsage::from_message_usage(msg_usage)
        } else {
            TokenUsage::default()
        };

        // Calculate content length and extract text
        let content_text_full = transcript.message.content.as_text();
        let content_length = content_text_full.len();

        // Check if this is a 5-hour limit message
        let is_limit_reached =
            transcript.is_api_error_message && content_text_full.contains("5-hour limit reached");

        // Store content text only for limit messages to save memory
        let content_text = if is_limit_reached {
            Some(content_text_full)
        } else {
            None
        };

        // Parse timestamp
        let timestamp = DateTime::parse_from_rfc3339(&transcript.timestamp)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        ClaudeBarUsageEntry {
            session_id: transcript.session_id.clone(),
            timestamp,
            role,
            usage,
            content_length,
            is_limit_reached,
            content_text,
            file_info: FileInfo {
                folder_name,
                file_name,
                file_date,
            },
        }
    }
}

impl TokenUsage {
    /// Convert from MessageUsage
    pub fn from_message_usage(usage: &MessageUsage) -> Self {
        let input_tokens = usage.input_tokens;
        let output_tokens = usage.output_tokens;
        let cache_creation_tokens = usage.cache_creation_input_tokens;
        let cache_read_tokens = usage.cache_read_input_tokens;
        let total_tokens = input_tokens + output_tokens + cache_creation_tokens + cache_read_tokens;

        Self {
            input_tokens,
            output_tokens,
            cache_creation_tokens,
            cache_read_tokens,
            total_tokens,
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Assistant => write!(f, "assistant"),
            UserRole::Unknown => write!(f, "unknown"),
        }
    }
}
