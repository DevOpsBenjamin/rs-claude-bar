
use crate::{claude_types::input::ClaudeCodeInput, config::StatusLineConfig};
use std::io::{self, Read};

use crate::{
    analyze::{Analyzer},
    config::{DisplayFormat, StatType}
};


/// Mock data for testing configuration UI
pub struct PromptData {
    pub tokens_used: i64,
    pub tokens_limit: i64,
    pub progress_percent: f64,
    pub time_elapsed_hours: i32,
    pub time_elapsed_minutes: i32,
    pub time_remaining_hours: i32,
    pub time_remaining_minutes: i32,
    pub message_count: i32,
    pub model_name: String,
    pub block_status: String,
    pub is_limited: bool,
}

impl  PromptData {
    pub fn new(analyze: &Analyzer) -> Self {
        // Try to get Claude Code input for model info
        let model_name = parse_claude_input()
            .map(|input| input.model.display_name)
            .unwrap_or_else(|| "Claude".to_string());
        let _current = analyze.get_current();

        Self {
            tokens_used: 100,
            tokens_limit: 1000,
            progress_percent: 100.0/1000.0*100.0,
            time_elapsed_hours: 2,
            time_elapsed_minutes: 12,
            time_remaining_hours: 3,
            time_remaining_minutes: 23,
            message_count: 43,
            model_name: model_name,
            block_status: "ACTIVE".to_string(),
            is_limited: false,
        }
    }
}

impl Default for PromptData {
    fn default() -> Self {
        Self {
            tokens_used: 15234,
            tokens_limit: 28400,
            progress_percent: 53.6,
            time_elapsed_hours: 2,
            time_elapsed_minutes: 15,
            time_remaining_hours: 2,
            time_remaining_minutes: 45,
            message_count: 48,
            model_name: "Claude 3.5 Sonnet".to_string(),
            block_status: "ACTIVE".to_string(),
            is_limited: false,
        }
    }
}

/// Generate a realistic example using mock data
pub fn generate_stat_with_format(_data: &PromptData, _stat_type: &StatType, _format: &DisplayFormat) -> String {
    return "TOTO".to_string();
}

pub fn parse_claude_input() -> Option<ClaudeCodeInput> {
    if !atty::is(atty::Stream::Stdin) {
        let mut input = String::new();
        if io::stdin().read_to_string(&mut input).is_ok() {
            if let Ok(parsed) = serde_json::from_str::<ClaudeCodeInput>(&input) {
                return Some(parsed);
            }
        }
    }
    None
}


pub fn generate_status_line(data: &PromptData, prompt_config: &StatusLineConfig) -> String {
    let prompt_parts: Vec<String> = prompt_config.items.iter()
        .filter(|item| item.enabled)
        .map(|item| generate_stat_with_format(data, &item.stat_type, &item.format))
        .collect();
    
    if prompt_parts.is_empty() {
        "💤 No items configured".to_string()
    } else {
        prompt_parts.join(&prompt_config.separator)
    }
}
