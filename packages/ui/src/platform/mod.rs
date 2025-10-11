//! Platform abstraction layer for UI components
//!
//! This module provides platform-agnostic interfaces for UI components,
//! allowing rust-analyzer to provide better completion and analysis
//! by factoring out cfg-dependent code into separate modules.

pub mod file_menu;
pub mod file_operations;

pub use file_menu::render_file_menu;
pub use file_operations::{
    delete_document, file_size, load_document, save_document, saved_files, share_document_mobile,
};
