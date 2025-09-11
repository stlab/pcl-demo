//! This crate contains all shared UI for the workspace.

mod document_ui;
pub use document_ui::DocumentUI;

#[cfg(target_arch = "wasm32")]
mod file_menu;
#[cfg(target_arch = "wasm32")]
pub use file_menu::FileMenu;

#[cfg(feature = "mobile")]
mod mobile_file_menu;
#[cfg(feature = "mobile")]
pub use mobile_file_menu::MobileFileMenu;

mod application_state;
pub use application_state::*;

mod document;
pub use document::*;

mod shapes;
mod shapes_doc;
mod shapes_ui;
