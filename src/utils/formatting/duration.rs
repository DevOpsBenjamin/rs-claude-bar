/// Format duration in hours and minutes
pub fn format_duration(duration: chrono::Duration) -> String {
    let total_minutes = duration.num_minutes();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    
    if hours > 0 {
        format!("{}h{:02}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}