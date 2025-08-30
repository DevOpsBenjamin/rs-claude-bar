use serde::{Deserialize, Serialize};

/// Statistics grouped by project
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ProjectStats {
    pub project_name: String,
    pub user_stats: RoleStats,
    pub assistant_stats: RoleStats,
    pub total_stats: RoleStats,
}

/// Statistics for a specific role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
struct RoleStats {
    pub entry_count: usize,
    pub total_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_creation_tokens: u32,
    pub cache_read_tokens: u32,
    pub total_content_length: usize,
}
