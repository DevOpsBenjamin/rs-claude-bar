use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemInfo {
    pub file_name: String,
    pub modified_time: DateTime<Utc>,
    pub created_time: DateTime<Utc>,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    pub folder_name: String,
    pub files: Vec<FileSystemInfo>,
}