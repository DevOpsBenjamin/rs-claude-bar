use crate::{
    analyze::Analyzer, 
    cli::BlocksCommands, 
    common::colors::*, 
    table::{
        HeaderInfo, 
        TableCreator,
        format_date,
        format_kind,
        format_duration,
        format_token_count,
        format_number_with_separators,
    }
};

/// Minimal `blocks` implementation: use Analyzer’s limit blocks and display last 10.
pub fn run(block_cmd: Option<BlocksCommands>, analyzer: &Analyzer) {
    println!(
        "{bold}{cyan}📊 5-Hour Usage Blocks (limits){reset}",
        bold = { BOLD }, cyan = { CYAN }, reset = { RESET }
    );

    // Get blocks based on command
    let all_blocks = analyzer.blocks_typed_all();
    let blocks = match block_cmd {
        Some(BlocksCommands::All) => {
            // Show all blocks, no filter
            all_blocks
        },
        Some(BlocksCommands::Limits) => {
            // Filter only limit blocks + current
            all_blocks.into_iter()
                .filter(|block| matches!(block.kind, crate::analyze::BlockKind::Limit | crate::analyze::BlockKind::Current))
                .collect()
        },
        Some(BlocksCommands::Gap) => {
            // Filter only gap blocks + current
            all_blocks.into_iter()
                .filter(|block| matches!(block.kind, crate::analyze::BlockKind::Gap | crate::analyze::BlockKind::Current))
                .collect()
        },
        None => {
            // Default: show last 10 limit blocks + current
            let filtered: Vec<_> = all_blocks.into_iter()
                .filter(|block| matches!(block.kind, crate::analyze::BlockKind::Limit | crate::analyze::BlockKind::Current))
                .collect();
            
            // Keep current block and take 10 most recent limits
            let current_blocks: Vec<_> = filtered.iter().cloned()
                .filter(|block| matches!(block.kind, crate::analyze::BlockKind::Current))
                .collect();
            let mut limit_blocks: Vec<_> = filtered.into_iter()
                .filter(|block| matches!(block.kind, crate::analyze::BlockKind::Limit))
                .collect();
            // Sort by end time (most recent first) and take last 10
            limit_blocks.sort_by_key(|b| b.end);
            limit_blocks = limit_blocks.into_iter().rev().take(10).rev().collect();
            
            // Put limit blocks first (oldest to newest), then current block (most recent at bottom)
            [limit_blocks, current_blocks].concat()
        }
    };

    // Table: Start | End | Duration | Tokens | Messages | Status (most recent first)
    let headers = vec![
        HeaderInfo::new("Start", 11),
        HeaderInfo::new("End", 11),
        HeaderInfo::new("Length", 7),
        HeaderInfo::new("Tokens", 6),
        HeaderInfo::new("Messages", 9),
        HeaderInfo::new("Status", 10),
    ];
    let mut tc = TableCreator::new(headers);
    for b in blocks {
        let start = b.start;
        let end = b.end;
        let duration = end.signed_duration_since(start);
        let tokens = b.stats.output_tokens;
        let messages = b.stats.assistant_messages + b.stats.user_messages;

        tc.add_row(vec![
            format_date(start, 1),
            format_date(end, 1),
            format_duration(duration, 7),
            format_token_count(tokens as u32, 6),
            format_number_with_separators(messages as u32),
            format_kind(&b.kind),
        ]);
    }
    tc.display(false);
}
