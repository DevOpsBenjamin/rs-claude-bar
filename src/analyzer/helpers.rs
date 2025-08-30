use chrono::{DateTime, Utc};
use crate::claudebar_types::config::SimpleBlock;

/// Detect if we're in a current 5-hour block or have moved to a new one
pub fn detect_block_status(now: DateTime<Utc>, current: &Option<SimpleBlock>) -> BlockStatus {
    match current {
        None => BlockStatus::NoCurrentBlock,
        Some(block) => {
            if now < block.start {
                // We're before the block starts (shouldn't happen normally)
                BlockStatus::BeforeCurrentBlock
            } else if now >= block.start && now <= block.end {
                // We're within the current block
                BlockStatus::InCurrentBlock
            } else {
                // We're past the current block - need a new one
                BlockStatus::NeedNewBlock
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockStatus {
    NoCurrentBlock,        // No current block exists
    BeforeCurrentBlock,    // Current time is before block start
    InCurrentBlock,        // We're within the current 5-hour block
    NeedNewBlock,         // Current block has ended, need new one
}