pub mod colors;
pub mod types;
pub mod parser;
pub mod status;
pub mod utils;

pub use colors::*;
pub use types::*;
pub use parser::*;
pub use status::*;
pub use utils::*;

pub fn generate_claude_status() -> Result<String, Box<dyn std::error::Error>> {
    status::generate_status()
}

pub fn debug_output() -> String {
    use colors::*;

    format!(
        "{}{}{} {}{}{} {} {}{} TEST{} {} {}{} TIME{} {} {}{} LEFT{} {} {}{} SONNET{}\n",
        BOLD,
        "🧠",
        RESET,
        GRAY,
        "[use /context]",
        RESET,
        format!("{}{}{}", GRAY, "|", RESET),
        BLUE,
        "💬",
        RESET,
        format!("{}{}{}", GRAY, "|", RESET),
        PURPLE,
        "⏱️",
        RESET,
        format!("{}{}{}", GRAY, "|", RESET),
        RED,
        "⏰",
        RESET,
        format!("{}{}{}", GRAY, "|", RESET),
        CYAN,
        "🤖",
        RESET
    )
}