use serde::{Deserialize, Serialize};
use super::message::TranscriptMessage;

/// Union type for different entry types in JSONL files
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeEntry {
    /// Session summary entries (simpler structure)
    Summary {
        #[serde(rename = "type")]
        entry_type: String, // Should be "summary"
        summary: String,
        #[serde(rename = "leafUuid")]
        leaf_uuid: String,
    },
    /// Regular transcript entries (more complex structure)
    Transcript(TranscriptEntry),
    /// Fallback for any other JSON structure
    Unknown(serde_json::Value),
}
impl ClaudeEntry {
    /// Get the timestamp if available
    pub fn timestamp(&self) -> Option<&str> {
        match self {
            ClaudeEntry::Transcript(entry) => Some(&entry.timestamp),
            ClaudeEntry::Summary { .. } => None,
            ClaudeEntry::Unknown(value) => {
                // Try to extract timestamp from unknown entry
                value.get("timestamp").and_then(|v| v.as_str())
            }
        }
    }
    
    /// Check if this entry has usage information
    pub fn has_usage(&self) -> bool {
        match self {
            ClaudeEntry::Transcript(entry) => entry.message.usage.is_some(),
            ClaudeEntry::Summary { .. } => false,
            ClaudeEntry::Unknown(_) => false,
        }
    }
    
    /// Get usage if available
    pub fn usage(&self) -> Option<&super::usage::MessageUsage> {
        match self {
            ClaudeEntry::Transcript(entry) => entry.message.usage.as_ref(),
            ClaudeEntry::Summary { .. } => None,
            ClaudeEntry::Unknown(_) => None,
        }
    }
}

/// Full Claude Code transcript entry - the top-level structure in JSONL files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptEntry {
    /// Parent conversation UUID (can be null)
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
    
    /// Whether this is a sidechain conversation
    #[serde(rename = "isSidechain")]
    pub is_sidechain: bool,
    
    /// Type of user (external, internal, etc.)
    #[serde(rename = "userType")]
    pub user_type: String,
    
    /// Current working directory when message was sent
    pub cwd: String,
    
    /// Session ID for this conversation
    #[serde(rename = "sessionId")]
    pub session_id: String,
    
    /// Claude Code version
    pub version: String,
    
    /// Git branch if in a git repository
    #[serde(rename = "gitBranch")]
    pub git_branch: String,
    
    /// Type of entry (user, assistant, system, etc.)
    #[serde(rename = "type")]
    pub entry_type: String,
    
    /// Unique ID for this specific entry
    pub uuid: String,
    
    /// ISO timestamp when entry was created
    pub timestamp: String,
    
    /// The actual message content and metadata
    pub message: TranscriptMessage,
    
    /// Whether this represents an API error
    #[serde(rename = "isApiErrorMessage", default)]
    pub is_api_error_message: bool,
    
    /// Whether this is a meta entry
    #[serde(rename = "isMeta", default)]
    pub is_meta: bool,
    
    /// Cost in USD (may be missing in some entries)
    #[serde(rename = "costUSD", default)]
    pub cost_usd: Option<f64>,
    
    /// Tool use result information (may be missing in some entries)
    #[serde(rename = "toolUseResult", default)]
    pub tool_use_result: Option<serde_json::Value>,
    
    /// Request ID (may be missing in some entries)
    #[serde(rename = "requestId", default)]
    pub request_id: Option<String>,
}