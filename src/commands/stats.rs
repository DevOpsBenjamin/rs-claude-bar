use tabled::{Table, Tabled};

#[derive(Tabled)]
struct StatRow {
    metric: &'static str,
    value: &'static str,
}

pub fn run() {
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
