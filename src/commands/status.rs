use rs_claude_bar::{debug_output, generate_claude_status};

pub fn run() {
    match generate_claude_status() {
        Ok(status) => println!("{}", status),
        Err(_) => println!("{}", debug_output()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_does_not_panic() {
        run();
    }
}
