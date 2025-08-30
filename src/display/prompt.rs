use std::{io::{self, Read}, thread::current};

use crate::{
    claude_types::input::ClaudeCodeInput, 
    config::StatusLineConfig, 
    display::generate_stat_with_format,
    analyze::{Analyzer}
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
    pub message_count: i64,
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
        let current = analyze.get_current();
        let current_token = current.stats.output_tokens;
        let max_token = analyze.output_token_max();
        let percent = 100.0 * current_token as f64 / max_token as f64;
        Self {
            tokens_used: current.stats.output_tokens,
            tokens_limit: max_token,
            progress_percent: percent,
            time_elapsed_hours: 2,
            time_elapsed_minutes: 12,
            time_remaining_hours: 3,
            time_remaining_minutes: 23,
            message_count: current.stats.assistant_messages + current.stats.user_messages,
            model_name: model_name,
            block_status: "ACTIVE".to_string(),
            is_limited: current.unlock_timestamp.is_some(),
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
