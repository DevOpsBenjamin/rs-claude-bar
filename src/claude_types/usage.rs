use serde::{Deserialize, Serialize};

/// Token usage information for a message  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageUsage {
    /// Input tokens consumed
    pub input_tokens: u32,
    
    /// Output tokens generated
    pub output_tokens: u32,
    
    /// Tokens used for cache creation
    pub cache_creation_input_tokens: u32,
    
    /// Tokens read from cache
    pub cache_read_input_tokens: u32,
    
    /// Server-side tool usage statistics
    #[serde(default)]
    pub server_tool_use: Option<ServerToolUse>,
    
    /// Service tier used (if applicable)
    pub service_tier: Option<String>,
}

/// Server-side tool usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerToolUse {
    /// Number of web search requests made
    pub web_search_requests: u32,
}

impl MessageUsage {
    /// Calculate total tokens used
    pub fn total_tokens(&self) -> u32 {
        self.input_tokens + self.output_tokens + self.cache_creation_input_tokens + self.cache_read_input_tokens
    }
    
    /// Check if this usage entry has any token consumption
    pub fn has_tokens(&self) -> bool {
        self.total_tokens() > 0
    }
}