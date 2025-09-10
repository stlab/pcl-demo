//! This crate contains all shared UI for the workspace.

mod document_ui;
pub use document_ui::DocumentUI;

#[cfg(target_arch = "wasm32")]
mod file_menu;
#[cfg(target_arch = "wasm32")]
pub use file_menu::FileMenu;

mod application_state;
pub use application_state::*;

mod document;
pub use document::*;
