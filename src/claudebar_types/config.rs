use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Stats file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatsFile {
    /// Last processed timestamp
    pub last_processed: Option<DateTime<Utc>>,
}
