use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GuessBlock {
    pub msg_timestamp: DateTime<Utc>,
    pub reset: String,
    pub end: DateTime<Utc>,
    pub start: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CurrentBlock {
    pub reset: String,
    pub end: Option<DateTime<Utc>>,
    pub start: Option<DateTime<Utc>>,
    
    pub min_timestamp: DateTime<Utc>,
    pub max_timestamp: DateTime<Utc>,
    pub assistant: AssistantInfo,
    pub user: UserInfo,
}

#[derive(Debug, Clone)]
pub struct AssistantInfo {
    pub content: i32,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cache_creation_tokens: i64,
    pub cache_read_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub content: i32
}
