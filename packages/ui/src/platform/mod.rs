//! Platform abstraction layer for UI components
//!
//! This module provides platform-agnostic interfaces for UI components,
//! allowing rust-analyzer to provide better completion and analysis
//! by factoring out cfg-dependent code into separate modules.

pub mod file_menu;
pub mod file_operations;

pub use file_menu::render_file_menu;
pub use file_operations::{save_document, load_document, delete_document, get_saved_files, get_file_size_impl, share_document_mobile};
