use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::{
    analyze::{build_limit_blocks_with_agg, build_per_hour_agg, LimitBlock, BlockData}, 
    cache::CacheInfo
};

pub struct Analyzer {
    limit_blocks: HashMap<DateTime<Utc>, LimitBlock>,
    per_hour_agg: HashMap<DateTime<Utc>, BlockData>,
}
impl Analyzer {
    pub fn new(cache: &CacheInfo) -> Self {
        // Build flattened aggregates once; reuse across analyses
        let per_hour_agg = build_per_hour_agg(cache);
        // Build limit windows using the aggregated per-hour data
        let limit_blocks = build_limit_blocks_with_agg(cache, &per_hour_agg);
        Self { limit_blocks, per_hour_agg }
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

    /// Immutable view of limit blocks map (for callers that want direct access)
    pub fn limit_blocks_map(&self) -> &HashMap<DateTime<Utc>, LimitBlock> {
        &self.limit_blocks
    }

    /// Immutable view of per-hour aggregated usage across all files
    pub fn per_hour_aggregate(&self) -> &HashMap<DateTime<Utc>, BlockData> {
        &self.per_hour_agg
    }
}
