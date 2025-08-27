use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{ClaudeBarUsageEntry, UserRole};

/// Statistics grouped by project
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectStats {
    pub project_name: String,
    pub user_stats: RoleStats,
    pub assistant_stats: RoleStats,
    pub total_stats: RoleStats,
}

/// Statistics for a specific role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleStats {
    pub entry_count: usize,
    pub total_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_creation_tokens: u32,
    pub cache_read_tokens: u32,
    pub total_content_length: usize,
}

impl Default for RoleStats {
    fn default() -> Self {
        Self {
            entry_count: 0,
            total_tokens: 0,
            input_tokens: 0,
            output_tokens: 0,
            cache_creation_tokens: 0,
            cache_read_tokens: 0,
            total_content_length: 0,
        }
    }
}

impl RoleStats {
    /// Add an entry to these stats
    pub fn add_entry(&mut self, entry: &ClaudeBarUsageEntry) {
        self.entry_count += 1;
        self.total_tokens += entry.usage.total_tokens;
        self.input_tokens += entry.usage.input_tokens;
        self.output_tokens += entry.usage.output_tokens;
        self.cache_creation_tokens += entry.usage.cache_creation_tokens;
        self.cache_read_tokens += entry.usage.cache_read_tokens;
        self.total_content_length += entry.content_length;
    }
    
    /// Combine with another RoleStats
    pub fn combine(&mut self, other: &RoleStats) {
        self.entry_count += other.entry_count;
        self.total_tokens += other.total_tokens;
        self.input_tokens += other.input_tokens;
        self.output_tokens += other.output_tokens;
        self.cache_creation_tokens += other.cache_creation_tokens;
        self.cache_read_tokens += other.cache_read_tokens;
        self.total_content_length += other.total_content_length;
    }
}

/// Group usage entries by project and calculate statistics
pub fn group_by_project(entries: &[ClaudeBarUsageEntry]) -> Vec<ProjectStats> {
    let mut project_map: HashMap<String, (RoleStats, RoleStats)> = HashMap::new();
    
    for entry in entries {
        let project_name = entry.file_info.folder_name.clone();
        let (user_stats, assistant_stats) = project_map.entry(project_name).or_default();
        
        match entry.role {
            UserRole::User => user_stats.add_entry(entry),
            UserRole::Assistant => assistant_stats.add_entry(entry),
            UserRole::Unknown => {} // Skip unknown roles
        }
    }
    
    // Convert to ProjectStats
    project_map
        .into_iter()
        .map(|(project_name, (user_stats, assistant_stats))| {
            let mut total_stats = user_stats.clone();
            total_stats.combine(&assistant_stats);
            
            ProjectStats {
                project_name,
                user_stats,
                assistant_stats,
                total_stats,
            }
        })
        .collect()
}