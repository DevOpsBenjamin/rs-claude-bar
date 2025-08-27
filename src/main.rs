mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("DEBUG: argv = {:?}", args);
    
    // Parse CLI first to see if we have a specific command
    let cli = Cli::parse();
    
    // Only try to parse Claude input if no explicit command is given (default status)
    if matches!(cli.command, None) {
        if let Some(claude_input) = rs_claude_bar::parse_claude_input() {
            println!("PARAM: session_id={}, model={}", claude_input.session_id, claude_input.model.display_name);
            return;
        } else {
            print!("NO PARAM");
            return;
        }
    }
    
    // Execute the command
    match cli.command.unwrap_or(Commands::Status) {
        Commands::Status => commands::status::run(),
        Commands::Update => commands::update::run(),
        Commands::History => commands::history::run(),
        Commands::Stats => commands::stats::run(),
        Commands::DisplayConfig => commands::display_config::run(),
        Commands::Debug => commands::debug::run(cli.data_path.as_deref()),
        Commands::Table => commands::table::run(cli.data_path.as_deref()),
    }
}
