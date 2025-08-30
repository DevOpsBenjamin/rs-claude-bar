use clap::Parser;
use std::time::Instant;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

use rs_claude_bar::config_manager::initialize_config;
use rs_claude_bar::cache::CacheManager;
use rs_claude_bar::analyze::Analyzer;
use rs_claude_bar::cli::{Cli, Commands};
use rs_claude_bar::commands;

fn main() {
    let start = std::time::Instant::now();
    // Initialize configuration (creates folder and file if needed)
    let config = initialize_config();    
    let config_duration = start.elapsed();

    // Parse CLI first to get global flags
    let cli = Cli::parse();
    
    let cache = std::time::Instant::now();
    // Load cache (will automatically scan projects subdirectory)
    let mut cache_manager = CacheManager::new(&config.claude_data_path, cli.no_cache);
    let cache_duration = cache.elapsed();

    let file = std::time::Instant::now();
    cache_manager.refresh_cache();
    let file_duration = file.elapsed();

    let analyze =  std::time::Instant::now();
    let mut analyzer = Analyzer::new(cache_manager.get_cache());
    let analyze_duration = file.elapsed();

    let exec = std::time::Instant::now();
    // Execute the command  
    match cli.command.unwrap_or(Commands::Info) {
        Commands::Info => commands::info::run(&config),
        Commands::Install => commands::install::run(&config),
        Commands::Help => commands::help::run(&config),
        Commands::Prompt => commands::prompt::run(&config),
        Commands::Display => commands::display::run(&config),
        Commands::Config { command } => commands::config::run(command, &config),
        Commands::Blocks => commands::blocks::run(&config, &analyzer),

        //Helper for debuging some part of code no use for real app
        Commands::Debug { limits } => commands::debug::run(&config, &mut cache_manager, limits),
    }    
    let exec_duration = exec.elapsed();

    // Save cache to disk with timing (unless --no-save)
    let save = std::time::Instant::now();
    if !cli.no_save {
        cache_manager.save();
    }
    let save_duration = save.elapsed();
    
    let total_duration = start.elapsed();

    //You can alwasy check last cmd duration
    let path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".claude-bar/last_exec");

    let content = format!(
        "Timestamp: {}\nConfig: {:.1} ms\nCache: {:.1} ms\nFile: {:.1} ms\nAnalyze: {:.1} ms\nExec: {:.1} ms\nSave: {:.1} ms\nTotal: {:.1} ms\n",
        Utc::now().to_rfc3339(),
        config_duration.as_secs_f64() * 1000.0,
        cache_duration.as_secs_f64() * 1000.0,
        file_duration.as_secs_f64() * 1000.0,
        analyze_duration.as_secs_f64() * 1000.0,
        exec_duration.as_secs_f64() * 1000.0,
        save_duration.as_secs_f64() * 1000.0,
        total_duration.as_secs_f64() * 1000.0,
    );
    let _ = fs::write(path, content);
}
