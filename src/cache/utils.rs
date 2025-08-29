use std::{collections::{HashMap, hash_map::Entry}, fs, path::{Path, PathBuf}};
use chrono::{DateTime, Utc};

use crate::cache::{CacheInfo, CacheStatus, CachedFile, CachedFolder};

/// Get the path to cache.json in ~/.claude-bar/ directory
/// Returns default path if home directory not found
fn get_cache_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".claude-bar")
        .join("cache.json")
}

/// Load cache from ~/.claude-bar/cache.json
pub fn load_cache() -> CacheInfo {
    let cache_path = get_cache_path();
    
    let content = fs::read_to_string(cache_path)
        .unwrap_or_default();
    
    if content.is_empty() {
        return CacheInfo::default();
    }
    
    serde_json::from_str(&content)
        .unwrap_or_default()
}

/// Save cache to ~/.claude-bar/cache.json
/// Fails silently if cannot save
pub fn save_cache(cache: &CacheInfo) {
    let cache_path = get_cache_path();
    
    // Create directory if it doesn't exist
    if let Some(parent) = cache_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    if let Ok(content) = serde_json::to_string_pretty(cache) {
        let _ = fs::write(cache_path, content);
    }
}

/// Update cache with current folder structure from base_path
/// Creates new entries or updates existing ones
pub fn set_file_info(cache: &mut CacheInfo, base_path: &str) {
    let path: &Path = Path::new(base_path);

    let dir_entries = fs::read_dir(path)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir());
    
    for entry in dir_entries {
        let folder_name = entry.file_name().to_string_lossy().to_string();
        
        // Use HashMap entry API for efficient single lookup
        let cached_folder = cache.folders.entry(folder_name).or_insert_with(|| {
            CachedFolder {
                files: HashMap::new(),
            }
        });
        
        scan_folder(cached_folder, &entry.path());
    }
}

/// Scan a folder and return list of cached files with updated status
fn scan_folder(cached_folder: &mut CachedFolder, folder_path: &Path) {
   let files = fs::read_dir(folder_path)
       .into_iter()
       .flatten()
       .filter_map(|entry| entry.ok())
       .filter(|entry| entry.path().is_file());
   
   for entry in files {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path();
        
        // Get file metadata (fail silently if not accessible)
        let metadata = fs::metadata(&file_path).ok();
        let modified_time = metadata
           .as_ref()
           .and_then(|m| m.modified().ok())
           .map(|t| DateTime::<Utc>::from(t))
           .unwrap_or_else(Utc::now);
        let created_time = metadata
           .as_ref()
           .and_then(|m| m.created().ok())
           .map(|t| DateTime::<Utc>::from(t))
           .unwrap_or_else(Utc::now);
        let size_bytes = metadata.map(|m| m.len()).unwrap_or(0);

        // Use HashMap entry API to distinguish new vs existing files
        match cached_folder.files.entry(file_name.clone()) {
            Entry::Vacant(entry) => {
                // New file not in cache - mark as NotInCache
                entry.insert(CachedFile {
                    file_name,
                    data: Vec::new(),
                    cache_status: CacheStatus::NotInCache,
                    modified_time,
                    created_time,
                    size_bytes,
                });
            },
            Entry::Occupied(mut entry) => {
                // Existing file - compare dates to check if needs refresh
                let cached_file = entry.get_mut();
                cached_file.cache_status = if modified_time > cached_file.modified_time {
                    CacheStatus::NeedsRefresh
                } else {
                    CacheStatus::Fresh
                };
                cached_file.modified_time = modified_time;
                cached_file.created_time = created_time;
                cached_file.size_bytes = size_bytes;
            }
        }
   }
}