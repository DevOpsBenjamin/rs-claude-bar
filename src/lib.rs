pub mod claude_sdk;
pub mod colors;
pub mod config;
pub mod parser;
pub mod status;
pub mod types;
pub mod utils;

pub use claude_sdk::*;
pub use colors::*;
pub use config::*;
pub use parser::*;
pub use status::*;
pub use types::*;
pub use utils::*;

pub fn generate_claude_status() -> Result<String, Box<dyn std::error::Error>> {
    status::generate_status()
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
