/// Format duration in hours and minutes
pub fn format_duration(duration: chrono::Duration, size: usize) -> String {
    let total_minutes = duration.num_minutes();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    
    let mut formatted = format!("{}m", minutes);
    if hours > 0 {
        formatted = format!("{}h{:02}m", hours, minutes)
    }
    format!("{:>width$}", formatted, width = size)
}