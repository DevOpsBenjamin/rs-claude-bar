use rs_claude_bar::colors::*;

pub fn run(_config: &rs_claude_bar::ConfigInfo) {
    let info_text = format!(
        r#"
{bold}{cyan}🤖 Claude Bar{reset} - Claude Code Usage Tracker

{bold}BASIC USAGE:{reset}
    rs-claude-bar [info]         Show this info
    rs-claude-bar prompt         Show status line
    rs-claude-bar help           Show all cmd help
    rs-claude-bar install        Setup Claude integration
"#,
        bold = if should_use_colors() { BOLD } else { "" },
        reset = if should_use_colors() { RESET } else { "" },
        cyan = if should_use_colors() { CYAN } else { "" },
    );

    print!("{}", info_text);
}