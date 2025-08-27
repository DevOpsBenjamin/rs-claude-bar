use serde::{Deserialize, Serialize};

use super::{
    text_block::TextBlock, thinking_block::ThinkingBlock, tool_result_block::ToolResultBlock,
    tool_use_block::ToolUseBlock,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text(TextBlock),
    Thinking(ThinkingBlock),
    ToolUse(ToolUseBlock),
    ToolResult(ToolResultBlock),
}
