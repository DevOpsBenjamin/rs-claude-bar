mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Status) {
        Commands::Status => commands::status::run(),
        Commands::Update => commands::update::run(),
        Commands::History => commands::history::run(),
        Commands::Stats => commands::stats::run(),
        Commands::DisplayConfig => commands::display_config::run(),
    }
}
