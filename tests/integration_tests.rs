use rs_claude_bar::claudebar_types::{group_by_project, ClaudeBarUsageEntry, ProjectStats, RoleStats};
use rs_claude_bar::claude_types::TranscriptEntry;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};

/// Load entries from a test data directory
fn load_test_entries(data_path: &str) -> Vec<ClaudeBarUsageEntry> {
    let mut usage_entries = Vec::new();
    let path = Path::new(data_path);
    
    if !path.exists() {
        return usage_entries;
    }
    
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

#[test]
fn test_small_folder_stats() {
    let entries = load_test_entries("tests/small");
    assert!(!entries.is_empty(), "Should find entries in tests/small");
    
    let project_stats = group_by_project(&entries);
    
    // Expected stats for tests/small (based on current output)
    // These values serve as regression tests
    let expected_stats = vec![
        ProjectStats {
            project_name: "-workspace-git-VueVN".to_string(),
            user_stats: RoleStats {
                entry_count: 3,
                total_tokens: 0,
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                total_content_length: 385, // 128 + 57 + 200
            },
            assistant_stats: RoleStats::default(), // No assistant entries
            total_stats: RoleStats {
                entry_count: 3,
                total_tokens: 0,
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                total_content_length: 385,
            },
        },
        ProjectStats {
            project_name: "-workspace-git-rs-claude-bar".to_string(),
            user_stats: RoleStats {
                entry_count: 2,
                total_tokens: 0,
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                total_content_length: 54, // 5 + 49
            },
            assistant_stats: RoleStats {
                entry_count: 3,
                total_tokens: 16532, // 0 + 0 + 16532
                input_tokens: 4,
                output_tokens: 1,
                cache_creation_tokens: 4783,
                cache_read_tokens: 11744,
                total_content_length: 74, // 36 + 36 + 2
            },
            total_stats: RoleStats {
                entry_count: 5,
                total_tokens: 16532,
                input_tokens: 4,
                output_tokens: 1,
                cache_creation_tokens: 4783,
                cache_read_tokens: 11744,
                total_content_length: 128, // 54 + 74
            },
        },
    ];
    
    println!("Actual project stats: {:#?}", project_stats);
    
    // Verify we have the expected number of projects
    assert_eq!(project_stats.len(), 2, "Should have 2 projects");
    
    // Verify each project has expected stats (order-independent)
    for expected in expected_stats {
        let actual = project_stats.iter()
            .find(|p| p.project_name == expected.project_name)
            .expect(&format!("Project {} not found", expected.project_name));
        
        assert_eq!(actual, &expected, "Stats mismatch for project {}", expected.project_name);
    }
}

#[test]
fn test_data_folder_stats() {
    let entries = load_test_entries("tests/data");
    
    if entries.is_empty() {
        println!("No entries found in tests/data - skipping test");
        return;
    }
    
    let project_stats = group_by_project(&entries);
    
    println!("Data folder project stats: {:#?}", project_stats);
    
    // Basic sanity checks
    assert!(!project_stats.is_empty(), "Should find projects in tests/data");
    
    // Verify each project has consistent totals
    for project in &project_stats {
        assert_eq!(
            project.total_stats.entry_count,
            project.user_stats.entry_count + project.assistant_stats.entry_count,
            "Entry count mismatch for project {}", project.project_name
        );
        
        assert_eq!(
            project.total_stats.total_tokens,
            project.user_stats.total_tokens + project.assistant_stats.total_tokens,
            "Token count mismatch for project {}", project.project_name
        );
        
        assert_eq!(
            project.total_stats.total_content_length,
            project.user_stats.total_content_length + project.assistant_stats.total_content_length,
            "Content length mismatch for project {}", project.project_name
        );
    }
}