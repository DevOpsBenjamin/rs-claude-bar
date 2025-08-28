use std::io::{self, Write};
use rs_claude_bar::{colors::*, StatType, DisplayFormat, DisplayItem};
use rs_claude_bar::config_manager::save_config;
use crate::cli::ConfigCommands;

pub fn run(config_cmd: Option<ConfigCommands>, config: &rs_claude_bar::ConfigInfo) {
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
        bold = if should_use_colors() { BOLD } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
        cyan = if should_use_colors() { CYAN } else { "" },
        green = if should_use_colors() { GREEN } else { "" },
        gray = if should_use_colors() { GRAY } else { "" },
    );

    print!("{}", help_text);
}

fn configure_claude_path(config: &rs_claude_bar::ConfigInfo) {
    let mut config = config.clone();
    
    println!("{bold}{cyan}🔧 Configure Claude Data Path{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        cyan = if should_use_colors() { CYAN } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
    );
    
    println!();
    println!("This is the directory where Claude Code stores conversation history files (JSONL format).");
    println!("Default location is ~/.claude (user home directory).");
    println!();
    
    println!("Current path: {yellow}{}{reset}", 
        config.claude_data_path,
        yellow = if should_use_colors() { YELLOW } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
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
                            green = if should_use_colors() { GREEN } else { "" },
                            yellow = if should_use_colors() { YELLOW } else { "" },
                            reset = if should_use_colors() { RESET } else { "" },
                        );
                    }
                    Err(e) => {
                        println!("{red}✗{reset} Failed to save config: {}",
                            e,
                            red = if should_use_colors() { RED } else { "" },
                            reset = if should_use_colors() { RESET } else { "" },
                        );
                    }
                }
            } else {
                println!("{red}✗{reset} Path does not exist: {}",
                    trimmed_input,
                    red = if should_use_colors() { RED } else { "" },
                    reset = if should_use_colors() { RESET } else { "" },
                );
            }
        }
        Err(e) => {
            println!("{red}✗{reset} Error reading input: {}",
                e,
                red = if should_use_colors() { RED } else { "" },
                reset = if should_use_colors() { RESET } else { "" },
            );
        }
    }
}

fn configure_display(config: &rs_claude_bar::ConfigInfo) {
    let mut config = config.clone();
    
    println!("{bold}{cyan}🎨 Configure Display Settings{reset}",
        bold = if should_use_colors() { BOLD } else { "" },
        cyan = if should_use_colors() { CYAN } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
    );
    println!();
    
    loop {
        // Show current display items
        println!("{bold}Current Display Items:{reset}",
            bold = if should_use_colors() { BOLD } else { "" },
            reset = if should_use_colors() { RESET } else { "" },
        );
        
        for (i, item) in config.display.items.iter().enumerate() {
            let enabled_indicator = if item.enabled { 
                format!("{green}✓{reset}", green = GREEN, reset = RESET) 
            } else { 
                format!("{red}✗{reset}", red = RED, reset = RESET) 
            };
            
            println!("  {}. {} {:?} ({:?})", 
                i + 1, 
                if should_use_colors() { enabled_indicator } else { if item.enabled { "✓" } else { "✗" }.to_string() },
                item.stat_type, 
                item.format
            );
        }
        
        println!();
        println!("Actions:");
        println!("  {green}1-{}{reset} Toggle enable/disable for item", 
            config.display.items.len(),
            green = if should_use_colors() { GREEN } else { "" },
            reset = if should_use_colors() { RESET } else { "" },
        );
        println!("  {green}a{reset} Add new display item",
            green = if should_use_colors() { GREEN } else { "" },
            reset = if should_use_colors() { RESET } else { "" },
        );
        println!("  {green}s{reset} Change separator (current: \"{}\")",
            config.display.separator,
            green = if should_use_colors() { GREEN } else { "" },
            reset = if should_use_colors() { RESET } else { "" },
        );
        println!("  {green}q{reset} Save and quit",
            green = if should_use_colors() { GREEN } else { "" },
            reset = if should_use_colors() { RESET } else { "" },
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
                        green = if should_use_colors() { GREEN } else { "" },
                        reset = if should_use_colors() { RESET } else { "" },
                    );
                }
                Err(e) => {
                    println!("{red}✗{reset} Failed to save config: {}",
                        e,
                        red = if should_use_colors() { RED } else { "" },
                        reset = if should_use_colors() { RESET } else { "" },
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

fn add_display_item(config: &mut rs_claude_bar::ConfigInfo) {
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

fn configure_separator(config: &mut rs_claude_bar::ConfigInfo) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_config_help_does_not_panic() {
        show_config_help();
    }
}