
// ANSI color and formatting codes - match bash script exactly
// Use explicit escape character (\x1b) so the terminal interprets colors properly
pub const RED: &str = "\x1b[0;31m";
pub const GREEN: &str = "\x1b[0;32m";
pub const YELLOW: &str = "\x1b[1;33m";
pub const BLUE: &str = "\x1b[0;34m";
pub const PURPLE: &str = "\x1b[0;35m";
pub const CYAN: &str = "\x1b[0;36m";
pub const WHITE: &str = "\x1b[1;37m";
pub const GRAY: &str = "\x1b[0;37m";  // Use same gray as bash script
pub const DEFAULT: &str = "\x1b[39m"; // Default foreground color
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

/// Apply color to text
pub fn colorize(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, RESET)
}

/// Create a visual progress bar
pub fn create_progress_bar(percent: f64, width: usize) -> String {
    let filled = ((percent * width as f64) / 100.0) as usize;
    let empty = width - filled;
    
    let mut bar = String::with_capacity(width + 2);
    bar.push('[');
    
    // Use Unicode block characters for filled portions
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in 0..empty {
        bar.push('░');
    }
    
    bar.push(']');
    bar
}

/// Get color for usage percentage
pub fn get_usage_color(percentage: f64) -> &'static str {
    if percentage < 50.0 {
        GREEN
    } else if percentage < 80.0 {
        YELLOW
    } else {
        RED
    }
}

/// Get color for time remaining
pub fn get_time_color(is_active: bool, minutes_remaining: i64) -> &'static str {
    if !is_active {
        RED
    } else if minutes_remaining < 30 {
        YELLOW
    } else {
        GREEN
    }
}

/// Check if colors should be used for status (always true now)
pub fn should_use_colors_for_status() -> bool {
    true
}