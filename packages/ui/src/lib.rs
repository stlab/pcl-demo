//! This crate contains all shared UI for the workspace.

mod document_ui;
pub use document_ui::DocumentUI;

mod platform;
pub use platform::{
    render_file_menu, 
    get_file_operations, 
    FileOperations, 
    FileOperationResult,
    PlatformFileOperations,
    PlatformFileMenu,
    WebFileOperations,
    MobileFileOperations,
    DesktopFileOperations
};

// Platform-specific modules are still needed for their implementations
// but are no longer exported directly - access through platform abstraction
#[cfg(target_arch = "wasm32")]
mod file_menu;

#[cfg(feature = "mobile")]
mod mobile_file_menu;

mod application_state;
pub use application_state::*;

mod document;
pub use document::*;
