pub fn run(config: &rs_claude_bar::ConfigInfo) {
    // TODO: Implement generate_claude_status_with_config that uses config
    println!(
        "🔄 Status with config path: {} (placeholder)",
        config.claude_data_path
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_does_not_panic() {
        let config = rs_claude_bar::ConfigInfo::default();
        run(&config);
    }
}
