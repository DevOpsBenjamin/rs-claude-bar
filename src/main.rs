mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    // Initialize configuration (creates folder and file if needed)
    let config = rs_claude_bar::initialize_config();

    // Parse CLI first to see if we have a specific command
    let cli = Cli::parse();

    // Claude input parsing is now handled in the status command itself

    // Execute the command
    match cli.command.unwrap_or(Commands::Status) {
        Commands::Status => commands::status::run(&config),
        Commands::Update => commands::update::run(&config),
        Commands::History => commands::history::run(&config),
        Commands::Stats => commands::stats::run(&config),
        Commands::DisplayConfig => commands::display_config::run(&config),
        Commands::Debug => commands::debug::run(&config),
        Commands::Table => commands::table::run(&config),
        Commands::Blocks { debug, gaps } => commands::blocks::run(&config, debug, gaps),
        Commands::Resets => commands::resets::run(&config),
        Commands::Config { command } => commands::config::run(command, &config),
    }
}
