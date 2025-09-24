//! Platform-agnostic file menu interface
//!
//! This module provides a unified interface for file menu components across different platforms,
//! factoring out cfg-dependent code to improve rust-analyzer support.

use dioxus::prelude::*;
use crate::application_state::ApplicationState;

/// Platform-specific file menu variants
pub enum PlatformFileMenu {
    Web,
    Mobile,
    Desktop,
}

impl PlatformFileMenu {
    /// Get the appropriate file menu variant for the current platform
    pub fn current() -> Self {
        if cfg!(target_arch = "wasm32") {
            PlatformFileMenu::Web
        } else if cfg!(feature = "mobile") {
            PlatformFileMenu::Mobile
        } else {
            PlatformFileMenu::Desktop
        }
    }
    
    /// Render the file menu for this platform
    pub fn render(&self, application_state: Signal<ApplicationState>) -> Element {
        match self {
            PlatformFileMenu::Web => {
                #[cfg(target_arch = "wasm32")]
                return rsx! {
                    crate::file_menu::FileMenu { application_state }
                };
                
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = application_state;
                    return rsx! {};
                }
            }
            PlatformFileMenu::Mobile => {
                #[cfg(feature = "mobile")]
                return rsx! {
                    crate::mobile_file_menu::MobileFileMenu { application_state }
                };
                
                #[cfg(not(feature = "mobile"))]
                {
                    let _ = application_state;
                    return rsx! {};
                }
            }
            PlatformFileMenu::Desktop => {
                let _ = application_state; // Suppress unused variable warning
                rsx! {}
            }
        }
    }
}

/// Render the appropriate file menu for the current platform
pub fn render_file_menu(application_state: Signal<ApplicationState>) -> Element {
    PlatformFileMenu::current().render(application_state)
}
