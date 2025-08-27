use std::path::Path;
use std::fs;
use std::env;

fn main() {
    match generate_status() {
        Ok(status) => println!("{}", status),
        Err(e) => println!("🤖 Claude Code | ❌ Error: {}", e),
    }
}

fn generate_status() -> Result<String, Box<dyn std::error::Error>> {
    // Find Claude data directory
    let home = env::var("HOME")?;
    let claude_dir = Path::new(&home).join(".claude").join("projects");
    
    if !claude_dir.exists() {
        return Ok("🤖 Claude Code | ❌ No Claude data found".to_string());
    }
    
    // Count JSONL files as a simple metric
    let mut total_files = 0;
    let mut total_entries = 0;
    
    if let Ok(entries) = fs::read_dir(&claude_dir) {
        for entry in entries.flatten() {
            if let Ok(project_entries) = fs::read_dir(entry.path()) {
                for file in project_entries.flatten() {
                    if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                        total_files += 1;
                        // Count lines in file as entries
                        if let Ok(content) = fs::read_to_string(file.path()) {
                            total_entries += content.lines().count();
                        }
                    }
                }
            }
        }
    }
    
    Ok(format!(
        "🧠 {} entries | 💬 {} files | ⏱️ Active | 🤖 Claude Code",
        total_entries, total_files
    ))
}
