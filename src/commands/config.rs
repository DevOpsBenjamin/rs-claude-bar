use std::io::{self, Write};

use crate::{
    common::colors::*,
    display::items::{StatType, DisplayFormat, DisplayItem},
    config_manager::save_config,
    cli::ConfigCommands,
    claudebar_types::ConfigInfo
};

pub fn run(config_cmd: Option<ConfigCommands>, config: &ConfigInfo) {
    match config_cmd {
        Some(ConfigCommands::ClaudePath) => configure_claude_path(config),
        Some(ConfigCommands::Display) => configure_display(config),
        None => show_config_help(), // Show help when no subcommand provided
    }
}

fn show_config_help() {
    let help_text = format!(r#"
{bold}{cyan}🔧 Configuration Commands{reset}

{bold}USAGE:{reset}
    rs-claude-bar config <SUBCOMMAND>

{bold}SUBCOMMANDS:{reset}
    {green}claude-path{reset}    Configure Claude data directory path
    {green}display{reset}        Configure display items and formats

{bold}EXAMPLES:{reset}
    {gray}# Configure Claude data path{reset}
    rs-claude-bar config claude-path
    
    {gray}# Configure display settings{reset}
    rs-claude-bar config display

{bold}CONFIG FILE LOCATION:{reset}
    ~/.claude-bar/config.json

"#,
        bold = { BOLD },
        reset = { RESET },
        cyan = { CYAN },
        green = { GREEN },
        gray = { GRAY },
    );

    print!("{}", help_text);
}

fn configure_claude_path(config: &ConfigInfo) {
    let mut config = config.clone();
    
    println!("{bold}{cyan}🔧 Configure Claude Data Path{reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
    );
    
    println!();
    println!("This is the directory where Claude Code stores conversation history files (JSONL format).");
    println!("Default location is ~/.claude (user home directory).");
    println!();
    
    println!("Current path: {yellow}{}{reset}", 
        config.claude_data_path,
        yellow = { YELLOW },
        reset = { RESET },
    );
    
    print!("Enter new path to Claude data directory (or press Enter to keep current): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = input.trim();
            
            // Check if input is empty or only whitespace
            if trimmed_input.is_empty() {
                println!("Path unchanged: {}", config.claude_data_path);
                return;
            }
            
            // Validate path exists
            if std::path::Path::new(trimmed_input).exists() {
                config.claude_data_path = trimmed_input.to_string();
                
                match save_config(&config) {
                    Ok(_) => {
                        println!("{green}✓{reset} Path updated to: {yellow}{}{reset}",
                            config.claude_data_path,
                            green = { GREEN },
                            yellow = { YELLOW },
                            reset = { RESET },
                        );
                    }
                    Err(e) => {
                        println!("{red}✗{reset} Failed to save config: {}",
                            e,
                            red = { RED },
                            reset = { RESET },
                        );
                    }
                }
            } else {
                println!("{red}✗{reset} Path does not exist: {}",
                    trimmed_input,
                    red = { RED },
                    reset = { RESET },
                );
            }
        }
        Err(e) => {
            println!("{red}✗{reset} Error reading input: {}",
                e,
                red = { RED },
                reset = { RESET },
            );
        }
    }
}

