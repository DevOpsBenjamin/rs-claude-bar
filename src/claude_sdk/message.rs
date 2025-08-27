use serde::{Deserialize, Serialize};

use super::{
    assistant_message::AssistantMessage, result_message::ResultMessage,
    system_message::SystemMessage, user_message::UserMessage,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "message", rename_all = "lowercase")]
pub enum Message {
    User(UserMessage),
    Assistant(AssistantMessage),
    System(SystemMessage),
    Result(ResultMessage),
}
