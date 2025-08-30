use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct LimitBlock {
    /// Timestamp when the block was lifted/reset (if available)
    pub unlock_timestamp: DateTime<Utc>,
    pub datas: BlockData,
}

#[derive(Debug, Clone)]
pub struct BlockData {
}
impl Default for BlockData {
    fn default() -> Self {
        BlockData {
        }
    }
}