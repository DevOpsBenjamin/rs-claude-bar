use std::fs;
use std::path::PathBuf;
use std::env;

/// Get the application directory path (~/.claude-bar/)
pub fn get_app_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".claude-bar")
}

/// Get the config file path (~/.claude-bar/config.json)
pub fn get_config_file_path() -> PathBuf {
    get_app_dir().join("config.json")
}

/// Ensure the application directory exists
pub fn ensure_app_dir_exists() -> std::io::Result<()> {
    let app_dir = get_app_dir();
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    Ok(())
}

