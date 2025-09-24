//! Platform abstraction layer for UI components
//!
//! This module provides platform-agnostic interfaces for UI components,
//! allowing rust-analyzer to provide better completion and analysis
//! by factoring out cfg-dependent code into separate modules.

pub mod file_menu;
pub mod file_operations;

pub use file_menu::*;
pub use file_operations::*;
