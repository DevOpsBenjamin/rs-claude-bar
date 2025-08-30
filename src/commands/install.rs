use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
struct Settings {
    #[serde(default)]
    #[serde(rename = "statusLine")]
    status_line: StatusLine,
    // keep any extra keys we don't model
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StatusLine {
    #[serde(rename = "type")]
    kind: String,
    command: String,
    padding: u32,
}
impl Default for StatusLine {
    fn default() -> Self {
        Self {
            kind: "command".into(),
            command: "rs-claude-bar prompt".into(),
            padding: 0,
        }
    }
}

fn settings_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".claude")
        .join("settings.json")
}

pub fn run() {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Load or default
    let mut settings: Settings = fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    // Ensure the command is what we want
    settings.status_line.command = "rs-claude-bar prompt".into();

    // Save
    let json = serde_json::to_string_pretty(&settings).unwrap();
    let _ = fs::write(path, json);
}
