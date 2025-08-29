use clap::Parser;

use rs_claude_bar::config_manager::initialize_config;
use rs_claude_bar::cache::CacheManager;
use rs_claude_bar::cli::{Cli, Commands};
use rs_claude_bar::commands;

fn main() {
    // Initialize configuration (creates folder and file if needed)
    let config = initialize_config();
    // Load cache (will automatically scan projects subdirectory)
    let mut cache_manager = CacheManager::new(&config.claude_data_path);

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
        Commands::Debug { parse, cache, file, blocks, gaps, limits, files, no_cache } => commands::debug::run(&config, &mut cache_manager, parse, cache, file, blocks, gaps, limits, files, no_cache),
    }
}
