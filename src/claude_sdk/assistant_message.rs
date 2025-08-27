use serde::{Deserialize, Serialize};

use super::content_block::ContentBlock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantMessage {
    pub content: Vec<ContentBlock>,
    pub model: String,
}
