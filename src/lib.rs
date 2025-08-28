pub mod app_dirs;
pub mod claude_types;
pub mod claudebar_types;
pub mod colors;
pub mod config;
pub mod config_manager;
pub mod parser;
pub mod status;
pub mod types;
pub mod utils;
pub mod analyze;

pub use app_dirs::*;
pub use claude_types::*;
pub use claudebar_types::*;
pub use colors::*;
pub use config::{load_config, reset_config_interactive, DisplayItem, Config};
pub use config_manager::*;
pub use parser::*;
pub use status::*;
pub use types::*;
pub use utils::*;
pub use analyze::*;

pub fn generate_claude_status() -> Result<String, Box<dyn std::error::Error>> {
    status::generate_status()
}

pub fn parse_claude_input() -> Option<claude_types::ClaudeCodeInput> {
    use std::io::{self, Read};
    
    if !atty::is(atty::Stream::Stdin) {
        let mut input = String::new();
        if io::stdin().read_to_string(&mut input).is_ok() {
            if let Ok(parsed) = serde_json::from_str::<claude_types::ClaudeCodeInput>(&input) {
                return Some(parsed);
            }
        }
    }
    None
}

pub fn debug_output() -> String {
    use colors::*;

    let sep = format!("{}|{}", GRAY, RESET);
    format!(
        "{}{}{} {}{}{} {} {}{} TEST{} {} {}{} TIME{} {} {}{} LEFT{} {} {}{} SONNET{}\n",
        BOLD,
        "🧠",
        RESET,
        GRAY,
        "[use /context]",
        RESET,
        sep,
        BLUE,
        "💬",
        RESET,
        sep,
        PURPLE,
        "⏱️",
        RESET,
        sep,
        RED,
        "⏰",
        RESET,
        sep,
        CYAN,
        "🤖",
        RESET
    )
}
