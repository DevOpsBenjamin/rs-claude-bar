use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayItem {
    pub name: String,
    pub enabled: bool,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub display: Vec<DisplayItem>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: vec![
                DisplayItem {
                    name: "context_window_size".to_string(),
                    enabled: true,
                    format: "progress".to_string(),
                },
                DisplayItem {
                    name: "context_max_tokens".to_string(),
                    enabled: true,
                    format: "number".to_string(),
                },
                DisplayItem {
                    name: "current_tokens".to_string(),
                    enabled: true,
                    format: "number/max".to_string(),
                },
                DisplayItem {
                    name: "max_tokens".to_string(),
                    enabled: true,
                    format: "number".to_string(),
                },
                DisplayItem {
                    name: "refresh_time".to_string(),
                    enabled: true,
                    format: "relative".to_string(),
                },
                DisplayItem {
                    name: "session_time".to_string(),
                    enabled: true,
                    format: "relative".to_string(),
                },
            ],
        }
    }
}

fn config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".claude-bar");
    path.push("config.json");
    path
}

pub fn load_config() -> Config {
    let path = config_path();
    if let Ok(contents) = fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<Config>(&contents) {
            return cfg;
        }
    }
    let cfg = Config::default();
    let _ = save_config(&cfg);
    cfg
}

pub fn save_config(cfg: &Config) -> std::io::Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let contents = serde_json::to_string_pretty(cfg).unwrap();
    fs::write(path, contents)
}

// display-config
pub fn reset_config_interactive() -> Config {
    let mut cfg = Config::default();
    for item in cfg.display.iter_mut() {
        print!("Show {}? (y/n) ", item.name);
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            item.enabled = input.trim().to_lowercase() != "n";
        }
        print!("Format for {} [{}]: ", item.name, item.format);
        let _ = io::stdout().flush();
        input.clear();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if !trimmed.is_empty() {
                item.format = trimmed.to_string();
            }
        }
    }
    let _ = save_config(&cfg);
    cfg
}
