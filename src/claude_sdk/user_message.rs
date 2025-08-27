use serde::{Deserialize, Serialize};

use super::message_content::MessageContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMessage {
    pub content: MessageContent,
}
