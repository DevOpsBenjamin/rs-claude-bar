use rs_claude_bar::colors::*;

pub fn run(_config: &rs_claude_bar::ConfigInfo) {
    let help_text = format!(
        r#"
{bold}{cyan}🤖 Claude Bar - Enhanced Claude Code Usage Tracker{reset}

{bold}DESCRIPTION:{reset}
    A fast, lightweight Rust tool for tracking and analyzing Claude Code usage.
    Parses JSONL transcript files to provide insights into token usage, sessions,
    and 5-hour window limits.

{bold}TIP:{reset}
    Run `rs-claude-bar install` to configure Claude settings.
    Use `rs-claude-bar prompt` to print the status line for your shell.

{bold}USAGE:{reset}
    rs-claude-bar [OPTIONS] [COMMAND]

{bold}COMMANDS:{reset}
    {green}help{reset}             Show this help message
    {green}prompt{reset}           Show current Claude status line
    {green}status{reset}           Show current Claude status line
    {green}table{reset}            Display usage data in detailed table format
    {green}debug{reset}            Debug parse JSONL files and show raw data
    {green}install{reset}          Configure Claude settings for prompt
    {green}config{reset}           Manage configuration settings
    {green}stats{reset}            Show statistical summaries
    {green}history{reset}          Show recent usage windows
    {green}update{reset}           Force refresh of cached statistics
    {green}display-config{reset}   Interactively configure display options

{bold}OPTIONS:{reset}
    {yellow}-h, --help{reset}             Print help information
    {yellow}-V, --version{reset}          Print version information

{bold}EXAMPLES:{reset}
    {gray}# Show this help message{reset}
    rs-claude-bar

    {gray}# Show status line output{reset}
    rs-claude-bar prompt

    {gray}# Show detailed usage table{reset}
    rs-claude-bar table

    {gray}# Debug parsing issues{reset}
    rs-claude-bar debug

    {gray}# Configure Claude settings{reset}
    rs-claude-bar install

    {gray}# Configure Claude data path{reset}
    rs-claude-bar config claude-path

    {gray}# Show config help{reset}
    rs-claude-bar config help

{bold}INTEGRATION:{reset}
    To use as Claude Code status line, add to ~/.claude/settings.json:
    {{
      "statusLine": {{
        "type": "command",
        "command": "/path/to/rs-claude-bar",
        "padding": 0
      }}
    }}

{bold}DATA SOURCES:{reset}
    - Claude Code JSONL transcript files
    - Default location: ~/.claude/projects/
    - Custom location: Use 'rs-claude-bar config claude-path' to change

{bold}OUTPUT FORMAT:{reset}
    Status line shows: 🧠 tokens (%) 🟡 | 💬 messages | ⏱️ elapsed | ⏰ remaining | 🤖 model

{bold}MORE INFO:{reset}
    - GitHub: https://github.com/DevOpsBenjamin/rs-claude-bar
    - Issues: Report bugs and feature requests on GitHub
    - License: MIT

"#,
        bold = if should_use_colors() { BOLD } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
        cyan = if should_use_colors() { CYAN } else { "" },
        green = if should_use_colors() { GREEN } else { "" },
        yellow = if should_use_colors() { YELLOW } else { "" },
        gray = if should_use_colors() { GRAY } else { "" },
    );

    print!("{}", help_text);
}

