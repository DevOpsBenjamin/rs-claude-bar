use crate::{
    common::colors::*,
};

pub fn run() {
    let info_text = format!(
        r#"
{bold}{cyan}🤖 Claude Bar{reset} - Claude Code Usage Tracker

{bold}BASIC USAGE:{reset}
    rs-claude-bar [info]         Show this info
    rs-claude-bar prompt         Show status line
    rs-claude-bar help           Show comprehensive help
    rs-claude-bar install        Setup Claude integration
    rs-claude-bar config         Manage configuration
    rs-claude-bar blocks         Show usage blocks

{bold}GET DETAILED HELP:{reset}
    rs-claude-bar help config    Configuration guide  
    rs-claude-bar help prompt    Status line integration
    rs-claude-bar help install   Installation guide
    rs-claude-bar help blocks    Usage blocks guide
"#,
        bold = { BOLD },
        reset = { RESET },
        cyan = { CYAN },
    );

    print!("{}", info_text);
}