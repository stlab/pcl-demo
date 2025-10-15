//! This crate contains all shared UI for the workspace.

mod document_ui;
pub use document_ui::DocumentUI;

mod platform;
pub use platform::{
    delete_document, file_size, load_document, render_file_menu, save_document, saved_files,
    share_document_mobile,
};

// Platform-specific modules - now available on all platforms for better rust-analyzer support
mod mobile_file_menu;
mod web_file_menu;

mod application_state;
pub use application_state::ApplicationState;

mod document;
pub use document::Document;

pub mod wasm_utilities;
pub use wasm_utilities::{JsonEncodedError, NormalizedJsResult};
