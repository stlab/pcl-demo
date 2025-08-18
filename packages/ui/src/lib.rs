//! This crate contains all shared UI for the workspace.

mod document_ui;
pub use document_ui::DocumentUI;

mod application_state;
pub use application_state::*;

mod document;
pub use document::*;
