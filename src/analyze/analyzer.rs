use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::{
    analyze::{analyze_blocks, DataBlock, BlockKind}, 
    cache::CacheInfo
};

pub struct Analyzer {
    data_blocks: HashMap<DateTime<Utc>, DataBlock>,
}
impl Analyzer {
    pub fn new(cache: &CacheInfo) -> Self {
        // Build blocks (uses internal flattened aggregation privately)
        let data_blocks = analyze_blocks(cache);
        Self { data_blocks }
    }

    /// Return all limit-typed DataBlocks, sorted by end/unlock desc (most recent first)
    pub fn limit_blocks_typed_all(&self) -> Vec<DataBlock> {
        let mut v: Vec<DataBlock> = self
            .data_blocks
            .values()
            .filter(|b| matches!(b.kind, BlockKind::Limit))
            .cloned()
            .collect();
        v.sort_by_key(|b| b.end);
        v.reverse();
        v
    }

    pub fn get_current(&self) -> DataBlock {
        self.data_blocks
            .values()
            .filter(|b| matches!(b.kind, BlockKind::Current))
            .cloned()
            .next()
            .unwrap()
    }

    /// Get all blocks (for debug purposes)
    pub fn all_blocks(&self) -> Vec<&DataBlock> {
        self.data_blocks.values().collect()
    }
}