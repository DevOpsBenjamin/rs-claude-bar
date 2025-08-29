use std::{fs, path::Path};
use chrono::{DateTime, Utc};

use crate::claudebar_types::file_info::{FileSystemInfo, FolderInfo};


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