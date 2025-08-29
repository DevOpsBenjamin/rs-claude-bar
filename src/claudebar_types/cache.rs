use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFile {
    pub file_name: String,
    pub cache_date: DateTime<Utc>,
    pub infos: Vec<serde_json::Value>, // Empty for now, will be populated later
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFolder {
    pub folder_name: String,
    pub files: Vec<CachedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub folders: Vec<CachedFolder>,
    pub last_updated: DateTime<Utc>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            folders: Vec::new(),
            last_updated: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CacheStatus {
    Fresh,           // File in cache and up-to-date
    NeedsRefresh,    // File modified since cache date
    NotInCache,      // File not in cache yet
}