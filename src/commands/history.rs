pub fn run(_config: &rs_claude_bar::ConfigInfo) {
    println!("📜 Showing recent windows... (placeholder)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_does_not_panic() {
        let config = rs_claude_bar::ConfigInfo {
            claude_data_path: "nonexistent".to_string(),
        };
        run(&config);
    }
}
