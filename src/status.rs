use crate::colors::*;
use crate::parser::{group_entries_into_windows, load_claude_data};
use crate::utils::{format_duration, format_token_count};
use chrono::{Duration, Utc};

/// Generate the complete status line for Claude Code
pub fn generate_status() -> Result<String, Box<dyn std::error::Error>> {
    // Load all entries from Claude data
    let all_entries = load_claude_data()?;

    // Group entries into 5-hour windows
    let windows = group_entries_into_windows(all_entries);

    if windows.is_empty() {
        return Ok("🤖 Claude Code | ❌ No windows found".to_string());
    }

    // Find the active window (current 5-hour window)
    let active_window = windows.iter().find(|w| w.is_active);
    let latest_window = windows.last().unwrap();

    // Use active window if it exists, otherwise show the latest window
    let current_window = active_window.unwrap_or(latest_window);

    // Build the formatted status line
    build_status_line(current_window)
}

/// Build the colorized status line from window data
fn build_status_line(
    window: &crate::types::UsageWindow,
) -> Result<String, Box<dyn std::error::Error>> {
    // Estimate token limit (this should be configurable in the future)
    // Based on typical Claude usage limits - this is an approximation
    let estimated_limit = 28_000_000; // ~28M tokens per 5-hour window

    let usage_percentage = (window.total_tokens as f64 / estimated_limit as f64) * 100.0;

    // Choose appropriate emoji based on usage
    let usage_indicator = if usage_percentage < 50.0 {
        "🟢"
    } else if usage_percentage < 80.0 {
        "🟡"
    } else {
        "🔴"
    };

    // Calculate time in window
    let now = Utc::now();
    let elapsed = if window.is_active {
        now.signed_duration_since(window.start_time)
    } else {
        window.end_time.signed_duration_since(window.start_time)
    };

    let remaining = if window.is_active {
        window.end_time.signed_duration_since(now)
    } else {
        Duration::zero()
    };

    // Format durations
    let elapsed_str = format_duration(elapsed);
    let remaining_str = if window.is_active {
        format!("{} left", format_duration(remaining))
    } else {
        "Complete".to_string()
    };

    // Get primary model used in current window
    let primary_model = window
        .models_used
        .first()
        .map(|s| s.as_str())
        .unwrap_or("Unknown");

    // Choose colors based on usage and time remaining
    let usage_color = get_usage_color(usage_percentage);
    let _time_color = get_time_color(window.is_active, remaining.num_minutes());

    // Create a compact progress bar for token usage
    let progress_bar = create_progress_bar(usage_percentage, 10);
    let colored_progress_bar = colorize(&progress_bar, usage_color);

    // Format tokens in human-readable format
    let tokens_display = format_token_count(window.total_tokens);

    // Build colorized status line similar to bash version
    let mut status = String::new();

    // Brain icon with token usage
    status.push_str(&format!(
        "{}{}{} ",
        if should_use_colors() { BOLD } else { "" },
        "🧠",
        if should_use_colors() { RESET } else { "" }
    ));

    status.push_str(&colorize(&tokens_display, usage_color));
    status.push_str(&format!(" ({:.1}%) {} ", usage_percentage, usage_indicator));
    status.push_str(&colored_progress_bar);

    // Separator
    status.push_str(&format!(" {} ", colorize("|", GRAY)));

    // Message count
    status.push_str(&format!(
        "{} {} ",
        colorize("💬", BLUE),
        colorize(&window.message_count.to_string(), BLUE)
    ));

    // Separator
    status.push_str(&format!(" {} ", colorize("|", GRAY)));

    // Session time
    status.push_str(&format!(
        "{} {} ",
        colorize("⏱️", PURPLE),
        colorize(&elapsed_str, PURPLE)
    ));

    // Separator
    status.push_str(&format!(" {} ", colorize("|", GRAY)));

    // Time remaining - TEST: Bold red to verify ANSI is working
    status.push_str(&format!(
        "{} {}{}{} ",
        colorize("⏰", RED),
        if should_use_colors() {
            format!("{}{}", BOLD, RED)
        } else {
            String::new()
        },
        remaining_str,
        if should_use_colors() { RESET } else { "" }
    ));

    // Separator
    status.push_str(&format!(" {} ", colorize("|", GRAY)));

    // Model
    status.push_str(&format!(
        "{} {}",
        colorize("🤖", CYAN),
        colorize(primary_model, CYAN)
    ));

    Ok(status)
}
