#[derive(Debug, PartialEq)]
pub enum BlockStatus {
    NoCurrentBlock,        // No current block exists
    BeforeCurrentBlock,    // Current time is before block start
    InCurrentBlock,        // We're within the current 5-hour block
    NeedNewBlock,         // Current block has ended, need new one
}