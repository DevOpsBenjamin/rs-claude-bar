use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::display::status_config::StatusLineConfig;

/// Stats file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsFile {
    /// Last processed timestamp
    pub last_processed: Option<DateTime<Utc>>,
}

/// Main configuration for Claude Bar application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
    /// Version of the config format
    pub version: String,
    
    /// Path to Claude Code data directory
    pub claude_data_path: String,
    
    /// Display preferences
    pub display: StatusLineConfig,
}