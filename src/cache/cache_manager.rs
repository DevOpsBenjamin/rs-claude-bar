use crate::cache::{load_cache, set_file_info, CacheInfo};

pub struct CacheManager {
    cache: CacheInfo,
}

impl CacheManager {
    pub fn new(base_path: &str) -> Self {
        let mut cache = load_cache();
        set_file_info(&mut cache, base_path);
        Self { cache }
    }

    pub fn set_file_info(&mut self, base_path: &str) {
        set_file_info(&mut self.cache, base_path);
    }

    pub fn get_cache(&self) -> &CacheInfo {
        &self.cache
    }
}

impl Drop for CacheManager {
    fn drop(&mut self) {
        //let _ = save_cache(); // Ignore les erreurs dans Drop
    }
}