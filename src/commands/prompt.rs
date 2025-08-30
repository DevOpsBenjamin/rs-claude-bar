use crate::{config::ConfigInfo, display::prompt::{generate_status_line, PromptData}};

pub fn run(config: &ConfigInfo, data: &PromptData) {    
    println!("{}", generate_status_line(data, &config.display));
}