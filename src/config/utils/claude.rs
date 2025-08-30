use crate::config::ConfigManager;

pub fn run_claude_config(_config_manager: &mut ConfigManager) {
}

/*

use crate::{common::colors::*, config::ConfigManager};

impl ConfigManager {
        /// Interactively configure the Claude data directory path
    pub fn configure_claude_path(&mut self) {
        let mut config = self.config.clone();
        
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
}

     */