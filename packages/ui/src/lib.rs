//! This crate contains all shared UI for the workspace.

mod document_ui;
pub use document_ui::DocumentUI;

mod platform;
pub use platform::{
    delete_document, get_file_size_impl, get_saved_files, load_document, render_file_menu,
    save_document, share_document_mobile,
};

// Platform-specific modules - now available on all platforms for better rust-analyzer support
mod file_menu;
mod mobile_file_menu;

mod application_state;
pub use application_state::ApplicationState;

mod document;
pub use document::Document;
