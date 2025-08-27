use chrono::{DateTime, Utc};

/// Represents a single usage entry from Claude Code logs
#[derive(Debug, Clone)]
pub struct UsageEntry {
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub model_id: String,
    pub model_display_name: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_creation_tokens: u32,
    pub cache_read_tokens: u32,
    pub total_tokens: u32,
    pub cost_usd: f64,
}

/// Represents a 5-hour usage window
#[derive(Debug, Clone)]
pub struct UsageWindow {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_tokens: u32,
    pub total_cost: f64,
    pub message_count: usize,
    pub models_used: Vec<String>,
    pub is_active: bool,
}

impl UsageEntry {
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            session_id: String::new(),
            model_id: String::new(),
            model_display_name: String::new(),
            input_tokens: 0,
            output_tokens: 0,
            cache_creation_tokens: 0,
            cache_read_tokens: 0,
            total_tokens: 0,
            cost_usd: 0.0,
        }
    }
}

impl UsageWindow {
    pub fn new(start_time: DateTime<Utc>) -> Self {
        Self {
            start_time,
            end_time: start_time + chrono::Duration::hours(5),
            total_tokens: 0,
            total_cost: 0.0,
            message_count: 0,
            models_used: Vec::new(),
            is_active: false,
        }
    }
}