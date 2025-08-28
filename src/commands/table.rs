use chrono::{DateTime, Utc};
use rs_claude_bar::{
    claude_types::TranscriptEntry,
    claudebar_types::{group_by_project, ClaudeBarUsageEntry},
};
use std::fs;
use std::path::Path;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct UsageRow {
    #[tabled(rename = "Session")]
    session_id: String,

    #[tabled(rename = "Time")]
    timestamp: String,

    #[tabled(rename = "Role")]
    role: String,

    #[tabled(rename = "Input")]
    input_tokens: u32,

    #[tabled(rename = "Output")]
    output_tokens: u32,

    #[tabled(rename = "Cache Create")]
    cache_creation: u32,

    #[tabled(rename = "Cache Read")]
    cache_read: u32,

    #[tabled(rename = "Total")]
    total_tokens: u32,

    #[tabled(rename = "Content Len")]
    content_length: usize,

    #[tabled(rename = "Limit Hit")]
    limit_reached: String,

    #[tabled(rename = "Folder")]
    folder: String,

    #[tabled(rename = "File")]
    file: String,
}

pub fn run(config: &rs_claude_bar::ConfigInfo) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);

    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    let mut usage_entries: Vec<ClaudeBarUsageEntry> = Vec::new();

    println!("🔍 Processing JSONL files in {}...", base_path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                println!("📁 Processing folder: {}", folder_name);

                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name().to_string_lossy().to_string();

                            // Get file modification date
                            let file_date = file
                                .metadata()
                                .ok()
                                .and_then(|meta| meta.modified().ok())
                                .and_then(|time| DateTime::<Utc>::from(time).into());

                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for line in content.lines() {
                                    if line.trim().is_empty() {
                                        continue;
                                    }

                                    // Try to parse as TranscriptEntry
                                    if let Ok(transcript) =
                                        serde_json::from_str::<TranscriptEntry>(line)
                                    {
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

    if usage_entries.is_empty() {
        println!("❌ No usage entries found!");
        return;
    }

    // Sort by timestamp (most recent first)
    usage_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    // Project grouping statistics (before moving usage_entries)
    let project_stats = group_by_project(&usage_entries);

    // Convert to table rows
    let table_rows: Vec<UsageRow> = usage_entries
        .into_iter()
        .map(|entry| UsageRow {
            session_id: entry.session_id[..8].to_string(), // Truncate for table
            timestamp: entry.timestamp.format("%m-%d %H:%M").to_string(),
            role: entry.role.to_string(),
            input_tokens: entry.usage.input_tokens,
            output_tokens: entry.usage.output_tokens,
            cache_creation: entry.usage.cache_creation_tokens,
            cache_read: entry.usage.cache_read_tokens,
            total_tokens: entry.usage.total_tokens,
            content_length: entry.content_length,
            limit_reached: if entry.is_limit_reached { "YES" } else { "NO" }.to_string(),
            folder: entry.file_info.folder_name.clone(),
            file: entry.file_info.file_name[..8].to_string(), // Truncate for table
        })
        .collect();

    // Get count before moving table_rows
    let entry_count = table_rows.len();

    // Display table
    let table = Table::new(&table_rows).to_string();
    println!("\n📊 Claude Bar Usage Table ({} entries):", entry_count);
    println!("{}", table);

    // Summary statistics
    let total_tokens: u32 = table_rows.iter().map(|row| row.total_tokens).sum();
    let limit_hits = table_rows
        .iter()
        .filter(|row| row.limit_reached == "YES")
        .count();
    let unique_sessions: std::collections::HashSet<String> = table_rows
        .iter()
        .map(|row| row.session_id.clone())
        .collect();

    println!("\n📈 Summary:");
    println!("  Total Entries: {}", table_rows.len());
    println!("  Total Tokens: {}", total_tokens);
    println!("  Limit Hits: {}", limit_hits);
    println!("  Unique Sessions: {}", unique_sessions.len());

    println!("\n📊 Project Statistics:");
    for project in &project_stats {
        println!("  Project: {}", project.project_name);
        println!(
            "    User     - Entries: {}, Tokens: {}, Content: {} chars",
            project.user_stats.entry_count,
            project.user_stats.total_tokens,
            project.user_stats.total_content_length
        );
        println!(
            "    Assistant - Entries: {}, Tokens: {}, Content: {} chars",
            project.assistant_stats.entry_count,
            project.assistant_stats.total_tokens,
            project.assistant_stats.total_content_length
        );
        println!(
            "    Total    - Entries: {}, Tokens: {}, Content: {} chars",
            project.total_stats.entry_count,
            project.total_stats.total_tokens,
            project.total_stats.total_content_length
        );
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_does_not_panic() {
        let config = rs_claude_bar::ConfigInfo::default();
        run(&config);
    }
}
