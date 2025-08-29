use crate::claudebar_types::ConfigInfo;
use serde_json::{json, Value};
use std::fs;

pub fn run(_config: &ConfigInfo) {
    // Determine settings file path
    let mut home = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Could not determine home directory");
            return;
        }
    };
    home.push(".claude");

    let path = home.join("settings.json");

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            eprintln!("Error creating settings directory: {}", err);
            return;
        }
    }

    // Read existing content if file exists
    let contents = fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());
    let mut data: Value = serde_json::from_str(&contents).unwrap_or_else(|_| json!({}));

    // Ensure statusLine section exists
    if !data.get("statusLine").is_some() {
        data["statusLine"] = json!({
            "type": "command",
            "command": "rs-claude-bar prompt",
            "padding": 0
        });
    } else if let Some(obj) = data.get_mut("statusLine").and_then(|v| v.as_object_mut()) {
        obj.insert(
            "command".to_string(),
            Value::String("rs-claude-bar prompt".to_string()),
        );
    }

    // Write back to file
    if let Err(err) = fs::write(&path, serde_json::to_string_pretty(&data).unwrap()) {
        eprintln!("Error writing settings file: {}", err);
    }
}