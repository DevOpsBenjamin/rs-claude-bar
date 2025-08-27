use serde::{Deserialize, Serialize};
use super::tool_use::{ToolUseBlock, ToolResultBlock};

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
    ToolResult(ToolResultBlock),
    
    /// Thinking block (Claude's internal reasoning)
    #[serde(rename = "thinking")]
    Thinking { content: String },
    
    /// Unknown/future content type - fallback for any unrecognized types
    #[serde(other)]
    Unknown,
}

impl ContentBlock {
    /// Extract text content from any block type
    pub fn extract_text(&self) -> Option<&str> {
        match self {
            ContentBlock::Text { text } => Some(text),
            ContentBlock::Thinking { content } => Some(content),
            ContentBlock::ToolResult(result) => result.content.as_deref(),
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
        matches!(self, ContentBlock::ToolResult(_))
    }
}