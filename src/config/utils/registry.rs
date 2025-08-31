use std::collections::HashMap;

use crate::{config::StatType, config::DisplayFormat};

/// Definition of a metric and its supported formats
#[derive(Debug, Clone)]
pub struct MetricDefinition {
    pub stat_type: StatType,
    pub name: String,
    pub description: String,
    pub supported_formats: Vec<DisplayFormat>,
    pub default_format: DisplayFormat,
    pub enabled_by_default: bool,
}

/// Registry of all available metrics
pub struct MetricRegistry {
    metrics: HashMap<StatType, MetricDefinition>,
}

impl MetricRegistry {
    pub fn new() -> Self {
        let mut metrics = HashMap::new();
        
        // Token metrics
        metrics.insert(StatType::TokenUsage, MetricDefinition {
            stat_type: StatType::TokenUsage,
            name: "Token Usage".to_string(),
            description: "Current token count in active block".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji, 
                DisplayFormat::Compact,
                DisplayFormat::Ratio,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::TokenProgress, MetricDefinition {
            stat_type: StatType::TokenProgress,
            name: "Token Progress".to_string(), 
            description: "Progress through current block limit".to_string(),
            supported_formats: vec![
                DisplayFormat::ProgressBar,
                DisplayFormat::PercentageOnly,
                DisplayFormat::StatusColored,
            ],
            default_format: DisplayFormat::PercentageOnly,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::TimeElapsed, MetricDefinition {
            stat_type: StatType::TimeElapsed,
            name: "Time Elapsed".to_string(),
            description: "Time spent in current block".to_string(),
            supported_formats: vec![
                DisplayFormat::Duration,
                DisplayFormat::DurationShort,
                DisplayFormat::Text,
            ],
            default_format: DisplayFormat::Duration,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::TimeRemaining, MetricDefinition {
            stat_type: StatType::TimeRemaining,
            name: "Time Remaining".to_string(),
            description: "Time left in current block".to_string(),
            supported_formats: vec![
                DisplayFormat::Duration,
                DisplayFormat::DurationShort,
                DisplayFormat::Text,
            ],
            default_format: DisplayFormat::Duration,
            enabled_by_default: false,
        });
        
        metrics.insert(StatType::MessageCount, MetricDefinition {
            stat_type: StatType::MessageCount,
            name: "Message Count".to_string(),
            description: "Total messages in current block".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji,
                DisplayFormat::Compact,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::Model, MetricDefinition {
            stat_type: StatType::Model,
            name: "Model Name".to_string(),
            description: "Current Claude model being used".to_string(),
            supported_formats: vec![
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji,
                DisplayFormat::Compact,
            ],
            default_format: DisplayFormat::TextWithEmoji,
            enabled_by_default: true,
        });
        
        metrics.insert(StatType::BlockStatus, MetricDefinition {
            stat_type: StatType::BlockStatus,
            name: "Block Status".to_string(),
            description: "Current block type (Active/Limited/Gap)".to_string(),
            supported_formats: vec![
                DisplayFormat::StatusIcon,
                DisplayFormat::StatusText,
                DisplayFormat::StatusColored,
            ],
            default_format: DisplayFormat::StatusIcon,
            enabled_by_default: false,
        });
        
        metrics.insert(StatType::ActivityStatus, MetricDefinition {
            stat_type: StatType::ActivityStatus,
            name: "Activity Status".to_string(),
            description: "Overall activity indicator".to_string(),
            supported_formats: vec![
                DisplayFormat::StatusIcon,
                DisplayFormat::StatusText,
            ],
            default_format: DisplayFormat::StatusIcon,
            enabled_by_default: false,
        });
        
        Self { metrics }
    }
    
    pub fn get_metric(&self, stat_type: &StatType) -> Option<&MetricDefinition> {
        self.metrics.get(stat_type)
    }
    
    pub fn all_metrics(&self) -> Vec<&MetricDefinition> {
        let mut metrics: Vec<_> = self.metrics.values().collect();
        // Sort by importance/common usage
        metrics.sort_by_key(|m| match m.stat_type {
            StatType::TokenUsage => 0,
            StatType::TokenProgress => 1,
            StatType::TimeElapsed => 2,
            StatType::TimeRemaining => 3,
            StatType::MessageCount => 4,
            StatType::Model => 5,
            StatType::BlockStatus => 6,
            _ => 99,
        });
        metrics
    }
}
