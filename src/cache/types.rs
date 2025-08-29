use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// represent the cache information in .claude_bar/cache.json
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CacheInfo {
    pub folders: Vec<CachedFolder>
}
impl Default for CacheInfo {
    fn default() -> Self { CacheInfo { folders: Vec::new() } }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFolder {
    pub folder_name: String,
    pub files: Vec<CachedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFile {
    pub file_name: String,
    pub modified_time: DateTime<Utc>,   //Use as cache date
    pub data: Vec<CachedFileData>,      // Empty for now, will be populated later
    #[serde(skip)]
    pub cache_status: CacheStatus,
    #[serde(skip)]
    pub created_time: DateTime<Utc>,
    #[serde(skip)]
    pub size_bytes: u64,
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

#[derive(Debug, Clone)]
pub enum CacheStatus {
    Fresh,           // File in cache and up-to-date
    NeedsRefresh,    // File modified since cache date
    NotInCache,      // File not in cache yet
}
impl Default for CacheStatus {
    fn default() -> Self { CacheStatus::NotInCache }
}
