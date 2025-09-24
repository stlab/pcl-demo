//! This crate contains all shared UI for the workspace.

mod document_ui;
pub use document_ui::DocumentUI;

mod platform;
pub use platform::{
    render_file_menu,
    FileOperationResult,
    save_document,
    load_document,
    delete_document,
    get_saved_documents,
    get_file_size,
    show_open_dialog,
    show_save_dialog,
    share_document
};

// Platform-specific modules - now available on all platforms for better rust-analyzer support
mod file_menu;
mod mobile_file_menu;

mod application_state;
pub use application_state::*;

mod document;
pub use document::*;
