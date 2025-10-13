//! Platform-agnostic file menu interface
//!
//! This module provides a unified interface for file menu components across different platforms,
//! factoring out cfg-dependent code to improve rust-analyzer support.

use crate::application_state::ApplicationState;
use crate::mobile_file_menu::MobileFileMenu;
use crate::web_file_menu::WebFileMenu;
use dioxus::prelude::*;

/// The file menu Dioxus component, if any, or else the empty component.
///
/// The desktop app uses the native menu system.
#[component]
pub fn FileMenu(application_state: Signal<ApplicationState>) -> Element {
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
