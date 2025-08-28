use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GuessBlock {
    pub msg_timestamp: DateTime<Utc>,
    pub reset: String,
    pub end: DateTime<Utc>,
    pub start: DateTime<Utc>,
}