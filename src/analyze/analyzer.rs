use crate::cache::CacheInfo;

pub struct Analyzer {
    cache: CacheInfo
}
impl Analyzer {
    pub fn new(cache: CacheInfo) -> Self {
        Self { cache }
    }
}