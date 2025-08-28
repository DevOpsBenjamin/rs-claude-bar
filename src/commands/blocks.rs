use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc, Duration};
use regex::Regex;
use rs_claude_bar::{claude_types::TranscriptEntry, claudebar_types::ClaudeBarUsageEntry, colors::*};

#[derive(Debug, Clone)]
pub struct UsageBlock {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub entries: Vec<ClaudeBarUsageEntry>,
    pub assistant_count: usize,
    pub limit_reached: bool,
    pub reset_time: Option<String>, // e.g., "10pm", "11pm"
    pub unlock_time: Option<DateTime<Utc>>, // calculated unlock timestamp
}

pub fn run(config: &rs_claude_bar::ConfigInfo) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);
    
    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }
    
    println!("{bold}{cyan}📊 5-Hour Usage Blocks Analysis{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        cyan = if should_use_colors() { CYAN } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
    );
    println!();
    
    // Load all entries
    let mut all_entries = load_all_entries(&base_path);
    if all_entries.is_empty() {
        println!("❌ No usage entries found!");
        return;
    }
    
    // Sort by timestamp (ascending for analysis)
    all_entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    
    println!("📈 Loaded {} entries from {} to {}", 
             all_entries.len(),
             all_entries.first().unwrap().timestamp.format("%Y-%m-%d %H:%M UTC"),
             all_entries.last().unwrap().timestamp.format("%Y-%m-%d %H:%M UTC"));
    println!();
    
    // Find limit reached messages and build blocks
    let mut blocks = analyze_usage_blocks(&all_entries);
    
    // Sort blocks by start time descending (most recent first)
    blocks.sort_by(|a, b| b.start_time.cmp(&a.start_time));
    
    println!("🔍 Found {} usage blocks:", blocks.len());
    println!();
    
    // Display blocks
    for (i, block) in blocks.iter().enumerate() {
        let limit_indicator = if block.limit_reached {
            format!("{red}🔴 LIMIT HIT{reset}", red = RED, reset = RESET)
        } else {
            format!("{green}🟢 ACTIVE{reset}", green = GREEN, reset = RESET)
        };
        
        let duration = block.end_time.signed_duration_since(block.start_time);
        let duration_str = format_duration_hours(duration);
        
        println!("Block {}: {} - {}", 
                 i + 1,
                 block.start_time.format("%Y-%m-%d %H:%M UTC"),
                 block.end_time.format("%Y-%m-%d %H:%M UTC"));
        println!("  Duration: {} | Assistant messages: {} | Status: {}", 
                 duration_str,
                 block.assistant_count,
                 limit_indicator);
        
        // Show reset time and unlock time for limit-reached blocks
        if block.limit_reached {
            if let Some(reset_time) = &block.reset_time {
                print!("  Reset time: {}", reset_time);
                if let Some(unlock_time) = &block.unlock_time {
                    println!(" | Unlocks at: {}", unlock_time.format("%Y-%m-%d %H:%M UTC"));
                } else {
                    println!(" | Unlock time: could not calculate");
                }
            } else {
                println!("  Reset time: not found in limit message");
            }
        }
        
        println!("  Total entries: {}", block.entries.len());
        println!();
    }
}

