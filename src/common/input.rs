use crate::claude_types::input::ClaudeCodeInput;
use std::io::{self, Read};

pub fn generate_claude_status() -> Result<String, Box<dyn std::error::Error>> {
    crate::status::generate_status()
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
