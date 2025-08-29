use chrono::{DateTime, Utc};
use std::fs;
use std::path::PathBuf;
use crate::claudebar_types::{
    cache::{Cache, CachedFolder, CachedFile, CacheStatus},
    file_info::{FileSystemInfo}
};

/// Determine cache status for a file by comparing file modification time with cached date
pub fn get_file_cache_status(file_info: &FileSystemInfo, cache: &Cache) -> CacheStatus {
    cache
        .folders
        .iter()
        .find(|folder| folder.folder_name == file_info.folder_name)
        .and_then(|folder| {
            folder
                .files
                .iter()
                .find(|file| file.file_name == file_info.file_name)
        })
        .map(|file| {
            if file_info.modified_time > file.cache_date {
                CacheStatus::NeedsRefresh
            } else {
                CacheStatus::Fresh
            }
        })
        .unwrap_or(CacheStatus::NotInCache)
}

/// Update cache with new file information (used by parse operations, not by debug --files)
pub fn update_file_in_cache(cache: &mut Cache, folder_name: &str, file_name: &str, modified_time: DateTime<Utc>) {
    // Find or create folder
    let folder = cache.folders.iter_mut()
        .find(|f| f.folder_name == folder_name);
    
    if let Some(folder) = folder {
        // Find or create file
        let file = folder.files.iter_mut()
            .find(|f| f.file_name == file_name);
        
        if let Some(file) = file {
            // Update existing file
            file.cache_date = modified_time;
        } else {
            // Add new file
            folder.files.push(CachedFile {
                file_name: file_name.to_string(),
                cache_date: modified_time,
                infos: Vec::new(),
            });
        }
    } else {
        // Add new folder with file
        cache.folders.push(CachedFolder {
            folder_name: folder_name.to_string(),
            files: vec![CachedFile {
                file_name: file_name.to_string(),
                cache_date: modified_time,
                infos: Vec::new(),
            }],
        });
    }
    
    // Update cache timestamp
    cache.last_updated = Utc::now();
}
