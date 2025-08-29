use std::{fs, path::Path};
use chrono::{DateTime, Utc};

use crate::claudebar_types::file_info::{FileSystemInfo, FolderInfo};

/// Scan all Claude data folders and extract file system information
pub fn scan_claude_folders(base_path: &str) -> Vec<FolderInfo> {
    let path = Path::new(base_path);

    let entries = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return Vec::new(),
    };

    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|entry| {
            let folder_name = entry.file_name().to_string_lossy().to_string();
            scan_folder(&entry.path(), &folder_name)
        })
        .collect()
}

/// Scan a single folder for file information
fn scan_folder(folder_path: &Path, folder_name: &str) -> FolderInfo {
    let entries = match fs::read_dir(folder_path) {
        Ok(rd) => rd,
        Err(_) => {
            return FolderInfo {
                folder_name: folder_name.to_string(),
                files: Vec::new(),
            }
        }
    };

    let files: Vec<FileSystemInfo> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let size_bytes = metadata.len();

            let modified_time = metadata
                .modified()
                .ok()
                .map(DateTime::<Utc>::from)
                .unwrap_or_else(Utc::now);

            // Creation time may not be available on some platforms
            let created_time = metadata
                .created()
                .ok()
                .map(DateTime::<Utc>::from)
                .unwrap_or_else(Utc::now);

            Some(FileSystemInfo {
     
                file_name: entry.file_name().to_string_lossy().to_string(),
                modified_time,
                created_time,           
                size_bytes,
            })
        })
        .collect();
    FolderInfo {
        folder_name: folder_name.to_string(),
        files,
    }
}