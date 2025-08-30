use crate::cache::{load_cache, save_cache, set_file_info, refresh_cache, CacheInfo};

pub struct CacheManager {
    cache: CacheInfo,
    base_path: String,
}

impl CacheManager {
    pub fn new(base_path: &str, no_cache: bool) -> Self {
        let cache = match no_cache {
            true => CacheInfo::default(),
            false => load_cache(),
        };
        let mut cm = Self { cache, base_path: base_path.to_string() };
        cm.set_file_info();
        
        cm
    }

    pub fn set_file_info(&mut self) {
        set_file_info(&mut self.cache, &self.base_path);
    }

    pub fn get_cache(&self) -> &CacheInfo {
        &self.cache
    }

    pub fn save(&self) {
        save_cache(&self.cache);
    }

    /// Refresh all files marked as NeedsRefresh in the cache
    /// Updates cache entries in memory without saving to disk
    pub fn refresh_cache(&mut self) {        
        refresh_cache(&mut self.cache, &self.base_path);
    }
}

impl Drop for CacheManager {
    fn drop(&mut self) {
        //let _ = save_cache(); // Ignore les erreurs dans Drop
    }
}