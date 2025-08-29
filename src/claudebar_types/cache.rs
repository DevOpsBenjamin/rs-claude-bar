use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFile {
    pub file_name: String,
    pub cache_date: DateTime<Utc>,
    pub data: Vec<CachedFileData>, // Empty for now, will be populated later
    #[serde(skip)]
    pub cache_status: CacheStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFolder {
    pub folder_name: String,
    pub files: Vec<CachedFile>,
}

// represent the cache information in .claude_bar/uid.json
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CacheInfo {
    pub folders: Vec<CachedFolder>
}

// represent .claude_bar/caches.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub base_path: String,
    pub file_path: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFileData {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockLines {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerHourBlock {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFileInfo {
    pub name: String,
    pub file_name: String,
    pub folder_name: String,
    pub modified_time: DateTime<Utc>,
    pub created_time: DateTime<Utc>,
    pub size_bytes: u64,
}
