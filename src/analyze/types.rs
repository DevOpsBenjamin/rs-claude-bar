use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct LimitBlock {
    /// Timestamp when the block was lifted/reset (if available)
    pub unlock_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DataBlock {
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cache_creation_tokens: i64,
    pub cache_read_tokens: i64,
    pub total_tokens: i64,
    pub assistant_messages: i64,
    pub user_messages: i64,
    pub total_content_length: i64,
    pub entry_count: i64,
}