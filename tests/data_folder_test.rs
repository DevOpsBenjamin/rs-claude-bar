mod common;

use crateclaudebar_types::{group_by_project, ProjectStats, RoleStats};

#[test]
fn test_data_folder_stats() {
    let entries = common::load_test_entries("tests/data");
    
    if entries.is_empty() {
        println!("No entries found in tests/data - skipping test");
        return;
    }
    
    let project_stats = group_by_project(&entries);
    
    // Expected stats for tests/data (based on actual output)
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
                total_content_length: 385,
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
                entry_count: 90,
                total_tokens: 0,
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                total_content_length: 79750,
            },
            assistant_stats: RoleStats {
                entry_count: 141,
                total_tokens: 10779769,
                input_tokens: 0, // We don't know the breakdown, so setting to 0 for now
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                total_content_length: 12738,
            },
            total_stats: RoleStats {
                entry_count: 231, // 90 + 141
                total_tokens: 10779769,
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_tokens: 0,
                cache_read_tokens: 0,
                total_content_length: 92488, // 79750 + 12738
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
            .unwrap_or_else(|| panic!("Project {} not found", expected.project_name));
        
        // For data folder, we mainly care about the high-level counts since token breakdown varies
        assert_eq!(actual.total_stats.entry_count, expected.total_stats.entry_count, 
                   "Total entry count mismatch for project {}", expected.project_name);
        assert_eq!(actual.total_stats.total_tokens, expected.total_stats.total_tokens, 
                   "Total token count mismatch for project {}", expected.project_name);
        assert_eq!(actual.total_stats.total_content_length, expected.total_stats.total_content_length, 
                   "Total content length mismatch for project {}", expected.project_name);
        
        assert_eq!(actual.user_stats.entry_count, expected.user_stats.entry_count, 
                   "User entry count mismatch for project {}", expected.project_name);
        assert_eq!(actual.assistant_stats.entry_count, expected.assistant_stats.entry_count, 
                   "Assistant entry count mismatch for project {}", expected.project_name);
    }
}