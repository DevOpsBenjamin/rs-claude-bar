use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;
use crate::claudebar_types::file_info::{FileSystemInfo, FolderInfo};

/// Scan all Claude data folders and extract file system information
pub fn scan_claude_folders(base_path: &str) -> Vec<FolderInfo> {
    let mut folders = Vec::new();
    let path = Path::new(base_path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                let folder_path = entry.path().to_string_lossy().to_string();
                
                let folder_info = scan_folder(&entry.path(), &folder_name, &folder_path);
                folders.push(folder_info);
            }
        }
    }

    folders
}

/// Scan a single folder for file information
fn scan_folder(folder_path: &Path, folder_name: &str, full_folder_path: &str) -> FolderInfo {
    let mut files = Vec::new();
    let mut total_size = 0u64;

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                let file_name = entry.file_name().to_string_lossy().to_string();
                
                // Get file metadata
                if let Ok(metadata) = entry.metadata() {
                    let size_bytes = metadata.len();
                    total_size += size_bytes;

                    // Get modification time
                    let modified_time = metadata
                        .modified()
                        .ok()
                        .map(|time| DateTime::<Utc>::from(time))
                        .unwrap_or_else(Utc::now);;

                    // Get creation time (may not be available on Linux)
                    let created_time = metadata
                        .created()
                        .ok()
                        .map(|time| DateTime::<Utc>::from(time))
                        .unwrap_or_else(Utc::now);;

                    let file_info = FileSystemInfo {
                        folder_name: folder_name.to_string(),
                        file_name: file_name.clone(),
                        file_path: entry.path().to_string_lossy().to_string(),
                        size_bytes,
                        modified_time,
                        created_time,
                        exists: true,
                    };

                    files.push(file_info);
                }
            }
        }
    }

    // Sort files by modification time (most recent first)
    files.sort_by(|a, b| b.modified_time.cmp(&a.modified_time));

    let total_files = files.len();

    FolderInfo {
        folder_name: folder_name.to_string(),
        folder_path: full_folder_path.to_string(),
        files,
        total_files,
        total_size_bytes: total_size,
    }
}