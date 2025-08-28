use serde::{Deserialize, Serialize};

/// Claude Code input parameters passed via stdin when used as status line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeInput {
    pub session_id: String,
    pub transcript_path: String,
    pub cwd: String,
    pub model: ClaudeCodeModel,
    pub workspace: ClaudeCodeWorkspace,
    pub version: String,
    pub output_style: ClaudeCodeOutputStyle,
    pub cost: ClaudeCodeCost,
    pub exceeds_200k_tokens: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeModel {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeWorkspace {
    pub current_dir: String,
    pub project_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeOutputStyle {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeCost {
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub total_api_duration_ms: u64,
    pub total_lines_added: u32,
    pub total_lines_removed: u32,
}