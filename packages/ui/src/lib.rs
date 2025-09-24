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
    WebFileOperations,
    MobileFileOperations,
    DesktopFileOperations
};

// Platform-specific modules - now available on all platforms for better rust-analyzer support
mod file_menu;
mod mobile_file_menu;

mod application_state;
pub use application_state::*;

mod document;
pub use document::*;
