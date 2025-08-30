use clap::Parser;
use std::time::Instant;
use std::fs;
use std::path::PathBuf;
use chrono::Utc;

use rs_claude_bar::config::ConfigManager;
use rs_claude_bar::cache::CacheManager;
use rs_claude_bar::analyze::Analyzer;
use rs_claude_bar::cli::{Cli, Commands};
use rs_claude_bar::display::prompt::PromptData;
use rs_claude_bar::commands::{self};

fn main() {
    let start = Instant::now();
    
    // Initialize configuration (creates folder and file if needed)
    let mut config_manager = ConfigManager::new();
    let config = config_manager.get_config();
    let config_duration = start.elapsed();

    // Parse CLI first to get global flags
    let cli = Cli::parse();
    
    let cache = Instant::now();
    // Load cache (will automatically scan projects subdirectory)
    let mut cache_manager = CacheManager::new(&config.claude_data_path, cli.no_cache);
    let cache_duration = cache.elapsed();

    let file = Instant::now();
    cache_manager.refresh_cache();
    let file_duration = file.elapsed();

    let analyze =  Instant::now();
    let analyzer = Analyzer::new(cache_manager.get_cache());
    let prompt_data = PromptData::new(&analyzer);
    let analyze_duration = analyze.elapsed();

    let exec = Instant::now();
    // Execute the command  
    match cli.command.unwrap_or(Commands::Info) {
        Commands::Info => commands::info::run(),
        Commands::Install => commands::install::run(),        
        Commands::Help { command } => commands::help::run(command),
        Commands::Prompt => commands::prompt::run(&config, &prompt_data),
        Commands::Config { command } => commands::config::run(command, &mut config_manager, &prompt_data),
        Commands::Blocks => commands::blocks::run(&analyzer),
    }    
    let exec_duration = exec.elapsed();

    // Save cache to disk with timing (unless --no-save)
    let save = Instant::now();
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
