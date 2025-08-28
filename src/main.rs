mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("DEBUG: argv = {:?}", args);
    
    // Initialize configuration (creates folder and file if needed)
    let config = rs_claude_bar::initialize_config();
    
    // Parse CLI first to see if we have a specific command
    let cli = Cli::parse();
    
    // Only try to parse Claude input if no explicit command is given (default status)
    if matches!(cli.command, None) {
        if let Some(claude_input) = rs_claude_bar::parse_claude_input() {
            println!("PARAM: session_id={}, model={}", claude_input.session_id, claude_input.model.display_name);
        } else {
            print!("NO PARAM");
        }
    }
    
    // Execute the command
    match cli.command.unwrap_or(Commands::Status) {
        Commands::Status => commands::status::run(&config),
        Commands::Update => commands::update::run(&config),
        Commands::History => commands::history::run(&config),
        Commands::Stats => commands::stats::run(&config),
        Commands::DisplayConfig => commands::display_config::run(&config),
        Commands::Debug => commands::debug::run(&config),
        Commands::Table => commands::table::run(&config),
        Commands::Blocks => commands::blocks::run(&config),
        Commands::Help => commands::help::run(&config),
        Commands::Config { command } => commands::config::run(command, &config),
    }
}
