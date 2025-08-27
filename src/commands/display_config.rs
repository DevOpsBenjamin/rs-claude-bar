use rs_claude_bar::reset_config_interactive;

pub fn run() {
    // allow user to reset display configuration interactively
    let cfg = reset_config_interactive();
    println!("Saved {} display items", cfg.display.len());
}
