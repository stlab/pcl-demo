//! Platform-agnostic file menu interface
//!
//! This module provides a unified interface for file menu components across different platforms,
//! factoring out cfg-dependent code to improve rust-analyzer support.

use crate::application_state::ApplicationState;
use crate::mobile_file_menu::MobileFileMenu;
use crate::web_file_menu::WebFileMenu;
use dioxus::prelude::*;

/// Renders the file menu for the current platform.
pub fn render_file_menu(application_state: Signal<ApplicationState>) -> Element {
    if cfg!(target_arch = "wasm32") {
        rsx! {
            WebFileMenu { application_state }
        }
    } else if cfg!(feature = "mobile") {
        rsx! {
            MobileFileMenu { application_state }
        }
    } else {
        rsx! {}
    }
}
