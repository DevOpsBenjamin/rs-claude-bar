
pub fn debug_output() -> String {
    use crate::common::colors::*;

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
