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
    rs-claude-bar help           Show all cmd help
    rs-claude-bar install        Setup Claude integration
"#,
        bold = { BOLD },
        reset = { RESET },
        cyan = { CYAN },
    );

    print!("{}", info_text);
}