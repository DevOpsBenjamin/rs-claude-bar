/// Format token count in human-readable format (e.g., "1.2M", "500k", "123")
pub fn format_token_count(tokens: u32, size: usize) -> String {
    
    let mut formatted = tokens.to_string();
    if tokens >= 1_000_000 {
        formatted = format!("{:.1}M", tokens as f64 / 1_000_000.0)
    } else if tokens >= 1_000 {
        formatted = format!("{}k", tokens / 1_000)
    }
    format!("{:>width$}", formatted, width = size)
}