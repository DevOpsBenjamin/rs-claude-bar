use chrono::{DateTime, Utc};
use crate::claudebar_types::usage_entry::ClaudeBarUsageEntry;

#[derive(Debug, Clone)]
pub struct UsageBlock {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub entries: Vec<ClaudeBarUsageEntry>,
    pub assistant_count: usize,
    pub limit_reached: bool,
    pub reset_time: Option<String>,         // e.g., "10pm", "11pm"
    pub unlock_time: Option<DateTime<Utc>>, // calculated unlock timestamp
    pub guessed: bool,
    pub total_tokens: u32,
}