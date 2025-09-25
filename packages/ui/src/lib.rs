//! This crate contains all shared UI for the workspace.

mod document_ui;
pub use document_ui::DocumentUI;

mod platform;
pub use platform::{
    render_file_menu,
    ,
    save_document,
    load_document,
    delete_document,
    get_saved_files,
    get_file_size_impl,
    share_document_mobile
};

// Platform-specific modules - now available on all platforms for better rust-analyzer support
mod file_menu;
mod mobile_file_menu;

mod application_state;
pub use application_state::*;

mod document;
pub use document::*;
