mod common;

use crateclaudebar_types::{group_by_project, ProjectStats, RoleStats};

#[test]
fn test_small_folder_stats() {
    let entries = common::load_test_entries("tests/small");
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
            .unwrap_or_else(|| panic!("Project {} not found", expected.project_name));
        
        assert_eq!(actual, &expected, "Stats mismatch for project {}", expected.project_name);
    }
}