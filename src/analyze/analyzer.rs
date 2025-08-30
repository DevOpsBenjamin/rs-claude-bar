use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::{
    analyze::{build_limit_blocks, LimitBlock}, 
    cache::CacheInfo
};

pub struct Analyzer {
    limit_blocks: HashMap<DateTime<Utc>, LimitBlock>
}
impl Analyzer {
    pub fn new(cache: &CacheInfo) -> Self {
        let limit_blocks = build_limit_blocks(cache);
        Self { limit_blocks }
    }

    /// Return all limit blocks as a vector of (start, LimitBlock),
    /// sorted by unlock time descending (most recent first).
    pub fn limit_blocks_all(&self) -> Vec<(DateTime<Utc>, LimitBlock)> {
        let mut items: Vec<(DateTime<Utc>, LimitBlock)> = self
            .limit_blocks
            .iter()
            .map(|(start, lb)| (*start, lb.clone()))
            .collect();

        // Sort by unlock timestamp descending (most recent first)
        items.sort_by_key(|(_, lb)| lb.unlock_timestamp);
        items.reverse();
        items
    }
}
