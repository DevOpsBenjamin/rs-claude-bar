use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemInfo {
    pub folder_name: String,
    pub file_name: String,
    pub file_path: String, // full path
    pub size_bytes: u64,
    pub modified_time: DateTime<Utc>,
    pub created_time: Option<DateTime<Utc>>, // Not available on all systems
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    pub folder_name: String,
    pub folder_path: String,
    pub files: Vec<FileSystemInfo>,
    pub total_files: usize,
    pub total_size_bytes: u64,
    pub most_recent_modified: Option<DateTime<Utc>>,
}