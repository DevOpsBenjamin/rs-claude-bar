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
}   