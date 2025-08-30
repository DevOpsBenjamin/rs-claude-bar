use chrono::{DateTime, Timelike, Utc};

/// Round timestamp down to hour boundary (14:32:15 -> 14:00:00)
pub fn round_to_hour_boundary(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
}
