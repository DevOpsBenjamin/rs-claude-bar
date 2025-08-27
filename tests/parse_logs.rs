use std::fs::File;
use std::io::{BufRead, BufReader};

use serde_json::Value;

#[test]
fn parse_all_sample_logs() {
    let file = File::open("tests/SAMPLE.jsonl").expect("sample log file");
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("read line");
        serde_json::from_str::<Value>(&line)
            .unwrap_or_else(|e| panic!("failed to parse line {}: {}", i + 1, e));
    }
}
