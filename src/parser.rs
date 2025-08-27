use std::path::Path;
use std::fs;
use std::env;
use serde_json::Value;
use chrono::{DateTime, Utc, Duration};

use crate::types::{UsageEntry, UsageWindow};
use crate::utils::format_model_name;

/// Parse a single JSONL line into a UsageEntry
pub fn parse_jsonl_entry(line: &str) -> Result<UsageEntry, Box<dyn std::error::Error>> {
    let json: Value = serde_json::from_str(line)
        .map_err(|e| format!("JSON parse error: {}", e))?;
    
    // Extract timestamp
    let timestamp_str = json["timestamp"]
        .as_str()
        .ok_or("Missing timestamp field")?;
    let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
        .map_err(|e| format!("Invalid timestamp format: {}", e))?
        .with_timezone(&Utc);
    
    // Extract session ID
    let session_id = json["sessionId"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    
    // Extract model information - check if this is an assistant message with model info
    let model_id = json["message"]["model"]
        .as_str()
        .or_else(|| json["model"]["id"].as_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Generate display name from model ID
    let model_display_name = if let Some(display) = json["message"]["model_display_name"].as_str()
        .or_else(|| json["model"]["display_name"].as_str()) {
        display.to_string()
    } else {
        // Generate display name from model ID
        match model_id.as_str() {
            id if id.contains("sonnet-4") => "Claude 4 Sonnet".to_string(),
            id if id.contains("sonnet") && id.contains("3.5") => "Claude 3.5 Sonnet".to_string(),
            id if id.contains("sonnet") => "Claude Sonnet".to_string(),
            id if id.contains("opus") => "Claude Opus".to_string(),
            id if id.contains("haiku") => "Claude Haiku".to_string(),
            _ => "Claude".to_string(),
        }
    };
    
    // Extract usage tokens - only present for assistant messages
    let usage = &json["message"]["usage"];
    let input_tokens = usage["input_tokens"].as_u64().unwrap_or(0) as u32;
    let output_tokens = usage["output_tokens"].as_u64().unwrap_or(0) as u32;
    let cache_creation_tokens = usage["cache_creation_input_tokens"].as_u64().unwrap_or(0) as u32;
    let cache_read_tokens = usage["cache_read_input_tokens"].as_u64().unwrap_or(0) as u32;
    let total_tokens = input_tokens + output_tokens + cache_creation_tokens + cache_read_tokens;
    
    // Skip entries with no token usage (usually user messages)
    if total_tokens == 0 {
        return Err("No token usage data (likely user message)".into());
    }
    
    // Extract cost - might not be present in all entries
    let cost_usd = json["costUSD"].as_f64().unwrap_or(0.0);
    
    Ok(UsageEntry {
        timestamp,
        session_id,
        model_id,
        model_display_name,
        input_tokens,
        output_tokens,
        cache_creation_tokens,
        cache_read_tokens,
        total_tokens,
        cost_usd,
    })
}

/// Load all JSONL entries from Claude data directory
pub fn load_claude_data() -> Result<Vec<UsageEntry>, Box<dyn std::error::Error>> {
    // Find Claude data directory
    let home = env::var("HOME")?;
    let claude_dir = Path::new(&home).join(".claude").join("projects");
    
    if !claude_dir.exists() {
        return Err("No Claude data found".into());
    }
    
    // Parse JSONL files and collect usage data
    let mut all_entries = Vec::new();
    let mut _total_files = 0;
    let mut parse_errors = 0;
    
    if let Ok(entries) = fs::read_dir(&claude_dir) {
        for entry in entries.flatten() {
            if let Ok(project_entries) = fs::read_dir(entry.path()) {
                for file in project_entries.flatten() {
                    if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                        _total_files += 1;
                        if let Ok(content) = fs::read_to_string(file.path()) {
                            for (line_num, line) in content.lines().enumerate() {
                                if line.trim().is_empty() {
                                    continue;
                                }
                                match parse_jsonl_entry(line) {
                                    Ok(entry) => all_entries.push(entry),
                                    Err(e) => {
                                        parse_errors += 1;
                                        // Only log first few errors to avoid spam
                                        if parse_errors <= 3 {
                                            eprintln!("Parse error on line {} in {:?}: {}", 
                                                line_num + 1, file.path(), e);
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    if all_entries.is_empty() {
        return Err("No usage data found".into());
    }
    
    Ok(all_entries)
}

/// Group usage entries into 5-hour windows
pub fn group_entries_into_windows(entries: Vec<UsageEntry>) -> Vec<UsageWindow> {
    if entries.is_empty() {
        return Vec::new();
    }
    
    // Sort entries by timestamp
    let mut sorted_entries = entries;
    sorted_entries.sort_by_key(|e| e.timestamp);
    
    let mut windows = Vec::new();
    let mut current_window_entries = Vec::new();
    let mut window_start_time: Option<DateTime<Utc>> = None;
    
    for entry in sorted_entries {
        let entry_time = entry.timestamp;
        
        // Determine if this entry belongs to the current window or starts a new one
        let should_start_new_window = match window_start_time {
            None => true, // First entry
            Some(start) => {
                // Check if entry is more than 5 hours after window start
                entry_time > start + Duration::hours(5)
            }
        };
        
        if should_start_new_window {
            // Finish the previous window if it exists
            if !current_window_entries.is_empty() {
                let window = create_window_from_entries(current_window_entries, window_start_time.unwrap());
                windows.push(window);
                current_window_entries = Vec::new();
            }
            
            // Start new window
            window_start_time = Some(entry_time);
        }
        
        current_window_entries.push(entry);
    }
    
    // Handle the last window
    if !current_window_entries.is_empty() {
        let window = create_window_from_entries(current_window_entries, window_start_time.unwrap());
        windows.push(window);
    }
    
    windows
}

/// Create a UsageWindow from a collection of entries
fn create_window_from_entries(entries: Vec<UsageEntry>, start_time: DateTime<Utc>) -> UsageWindow {
    let total_tokens: u32 = entries.iter().map(|e| e.total_tokens).sum();
    let total_cost: f64 = entries.iter().map(|e| e.cost_usd).sum();
    let message_count = entries.len();
    
    // Collect unique models used in this window
    let mut models_used = Vec::new();
    for entry in &entries {
        let formatted_model = format_model_name(&entry.model_display_name);
        if !models_used.contains(&formatted_model) {
            models_used.push(formatted_model);
        }
    }
    
    let end_time = start_time + Duration::hours(5);
    let now = Utc::now();
    let is_active = now >= start_time && now < end_time;
    
    UsageWindow {
        start_time,
        end_time,
        total_tokens,
        total_cost,
        message_count,
        models_used,
        is_active,
    }
}