use crate::{
    cli::ConfigCommands, common::colors::*, config::{utils::PromptData, ConfigManager}
};

pub fn run(config_cmd: Option<ConfigCommands>, config_manager: &mut ConfigManager, data: &PromptData) {
    match config_cmd {
        Some(ConfigCommands::ClaudePath) => config_manager.configure_claude(),
        Some(ConfigCommands::Display) => config_manager.configure_display(&data),
        None => show_config_help(), // Show help when no subcommand provided
    }
}

pub fn show_config_help() {
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