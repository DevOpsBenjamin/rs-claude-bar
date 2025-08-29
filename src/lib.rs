// Public modules that can be used as crate::module_name::*
pub mod analyzer;
pub mod claude_types;
pub mod claudebar_types;
pub mod config_manager;
pub mod display;
pub mod helpers;
pub mod status;
pub mod utils;

// Re-export utility modules for internal use
pub mod colors {
    pub use crate::common::colors::*;
}
pub mod app_dirs {
    pub use crate::common::app_dirs::*;
}

// Private modules for internal use
mod common;

// Re-export commonly used items
pub use common::*;

// Re-export the main configuration function
pub use config_manager::initialize_config;