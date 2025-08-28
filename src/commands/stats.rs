use tabled::{Table, Tabled};

#[derive(Tabled)]
struct StatRow {
    metric: &'static str,
    value: &'static str,
}

pub fn run(_config: &rs_claude_bar::ConfigInfo) {
    let data = [
        StatRow {
            metric: "Total Tokens",
            value: "0",
        },
        StatRow {
            metric: "Total Cost",
            value: "$0.00",
        },
    ];
    let table = Table::new(data).to_string();
    println!("{}", table);
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
