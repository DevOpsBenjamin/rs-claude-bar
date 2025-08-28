use crate::app_dirs::{ensure_app_dir_exists, get_config_file_path};
use crate::claudebar_types::ConfigInfo;
use std::fs;

/// Initialize and load the configuration
pub fn initialize_config() -> ConfigInfo {
    // Ensure app directory exists
    let _ = ensure_app_dir_exists();
    
    // Try to load existing config, or create default if none exists
    load_or_create_config()
}

/// Load existing config or create a default one
fn load_or_create_config() -> ConfigInfo {
    let config_path = get_config_file_path();
    
    if config_path.exists() {
        // Try to load existing config
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                match serde_json::from_str::<ConfigInfo>(&content) {
                    Ok(config) => return config,
                    Err(_) => {
                        // Config file is corrupted, recreate it
                        let default_config = ConfigInfo::default();
                        let _ = save_config(&default_config);
                        return default_config;
                    }
                }
            }
            Err(_) => {
                // Can't read file, create default
                let default_config = ConfigInfo::default();
                let _ = save_config(&default_config);
                return default_config;
            }
        }
    }
    
    // Config doesn't exist, create default
    let default_config = ConfigInfo::default();
    let _ = save_config(&default_config);
    default_config
}

/// Save configuration to file
pub fn save_config(config: &ConfigInfo) -> std::io::Result<()> {
    let config_path = get_config_file_path();
    let json_content = serde_json::to_string_pretty(config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    
    fs::write(config_path, json_content)
}