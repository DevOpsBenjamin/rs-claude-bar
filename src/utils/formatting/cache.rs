use crate::claudebar_types::cache::CacheStatus;

pub fn format_cache_status(status: CacheStatus) -> String {
    match status {
        CacheStatus::Fresh => format!("{ico:<2}{status:>8}", ico = "✅", status = "Fresh"),
        CacheStatus::NeedsRefresh => format!("{ico:<2}{status:>8}", ico = "🔄", status = "Refresh"),
        CacheStatus::NotInCache => format!("{ico:<2}{status:>8}", ico = "❌", status = "Missing"),
    }
}