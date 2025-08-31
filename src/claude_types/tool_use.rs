use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool use request block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUseBlock {
    /// Unique ID for this tool use
    pub id: String,
    
    /// Name of the tool being used
    pub name: String,
    
    /// Input parameters for the tool (arbitrary JSON)
    pub input: Value,
}

/// A tool execution result block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultBlock {
    /// ID of the corresponding tool_use block
    pub tool_use_id: String,
    
    /// Whether the tool execution was successful
    #[serde(default)]
    pub is_error: bool,
    
    /// Result content (may be text, JSON, etc.)
    pub content: Option<String>,
}

impl ToolUseBlock {
    /// Get a specific parameter from the input
    pub fn get_param<T>(&self, key: &str) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let value = self.input.get(key)?;
        serde_json::from_value(value.clone()).ok()
    }
    
    /// Get parameter as string
    pub fn get_param_str(&self, key: &str) -> Option<String> {
        self.input.get(key)?.as_str().map(|s| s.to_string())
    }
}

impl ToolResultBlock {
    /// Check if this represents an error result
    pub fn is_error(&self) -> bool {
        self.is_error
    }
    
    /// Get content as string, handling None case
    pub fn content_or_empty(&self) -> &str {
        self.content.as_deref().unwrap_or("")
    }
}