use std::{fs, path::PathBuf};

use crate::config::ConfigInfo;

/// Get the path to config.json in ~/.claude-bar/ directory
/// Returns default path if home directory not found
fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".claude-bar")
        .join("config.json")
}


/// Load config from ~/.claude-bar/config.json
pub fn load_config() -> ConfigInfo {
    let config_path = get_config_path();

    let content = fs::read_to_string(config_path)
        .unwrap_or_default();
    
    if content.is_empty() {
        return ConfigInfo::default();
    }
    
    serde_json::from_str(&content)
        .unwrap_or_default()
}

/// Save config to ~/.claude-bar/config.json
/// Fails silently if cannot save
pub fn save_config(config: &ConfigInfo) {
    let config_path = get_config_path();    
    // Create directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(content) = serde_json::to_string_pretty(config) {
        let _ = fs::write(config_path, content);
    }
}
