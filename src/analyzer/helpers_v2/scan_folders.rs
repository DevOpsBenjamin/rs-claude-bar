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
            let folder_path = entry.path().to_string_lossy().to_string();
            scan_folder(&entry.path(), &folder_name, &folder_path)
        })
        .collect()
}

/// Scan a single folder for file information
fn scan_folder(folder_path: &Path, folder_name: &str, full_folder_path: &str) -> FolderInfo {
    let entries = match fs::read_dir(folder_path) {
        Ok(rd) => rd,
        Err(_) => {
            return FolderInfo {
                folder_name: folder_name.to_string(),
                folder_path: full_folder_path.to_string(),
                files: Vec::new(),
                total_files: 0,
                total_size_bytes: 0,
            }
        }
    };

    let mut files: Vec<FileSystemInfo> = entries
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
                folder_name: folder_name.to_string(),
                file_name: entry.file_name().to_string_lossy().to_string(),
                file_path: entry.path().to_string_lossy().to_string(),
                size_bytes,
                modified_time,
                created_time,
                exists: true,
            })
        })
        .collect();

    // Sort files by modification time (most recent first)
    files.sort_by(|a, b| b.modified_time.cmp(&a.modified_time));

    let total_files = files.len();
    let total_size: u64 = files.iter().map(|f| f.size_bytes).sum();

    FolderInfo {
        folder_name: folder_name.to_string(),
        folder_path: full_folder_path.to_string(),
        files,
        total_files,
        total_size_bytes: total_size,
    }
}
