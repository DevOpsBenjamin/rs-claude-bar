use serde::{Deserialize, Serialize};

use super::{
    mcp_http_server_config::McpHttpServerConfig, mcp_sse_server_config::McpSseServerConfig,
    mcp_stdio_server_config::McpStdioServerConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpServerConfig {
    Stdio(McpStdioServerConfig),
    Sse(McpSseServerConfig),
    Http(McpHttpServerConfig),
}
