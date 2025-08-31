
use crate::{
    cli::HelpCommands,
    common::colors::*
};

pub fn run(help_cmd: Option<HelpCommands>) {
    match help_cmd {
        Some(HelpCommands::Config) => show_config_help(),
        Some(HelpCommands::Prompt) => show_prompt_help(),
        Some(HelpCommands::Install) => show_install_help(),
        Some(HelpCommands::Blocks) => show_blocks_help(),
        None => show_general_help(),
    }
}

fn show_general_help() {
    let help_text = format!(
        r#"{bold}{cyan}🤖 Claude Bar - Enhanced Claude Code Usage Tracker{reset}

{bold}DESCRIPTION:{reset}
    A fast, lightweight Rust tool for tracking and analyzing Claude Code usage.
    Parses JSONL transcript files to provide insights into token usage, sessions,
    and 5-hour window limits.

{bold}USAGE:{reset}
    rs-claude-bar [OPTIONS] [COMMAND]

{bold}AVAILABLE COMMANDS:{reset}
    {green}info{reset}             Show basic usage information
    {green}prompt{reset}           Show current Claude status line for shell integration
    {green}install{reset}          Setup Claude Code integration and configuration
    {green}config{reset}           Manage configuration settings
    {green}blocks{reset}           Display 5-hour usage blocks and limits
    {green}help{reset}             Show detailed help for specific commands

{bold}GET HELP FOR SPECIFIC COMMANDS:{reset}
    rs-claude-bar help config       Configure Claude data path and display settings
    rs-claude-bar help prompt       Status line integration guide
    rs-claude-bar help install      Installation and setup guide
    rs-claude-bar help blocks       Usage blocks and limits guide

{bold}GLOBAL OPTIONS:{reset}
    {yellow}--no-cache{reset}        Force bypass cache and reprocess all files
    {yellow}--no-save{reset}         Don't save cache after processing
    {yellow}-h, --help{reset}        Print help information
    {yellow}-V, --version{reset}     Print version information

{bold}QUICK START:{reset}
    1. rs-claude-bar install        # Configure Claude integration
    2. rs-claude-bar prompt         # Test status line output
    3. rs-claude-bar blocks         # View usage blocks

{bold}DATA SOURCES:{reset}
    - Claude Code JSONL transcript files
    - Default location: ~/.claude/projects/
    - Custom location: Use 'rs-claude-bar config claude-path' to change
"#,
        bold = BOLD,
        reset = RESET,
        cyan = CYAN,
        green = GREEN,
        yellow = YELLOW,
    );

    print!("{}", help_text);
}

fn show_config_help() {
    let help_text = format!(
        r#"{bold}{cyan}🔧 Configuration Help{reset}

{bold}USAGE:{reset}
    rs-claude-bar config [SUBCOMMAND]

{bold}SUBCOMMANDS:{reset}
    {green}claude-path{reset}       Configure Claude data directory path
    {green}display{reset}           Configure display settings and formatting

{bold}EXAMPLES:{reset}
    {gray}# Set custom Claude data path{reset}
    rs-claude-bar config claude-path

    {gray}# Configure display options{reset}
    rs-claude-bar config display

{bold}CONFIG FILES:{reset}
    - Configuration: ~/.claude-bar/config.json
    - Cache data: ~/.claude-bar/cache.json
    - Last execution: ~/.claude-bar/last_exec

{bold}DEFAULT PATHS:{reset}
    - Claude data: ~/.claude/projects/
    - Config dir: ~/.claude-bar/
"#,
        bold = BOLD,
        reset = RESET,
        cyan = CYAN,
        green = GREEN,
        gray = GRAY,
    );

    print!("{}", help_text);
}

fn show_prompt_help() {
    let help_text = format!(
        r#"{bold}{cyan}📊 Prompt/Status Line Help{reset}

{bold}USAGE:{reset}
    rs-claude-bar prompt

{bold}OUTPUT FORMAT:{reset}
    [████░░░░░░] 45.1% • 23.5K/52.0K • 💬 338 • 3h 23m remaining • 🤖 Claude

{bold}COMPONENTS:{reset}
    - Progress bar showing usage percentage within 5-hour window
    - Token usage: current/limit (e.g., 23.5K/52.0K)
    - Message count for current session
    - Time remaining until window reset
    - Current Claude model indicator

{bold}SHELL INTEGRATION:{reset}
    {gray}# Add to ~/.bashrc or ~/.zshrc{reset}
    export PS1="$(rs-claude-bar prompt) $PS1"

{bold}CLAUDE CODE INTEGRATION:{reset}
    Add to ~/.claude/settings.json:
    {{
      "statusLine": {{
        "type": "command",
        "command": "/path/to/rs-claude-bar prompt",
        "padding": 0
      }}
    }}

{bold}PERFORMANCE:{reset}
    - Sub-100ms response time through intelligent caching
    - Only processes new data since last run
    - Minimal filesystem access
"#,
        bold = BOLD,
        reset = RESET,
        cyan = CYAN,
        gray = GRAY,
    );

    print!("{}", help_text);
}

fn show_install_help() {
    let help_text = format!(
        r#"{bold}{cyan}⚙️ Installation Help{reset}

{bold}USAGE:{reset}
    rs-claude-bar install

{bold}WHAT IT DOES:{reset}
    - Detects Claude Code installation and data directory
    - Creates configuration files in ~/.claude-bar/
    - Sets up optimal display settings
    - Configures Claude Code status line integration

{bold}MANUAL INSTALLATION:{reset}
    1. Build release version: cargo build --release
    2. Copy binary to PATH: cp target/release/rs-claude-bar /usr/local/bin/
    3. Run install: rs-claude-bar install
    4. Test integration: rs-claude-bar prompt

{bold}CLAUDE CODE INTEGRATION:{reset}
    The install command will guide you through adding this to ~/.claude/settings.json:
    {{
      "statusLine": {{
        "type": "command", 
        "command": "rs-claude-bar prompt",
        "padding": 0
      }}
    }}

{bold}TROUBLESHOOTING:{reset}
    - Ensure Claude Code is installed and has been run at least once
    - Check that ~/.claude/projects/ exists and contains data
    - Verify permissions on ~/.claude-bar/ directory
"#,
        bold = BOLD,
        reset = RESET,
        cyan = CYAN,
    );

    print!("{}", help_text);
}

fn show_blocks_help() {
    let help_text = format!(
        r#"{bold}{cyan}📊 Usage Blocks Help{reset}

{bold}USAGE:{reset}
    rs-claude-bar blocks [SUBCOMMAND]

{bold}SUBCOMMANDS:{reset}
    {green}all{reset}               Show all usage blocks from cache
    {green}limits{reset}            Show all limit events and reset times
    {green}gaps{reset}              Show usage gaps between blocks

{bold}5-HOUR WINDOWS:{reset}
    Claude Code enforces 5-hour usage windows for rate limiting.
    This command shows your usage patterns within these windows.

{bold}EXAMPLES:{reset}
    {gray}# Show recent usage blocks{reset}
    rs-claude-bar blocks

    {gray}# Show all cached usage data{reset}
    rs-claude-bar blocks all

    {gray}# Show when limits were hit{reset}
    rs-claude-bar blocks limits

    {gray}# Show usage gaps{reset}
    rs-claude-bar blocks gaps

{bold}OUTPUT INFORMATION:{reset}
    - Block start/end times
    - Token usage (input/output/cache)
    - Message counts
    - Limit events and reset times
"#,
        bold = BOLD,
        reset = RESET,
        cyan = CYAN,
        green = GREEN,
        gray = GRAY,
    );

    print!("{}", help_text);
}

