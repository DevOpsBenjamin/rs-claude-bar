/// Format a UTC datetime using "%m-%d %H:%M", right-aligned to `size` width
pub fn format_text(text: &str, size: usize) -> String {
    format!("{:>width$}", text, width = size)
}