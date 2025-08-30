use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct LimitBlock {
    /// Timestamp when the block was lifted/reset (if available)
    pub unlock_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockKind {
    Limit,
    Gap,
    Current,
}

/// Aggregated metrics for any block
#[derive(Debug, Clone, Default)]
pub struct DataStats {
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

/// Unified block representation with type + time span + stats
#[derive(Debug, Clone)]
pub struct DataBlock {
    pub kind: BlockKind,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub min_timestamp: DateTime<Utc>,
    pub max_timestamp: DateTime<Utc>,
    pub unlock_timestamp: Option<DateTime<Utc>>, // only for Limit/Current when applicable
    pub stats: DataStats,
}
