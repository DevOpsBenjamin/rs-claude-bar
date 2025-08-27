use rs_claude_bar::{generate_claude_status, debug_output};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check for debug flag (Rust standard convention)
    if args.len() > 1 && args[1] == "--debug" {
        print!("{}", debug_output());
        return;
    }
    
    match generate_claude_status() {
        Ok(status) => print!("{}", status),
        Err(e) => print!("🤖 Claude Code | ❌ Error: {}", e),
    }
}