use crate::application_state::ApplicationState;
use dioxus::prelude::*;

const DOCUMENT_CSS: Asset = asset!("/assets/styling/document.css");

/// Helper function to conditionally render the file menu
#[cfg(target_arch = "wasm32")]
fn render_file_menu(application_state: Signal<ApplicationState>) -> Element {
    rsx! {
        crate::file_menu::FileMenu { application_state }
    }
}

#[cfg(feature = "mobile")]
fn render_file_menu(application_state: Signal<ApplicationState>) -> Element {
    rsx! {
        crate::mobile_file_menu::MobileFileMenu { application_state }
    }
}

#[cfg(not(any(target_arch = "wasm32", feature = "mobile")))]
fn render_file_menu(_application_state: Signal<ApplicationState>) -> Element {
    rsx! {}
}

/// The UI element that describes a document.
#[component]
pub fn DocumentUI(application_state: Signal<ApplicationState>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DOCUMENT_CSS }

        // Show appropriate file menu for each platform
        {render_file_menu(application_state)}

        // Show the SVG canvas
        crate::shapes_ui::SvgCanvasDiv { }
    }
}
