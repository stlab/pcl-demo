//! Platform-agnostic file menu interface
//!
//! This module provides a unified interface for file menu components across different platforms,
//! factoring out cfg-dependent code to improve rust-analyzer support.

use dioxus::prelude::*;
use crate::application_state::ApplicationState;

/// Render the appropriate file menu for the current platform
pub fn render_file_menu(application_state: Signal<ApplicationState>) -> Element {
    #[cfg(target_arch = "wasm32")]
    return rsx! {
        crate::file_menu::FileMenu { application_state }
    };
    
    #[cfg(feature = "mobile")]
    return rsx! {
        crate::mobile_file_menu::MobileFileMenu { application_state }
    };
    
    #[cfg(not(any(target_arch = "wasm32", feature = "mobile")))]
    {
        let _ = application_state; // Suppress unused variable warning
        return rsx! {};
    }
}