fn configure_display(config: &ConfigInfo) {
    let mut config = config.clone();
    
    println!("{bold}{cyan}🎨 Configure Display Settings{reset}",
        bold = { BOLD },
        cyan = { CYAN },
        reset = { RESET },
    );
    println!();
    
    loop {
        // Show current display items
        println!("{bold}Current Display Items:{reset}",
            bold = { BOLD },
            reset = { RESET },
        );
        
        for (i, item) in config.display.items.iter().enumerate() {
            let enabled_indicator = if item.enabled { 
                format!("{green}✓{reset}", green = GREEN, reset = RESET) 
            } else { 
                format!("{red}✗{reset}", red = RED, reset = RESET) 
            };
            
            println!("  {}. {} {:?} ({:?})", 
                i + 1, 
                if item.enabled { "✓" } else { "✗" }.to_string(),
                item.stat_type, 
                item.format
            );
        }
        
        println!();
        println!("Actions:");
        println!("  {green}1-{}{reset} Toggle enable/disable for item", 
            config.display.items.len(),
            green = { GREEN },
            reset = { RESET },
        );
        println!("  {green}a{reset} Add new display item",
            green = { GREEN },
            reset = { RESET },
        );
        println!("  {green}s{reset} Change separator (current: \"{}\")",
            config.display.separator,
            green = { GREEN },
            reset = { RESET },
        );
        println!("  {green}q{reset} Save and quit",
            green = { GREEN },
            reset = { RESET },
        );
        
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        
        let choice = input.trim().to_lowercase();
        
        if choice == "q" {
            // Save config
            match save_config(&config) {
                Ok(_) => {
                    println!("{green}✓{reset} Display configuration saved!",
                        green = { GREEN },
                        reset = { RESET },
                    );
                }
                Err(e) => {
                    println!("{red}✗{reset} Failed to save config: {}",
                        e,
                        red = { RED },
                        reset = { RESET },
                    );
                }
            }
            break;
        } else if choice == "a" {
            add_display_item(&mut config);
        } else if choice == "s" {
            configure_separator(&mut config);
        } else if let Ok(num) = choice.parse::<usize>() {
            if num > 0 && num <= config.display.items.len() {
                config.display.items[num - 1].enabled = !config.display.items[num - 1].enabled;
                println!("Toggled item {} {}", num, 
                    if config.display.items[num - 1].enabled { "ON" } else { "OFF" }
                );
            }
        }
        
        println!(); // Add space between iterations
    }
}

fn add_display_item(config: &mut ConfigInfo) {
    println!("Available stat types:");
    let stat_types = [
        StatType::TokenUsage,
        StatType::TokenPercentage, 
        StatType::TimeElapsed,
        StatType::TimeRemaining,
        StatType::ResetTime,
        StatType::Model,
        StatType::MessageCount,
        StatType::BlockStatus,
    ];
    
    for (i, stat_type) in stat_types.iter().enumerate() {
        println!("  {}. {:?}", i + 1, stat_type);
    }
    
    print!("Select stat type (1-{}): ", stat_types.len());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    
    if let Ok(choice) = input.trim().parse::<usize>() {
        if choice > 0 && choice <= stat_types.len() {
            let stat_type = stat_types[choice - 1].clone();
            
            // Select format
            println!("Available display formats:");
            let formats = [
                DisplayFormat::Text,
                DisplayFormat::TextWithEmoji,
                DisplayFormat::ProgressBar,
                DisplayFormat::Compact,
                DisplayFormat::PercentageOnly,
                DisplayFormat::Duration,
                DisplayFormat::StatusIcon,
            ];
            
            for (i, format) in formats.iter().enumerate() {
                println!("  {}. {:?}", i + 1, format);
            }
            
            print!("Select format (1-{}): ", formats.len());
            io::stdout().flush().unwrap();
            
            let mut format_input = String::new();
            if io::stdin().read_line(&mut format_input).is_err() {
                return;
            }
            
            if let Ok(format_choice) = format_input.trim().parse::<usize>() {
                if format_choice > 0 && format_choice <= formats.len() {
                    let format = formats[format_choice - 1].clone();
                    let new_item = DisplayItem::new(stat_type, format);
                    config.display.items.push(new_item);
                    println!("Added new display item!");
                }
            }
        }
    }
}

fn configure_separator(config: &mut ConfigInfo) {
    print!("Enter new separator (current: \"{}\"): ", config.display.separator);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let new_separator = input.trim();
        if !new_separator.is_empty() {
            config.display.separator = new_separator.to_string();
            println!("Separator updated to: \"{}\"", config.display.separator);
        }
    }
}

