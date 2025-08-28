use std::io::{self, Write};
use rs_claude_bar::colors::*;
use rs_claude_bar::config_manager::save_config;
use crate::cli::ConfigCommands;

pub fn run(config_cmd: Option<ConfigCommands>, config: &rs_claude_bar::ConfigInfo) {
    match config_cmd {
        Some(ConfigCommands::Help) => show_config_help(),
        Some(ConfigCommands::ClaudePath) => configure_claude_path(config),
        None => show_config_help(), // Show help when no subcommand provided
    }
}

fn show_config_help() {
    let help_text = format!(r#"
{bold}{cyan}🔧 Configuration Commands{reset}

{bold}USAGE:{reset}
    rs-claude-bar config <SUBCOMMAND>

{bold}SUBCOMMANDS:{reset}
    {green}help{reset}           Show this configuration help
    {green}claude-path{reset}    Configure Claude data directory path

{bold}EXAMPLES:{reset}
    {gray}# Show config help{reset}
    rs-claude-bar config help

    {gray}# Configure Claude data path{reset}
    rs-claude-bar config claude-path

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_config_help_does_not_panic() {
        show_config_help();
    }
}