use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::mcp_server_config::McpServerConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpServers {
    Map(HashMap<String, McpServerConfig>),
    Path(PathBuf),
}
