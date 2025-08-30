use crate::analyze::BlockKind;

pub fn format_kind(kind: &BlockKind) -> String {
    match kind {
        BlockKind::Limit => format!("{status:>8}", status = "Limit"),
        BlockKind::Gap => format!("{status:>8}", status = "Session"),
        BlockKind::Current => format!("{status:>8}", status = "Current"),
    }
}