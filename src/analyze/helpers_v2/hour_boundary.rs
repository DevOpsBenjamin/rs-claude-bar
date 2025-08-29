use chrono::{DateTime, Utc, Timelike};

/// Round timestamp down to previous hour boundary (e.g., 15:44 → 15:00)
pub fn round_to_hour_boundary(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_minute(0)
      .unwrap()
      .with_second(0)
      .unwrap()
      .with_nanosecond(0)
      .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_round_to_hour_boundary() {
        let test_time = Utc.with_ymd_and_hms(2025, 8, 29, 15, 44, 30).unwrap();
        let expected = Utc.with_ymd_and_hms(2025, 8, 29, 15, 0, 0).unwrap();
        
        assert_eq!(round_to_hour_boundary(test_time), expected);
    }
    
    #[test]
    fn test_already_on_boundary() {
        let test_time = Utc.with_ymd_and_hms(2025, 8, 29, 15, 0, 0).unwrap();
        
        assert_eq!(round_to_hour_boundary(test_time), test_time);
    }
}