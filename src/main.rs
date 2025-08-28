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
    match cli.command.unwrap_or(Commands::Help) {
        Commands::Help => commands::help::run(&config),
        Commands::Prompt => commands::prompt::run(&config),
        Commands::Update => commands::update::run(&config),
        Commands::History => commands::history::run(&config),
        Commands::Stats => commands::stats::run(&config),
        Commands::DisplayConfig => commands::display_config::run(&config),
        Commands::Debug { parse } => commands::debug::run(&config, parse),
        Commands::Table => commands::table::run(&config),
        Commands::Blocks { debug, gaps, limits } => commands::blocks::run(&config, debug, gaps, limits),
        Commands::Resets => commands::resets::run(&config),
        Commands::Install => commands::install::run(&config),
        Commands::Config { command } => commands::config::run(command, &config),
    }
}
