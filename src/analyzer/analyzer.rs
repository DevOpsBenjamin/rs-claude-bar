use chrono::{DateTime, Utc};

use crate::{
    claudebar_types::{
        config::ConfigInfo,
        file_info::FileSystemInfo,
        per_hour_log::PerHourLog,
    },
    analyzer::helpers_v2::{
        //parse_file_since_boundary,
        group_entries_by_hour,
        round_to_hour_boundary,
        extract_limit_info,
    }
};

pub struct Analyzer {
    config: ConfigInfo,
}
impl Analyzer {
    /// Create new analyzer with loaded cache and config
    pub fn new(config: ConfigInfo) -> Self {
        Self { config }
    }

    /*
        /// Parse files and update cache with per-hour info
    pub fn parse_and_cache_files(&mut self, files_to_parse: Vec<FileSystemInfo>, no_cache: bool) {
        for file in files_to_parse {
            println!("📝 Parsing file: {}/{}", file.modified_time, file.file_name);
            
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
    */
    /// Get the hour boundary to parse from (previous full hour from cache date)
    fn get_file_cache_boundary(&self, file: &FileSystemInfo) -> Option<DateTime<Utc>> {
        // Find file in cache
        
        /*
        let cached_folder = self.cache.folders.iter()
            .find(|folder| folder.folder_name == file.folder_name)?;
        
        let cached_file = cached_folder.files.iter()
            .find(|f| f.file_name == file.file_name)?;
        
        // Round cache date down to previous hour boundary
        Some(round_to_hour_boundary(cached_file.modified_time))
        */
        None
    }
}