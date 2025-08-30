use crate::{common::colors::RESET, config::ConfigInfo, display::prompt::{generate_status_line, PromptData}};

pub fn run(config: &ConfigInfo, data: &PromptData) {    
    println!("{reset}{}", generate_status_line(data, &config.display), reset = RESET);
}