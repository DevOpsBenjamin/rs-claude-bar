use serde::{Deserialize, Serialize};
use super::tool_use::ToolUseBlock;

/// A content block within a message - can be text, tool_use, tool_result, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    /// Plain text content
    #[serde(rename = "text")]
    Text { text: String },
    
    /// Tool use request
    #[serde(rename = "tool_use")]
    ToolUse(ToolUseBlock),
    
    /// Tool execution result
    #[serde(rename = "tool_result")]
    ToolResult {
        #[serde(rename = "tool_use_id")]
        tool_use_id: String,
        content: String,
        #[serde(default)]
        is_error: bool,
    },
    
    /// Thinking block (Claude's internal reasoning)
    #[serde(rename = "thinking")]
    Thinking { 
        thinking: String,
        signature: Option<String>,
    },
    
    /// Unknown/future content type - fallback for any unrecognized types
    #[serde(other)]
    Unknown,
}

impl ContentBlock {
    /// Extract text content from any block type
    pub fn extract_text(&self) -> Option<&str> {
        match self {
            ContentBlock::Text { text } => Some(text),
            ContentBlock::Thinking { thinking, .. } => Some(thinking),
            ContentBlock::ToolResult { content, .. } => Some(content),
            _ => None,
        }
    }
    
    /// Check if this is a text block
    pub fn is_text(&self) -> bool {
        matches!(self, ContentBlock::Text { .. })
    }
    
    /// Check if this is a tool use block
    pub fn is_tool_use(&self) -> bool {
        matches!(self, ContentBlock::ToolUse(_))
    }
    
    /// Check if this is a tool result block
    pub fn is_tool_result(&self) -> bool {
        matches!(self, ContentBlock::ToolResult { .. })
    }
}