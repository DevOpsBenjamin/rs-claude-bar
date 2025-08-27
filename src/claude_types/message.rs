use serde::{Deserialize, Serialize};
use super::usage::MessageUsage;
use super::content::ContentBlock;

/// The message portion of a transcript entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptMessage {
    /// Message ID (may be missing in some entries)
    #[serde(default)]
    pub id: Option<String>,
    
    /// Model used for this message (e.g., "claude-sonnet-4-20250514" or "<synthetic>")
    #[serde(default)]
    pub model: Option<String>,
    
    /// Role of the message sender (user, assistant, system)
    #[serde(default)]
    pub role: Option<String>,
    
    /// Why the message stopped (stop_sequence, max_tokens, etc.)
    pub stop_reason: Option<String>,
    
    /// The actual stop sequence used
    pub stop_sequence: Option<String>,
    
    /// Message type
    #[serde(rename = "type", default)]
    pub message_type: Option<String>,
    
    /// Token usage information
    #[serde(default)]
    pub usage: Option<MessageUsage>,
    
    /// Content - can be either a string or array of content blocks
    #[serde(default)]
    pub content: Option<MessageContent>,
}

/// Content can be either a string (for simple messages) or array of blocks (for complex messages)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple string content
    String(String),
    /// Array of structured content blocks
    Blocks(Vec<ContentBlock>),
}

impl MessageContent {
    /// Extract text content regardless of format
    pub fn as_text(&self) -> String {
        match self {
            MessageContent::String(s) => s.clone(),
            MessageContent::Blocks(blocks) => {
                blocks.iter()
                    .filter_map(|block| block.extract_text())
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }
    
    /// Check if this is a string content
    pub fn is_string(&self) -> bool {
        matches!(self, MessageContent::String(_))
    }
    
    /// Check if this is block content
    pub fn is_blocks(&self) -> bool {
        matches!(self, MessageContent::Blocks(_))
    }
}

impl Default for MessageContent {
    fn default() -> Self {
        MessageContent::String(String::new())
    }
}

/// Model information structure (alternative format seen in some entries)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID
    pub id: String,
    
    /// Human-readable model name
    #[serde(rename = "display_name")]
    pub display_name: String,
}