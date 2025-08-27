use serde::{Deserialize, Serialize};
use super::message::TranscriptMessage;

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
}