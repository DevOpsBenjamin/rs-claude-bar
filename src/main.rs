mod analyze;
mod claude_types;
mod claudebar_types;
mod cli;
mod commands;
mod common;
mod config_manager;
mod display;
mod helpers;
mod status;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use crate::config_manager::config_loader::*;

fn main() {
    // Initialize configuration (creates folder and file if needed)
    let config = initialize_config();

    // Parse CLI first to see if we have a specific command
    let cli = Cli::parse();

    // Execute the command  
    match cli.command.unwrap_or(Commands::Info) {
        Commands::Info => commands::info::run(&config),
        Commands::Install => commands::install::run(&config),
        Commands::Help => commands::help::run(&config),
        Commands::Prompt => commands::prompt::run(&config),
        Commands::DisplayConfig => commands::display_config::run(&config),
        Commands::Config { command } => commands::config::run(command, &config),
        Commands::Blocks => commands::blocks::run(&config),

        //Helper for debuging some part of code no use for real app
        Commands::Debug { parse, file, blocks, gaps, limits, files } => commands::debug::run(&config, parse, file, blocks, gaps, limits, files),
    }
}