fn load_all_entries(base_path: &str) -> Vec<ClaudeBarUsageEntry> {
    let mut usage_entries = Vec::new();
    let path = Path::new(base_path);
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                
                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name().to_string_lossy().to_string();
                            
                            // Get file modification date
                            let file_date = file.metadata()
                                .ok()
                                .and_then(|meta| meta.modified().ok())
                                .and_then(|time| DateTime::<Utc>::from(time).into());
                            
                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for line in content.lines() {
                                    if line.trim().is_empty() {
                                        continue;
                                    }
                                    
                                    if let Ok(transcript) = serde_json::from_str::<TranscriptEntry>(line) {
                                        let usage_entry = ClaudeBarUsageEntry::from_transcript(
                                            &transcript,
                                            folder_name.clone(),
                                            file_name.clone(),
                                            file_date,
                                        );
                                        usage_entries.push(usage_entry);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    usage_entries
}

fn analyze_usage_blocks(entries: &[ClaudeBarUsageEntry]) -> Vec<UsageBlock> {
    let mut blocks = Vec::new();
    let mut current_block_entries: Vec<ClaudeBarUsageEntry> = Vec::new();
    let mut block_start: Option<DateTime<Utc>> = None;
    let mut previous_was_user = false;
    
    for entry in entries {
        let is_user = matches!(entry.role, rs_claude_bar::claudebar_types::UserRole::User);
        let is_assistant = matches!(entry.role, rs_claude_bar::claudebar_types::UserRole::Assistant);
        
        // Detect limit: when we see assistant messages after user input and is_limit_reached is true
        let is_limit_hit = entry.is_limit_reached && is_assistant && previous_was_user;
        
        if is_limit_hit {
            
            // Add current entry to block before ending it
            current_block_entries.push(entry.clone());
            
            // End current block if we have one
            if !current_block_entries.is_empty() && block_start.is_some() {
                let assistant_count = current_block_entries
                    .iter()
                    .filter(|e| matches!(e.role, rs_claude_bar::claudebar_types::UserRole::Assistant))
                    .count();
                
                // Parse reset time from the limit message content
                let content_text = get_entry_content_text(entry);
                let reset_time = parse_reset_time(&content_text);
                let unlock_time = reset_time.as_ref()
                    .and_then(|rt| calculate_unlock_time(entry.timestamp, rt));
                
                blocks.push(UsageBlock {
                    start_time: block_start.unwrap(),
                    end_time: entry.timestamp,
                    entries: current_block_entries.clone(),
                    assistant_count,
                    limit_reached: true,
                    reset_time,
                    unlock_time,
                });
                
                current_block_entries.clear();
            }
            
            // Start new block after limit reset (estimate 5 hours later)
            block_start = Some(entry.timestamp + Duration::hours(5));
            previous_was_user = false;
            continue;
        }
        
        // Start first block if we haven't started yet
        if block_start.is_none() {
            block_start = Some(entry.timestamp);
        }
        
        current_block_entries.push(entry.clone());
        
        // Track if this was a user message (ignore assistant messages for this tracking)
        if is_user {
            previous_was_user = true;
        }
        // Don't reset previous_was_user for consecutive assistant messages
    }
    
    // Handle remaining entries as final block
    if !current_block_entries.is_empty() && block_start.is_some() {
        let assistant_count = current_block_entries
            .iter()
            .filter(|e| matches!(e.role, rs_claude_bar::claudebar_types::UserRole::Assistant))
            .count();
        
        // End time is last entry timestamp
        let end_time = current_block_entries.last().unwrap().timestamp;
        
        blocks.push(UsageBlock {
            start_time: block_start.unwrap(),
            end_time,
            entries: current_block_entries,
            assistant_count,
            limit_reached: false,
            reset_time: None,
            unlock_time: None,
        });
    }
    
    blocks
}

/// Get content text from a ClaudeBarUsageEntry
fn get_entry_content_text(entry: &ClaudeBarUsageEntry) -> String {
    entry.content_text.clone().unwrap_or_default()
}

/// Parse reset time from limit message content
fn parse_reset_time(content: &str) -> Option<String> {
    // Pattern for "resets 10pm" (with bullet separator ∙)
    let re = Regex::new(r"(?i)resets?\s+(\d{1,2}(?::\d{2})?\s*(?:am|pm))").ok()?;
    if let Some(caps) = re.captures(content) {
        return Some(caps[1].to_lowercase());
    }
    
    // Pattern for "resets at 10pm"
    let re2 = Regex::new(r"(?i)resets?\s+at\s+(\d{1,2}(?::\d{2})?\s*(?:am|pm))").ok()?;
    if let Some(caps) = re2.captures(content) {
        return Some(caps[1].to_lowercase());
    }
    
    // Try alternative patterns like "until 10pm" or "at 10pm"
    let re3 = Regex::new(r"(?i)(?:until|at)\s+(\d{1,2}(?::\d{2})?\s*(?:am|pm))").ok()?;
    if let Some(caps) = re3.captures(content) {
        return Some(caps[1].to_lowercase());
    }
    
    None
}

/// Calculate unlock time based on limit timestamp and reset time
fn calculate_unlock_time(limit_timestamp: DateTime<Utc>, reset_time: &str) -> Option<DateTime<Utc>> {
    // Parse the reset time (e.g., "10pm", "10:30pm")
    let re = Regex::new(r"(\d{1,2})(?::(\d{2}))?\s*(am|pm)").ok()?;
    let caps = re.captures(reset_time)?;
    
    let hour: u32 = caps[1].parse().ok()?;
    let minute: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let is_pm = caps[3].eq_ignore_ascii_case("pm");
    
    // Convert to 24-hour format
    let hour_24 = match (hour, is_pm) {
        (12, false) => 0,  // 12am -> 0
        (12, true) => 12,  // 12pm -> 12
        (h, false) => h,   // am hours
        (h, true) => h + 12, // pm hours
    };
    
    if hour_24 >= 24 || minute >= 60 {
        return None;
    }
    
    // Get the date of the limit timestamp
    let limit_date = limit_timestamp.date_naive();
    
    // Create reset time on the same day
    let reset_time_same_day = limit_date.and_hms_opt(hour_24, minute, 0)?
        .and_local_timezone(Utc).single()?;
    
    // If the reset time already passed today, it's tomorrow
    let unlock_time = if reset_time_same_day > limit_timestamp {
        reset_time_same_day
    } else {
        // Add one day
        (limit_date + Duration::days(1)).and_hms_opt(hour_24, minute, 0)?
            .and_local_timezone(Utc).single()?
    };
    
    Some(unlock_time)
}

fn format_duration_hours(duration: Duration) -> String {
    let total_hours = duration.num_hours();
    let minutes = (duration.num_minutes() % 60).abs();
    
    if total_hours == 0 {
        format!("{}m", minutes)
    } else {
        format!("{}h {}m", total_hours, minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_does_not_panic() {
        run(Some("nonexistent"));
    }
}