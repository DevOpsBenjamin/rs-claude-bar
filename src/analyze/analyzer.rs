use chrono::{DateTime, Utc};

use crate::{
    claudebar_types::{
        cache::{Cache, CacheStatus},
        config::ConfigInfo,
        file_info::FileSystemInfo,
        per_hour_log::PerHourLog,
    },
    helpers::{
        cache::{load_cache, save_cache, get_file_cache_status},
        file_system::scan_claude_folders,
    },
    analyze::helpers_v2::{
        parse_file_since_boundary,
        group_entries_by_hour,
        round_to_hour_boundary,
        extract_limit_info,
    }
};

pub struct Analyzer {
    cache: Cache,
    config: ConfigInfo,
}

impl Analyzer {
    /// Create new analyzer with loaded cache and config
    pub fn new(config: ConfigInfo) -> Self {
        let cache = load_cache();
        Self { cache, config }
    }

    /// Public method for debug --files command (reusable)
    pub fn scan_files(&self, base_path: &str) -> Vec<FileSystemInfo> {
        // Reuse existing file scanning logic
        let folders = scan_claude_folders(base_path);
        let mut all_files = Vec::new();
        
        for folder in folders {
            all_files.extend(folder.files);
        }
        
        all_files
    }

    /// Analyze files and determine which need parsing
    pub fn analyze_files(&mut self, base_path: &str) -> (Vec<FileSystemInfo>, Vec<FileSystemInfo>) {
        let files = self.scan_files(base_path);
        let mut needs_parsing = Vec::new();
        let mut cached_files = Vec::new();
        
        for file in files {
            let cache_status = get_file_cache_status(&file, &self.cache);
            
            match cache_status {
                CacheStatus::Fresh => {
                    cached_files.push(file);
                }
                CacheStatus::NeedsRefresh | CacheStatus::NotInCache => {
                    needs_parsing.push(file);
                }
            }
        }
        
        (needs_parsing, cached_files)
    }

    /// Parse files and update cache with per-hour info
    pub fn parse_and_cache_files(&mut self, files_to_parse: Vec<FileSystemInfo>, no_cache: bool) {
        for file in files_to_parse {
            println!("📝 Parsing file: {}/{}", file.folder_name, file.file_name);
            
            // Get cache date (rounded to previous hour boundary) 
            let cache_boundary = if no_cache { None } else { self.get_file_cache_boundary(&file) };
            
            // Parse only entries after cache boundary (or all if no_cache)
            let new_entries = parse_file_since_boundary(&file, cache_boundary);
            
            if new_entries.is_empty() {
                println!("   ✅ No new entries to process");
                continue;
            }
            
            // Group entries by hour and create PerHourLog summaries
            let hour_logs = group_entries_by_hour(new_entries.clone());
            
            // Extract limit information from the same entries
            let limit_infos = extract_limit_info(&new_entries);
            
            println!("   📊 Created {} hour logs, {} limit entries", hour_logs.len(), limit_infos.len());
            
            // TODO: Store hour_logs and limit_infos in cache.infos for this file
            // TODO: Merge with existing cached data
            
            // TODO: Uncomment when feature is ready
            // Update file cache date to current file modification time
            // update_file_in_cache(
            //     &mut self.cache,
            //     &file.folder_name,
            //     &file.file_name,
            //     file.modified_time,
            // );
        }
    }
    
    /// Get the hour boundary to parse from (previous full hour from cache date)
    fn get_file_cache_boundary(&self, file: &FileSystemInfo) -> Option<DateTime<Utc>> {
        // Find file in cache
        let cached_folder = self.cache.folders.iter()
            .find(|folder| folder.folder_name == file.folder_name)?;
        
        let cached_file = cached_folder.files.iter()
            .find(|f| f.file_name == file.file_name)?;
        
        // Round cache date down to previous hour boundary
        Some(round_to_hour_boundary(cached_file.cache_date))
    }

    /// Save updated cache to disk
    pub fn save_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        save_cache(&self.cache)
    }

    /// Get cache reference for external use
    pub fn get_cache(&self) -> &Cache {
        &self.cache
    }
}