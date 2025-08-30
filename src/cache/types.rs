use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// represent the cache information in .claude_bar/cache.json
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CacheInfo {
    pub folders: HashMap<String, CachedFolder>
}
impl Default for CacheInfo {
    fn default() -> Self { CacheInfo { folders: HashMap::new() } }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFolder {
    pub files: HashMap<String, CachedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFile {
    pub file_name: String,
    pub cache_time: DateTime<Utc>,   //Use as cache date
    /// Map of limit/unlock events keyed by block timestamp
    pub blocks: HashMap<DateTime<Utc>, BlockLine>,
    /// Map of hourly usage summaries (hour_start -> PerHourBlock) for O(1) lookup
    pub per_hour: HashMap<DateTime<Utc>, PerHourBlock>,
    #[serde(skip)]
    pub cache_status: CacheStatus,
    #[serde(skip)]
    pub modified_time: DateTime<Utc>,
    #[serde(skip)]
    pub created_time: DateTime<Utc>,
    #[serde(skip)]
    pub size_bytes: u64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockLine {
    /// Timestamp when the block was lifted/reset (if available)
    pub unlock_timestamp: Option<DateTime<Utc>>,
    /// Human-readable reset time (e.g. "5pm", "2h30m")
    pub reset_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerHourBlock {
    /// Start of the hour block (e.g. 01:00:00)
    pub hour_start: DateTime<Utc>,
    /// End of the hour block (e.g. 01:59:59)  
    pub hour_end: DateTime<Utc>,
    /// Minimum timestamp found in this hour
    pub min_timestamp: DateTime<Utc>,
    /// Maximum timestamp found in this hour  
    pub max_timestamp: DateTime<Utc>,
    /// Total input tokens used in this hour
    pub input_tokens: u32,
    /// Total output tokens used in this hour
    pub output_tokens: u32,
    /// Total cache creation tokens in this hour
    pub cache_creation_tokens: u32,
    /// Total cache read tokens in this hour
    pub cache_read_tokens: u32,
    /// Number of assistant messages in this hour
    pub assistant_messages: u32,
    /// Number of user messages in this hour
    pub user_messages: u32,
    /// Total content length of all messages in this hour
    pub total_content_length: u64,
    /// Number of entries processed in this hour
    pub entry_count: u32,
}

#[derive(Debug, Clone)]
pub enum CacheStatus {
    Fresh,           // File in cache and up-to-date
    NeedsRefresh,    // File modified since cache date
    NotInCache,      // File not in cache yet
}
impl Default for CacheStatus {
    fn default() -> Self { CacheStatus::NotInCache }
}
