use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::{
    analyze::{analyze_blocks, DataBlock, BlockKind}, 
    cache::CacheInfo
};

pub struct Analyzer {
    data_blocks: HashMap<DateTime<Utc>, DataBlock>,
    /// Mean output tokens after p20-p80 removal from limit blocks
    output_token_max: i64,
}
impl Analyzer {
    pub fn new(cache: &CacheInfo) -> Self {
        // Build blocks (uses internal flattened aggregation privately)
        let data_blocks = analyze_blocks(cache);
        
        // Calculate output_token_max from limit blocks using p20-p80 removal
        let mut output_counts: Vec<i64> = data_blocks.values()
            .filter(|b| matches!(b.kind, BlockKind::Limit))
            .map(|b| b.stats.output_tokens)
            .collect();
        
        let output_token_max = if output_counts.is_empty() {
            0
        } else {
            output_counts.sort_unstable();
            let len = output_counts.len();
            let p20_idx = (len as f64 * 0.2) as usize;
            let p80_idx = (len as f64 * 0.8) as usize;
            
            // Remove p20 and p80 outliers, keep middle 60%
            let trimmed = &output_counts[p20_idx..p80_idx.min(len)];
            
            if trimmed.is_empty() {
                output_counts[0] // fallback if too few samples
            } else {
                trimmed.iter().sum::<i64>() / trimmed.len() as i64
            }
        };
        
        Self { 
            data_blocks,
            output_token_max,
        }
    }

    /// Return all limit-typed DataBlocks, sorted by end/unlock desc (most recent first)
    pub fn blocks_typed_all(&self) -> Vec<DataBlock> {
        let mut v: Vec<DataBlock> = self
            .data_blocks
            .values()
            .cloned()
            .collect();
        v.sort_by_key(|b| b.end);
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
    
    /// Get calculated output token max (mean after p20-p80 removal)
    pub fn output_token_max(&self) -> i64 {
        self.output_token_max
    }
}