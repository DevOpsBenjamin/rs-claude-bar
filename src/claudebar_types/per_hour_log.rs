use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Per-hour aggregated data for cache storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerHourLog {
    /// Hour block start timestamp (e.g., 2025-08-29T14:00:00Z)
    pub hour_start: DateTime<Utc>,
    
    /// First entry timestamp in this hour block
    pub first_entry: DateTime<Utc>,
    
    /// Last entry timestamp in this hour block  
    pub last_entry: DateTime<Utc>,
    
    /// Total entries in this hour
    pub entry_count: u32,
    
    /// Token usage aggregated for this hour
    pub total_input_tokens: u32,
    pub total_output_tokens: u32,
    pub total_cache_creation_tokens: u32,
    pub total_cache_read_tokens: u32,
    pub total_tokens: u32,
    
    /// Content length aggregated
    pub total_content_length: u32,
    
    /// User vs Assistant entry counts
    pub user_entries: u32,
    pub assistant_entries: u32,
    
    /// Any limit hits in this hour
    pub limit_hits: u32,
}

impl Default for PerHourLog {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            hour_start: now,
            first_entry: now,
            last_entry: now,
            entry_count: 0,
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_cache_creation_tokens: 0,
            total_cache_read_tokens: 0,
            total_tokens: 0,
            total_content_length: 0,
            user_entries: 0,
            assistant_entries: 0,
            limit_hits: 0,
        }
    }
}