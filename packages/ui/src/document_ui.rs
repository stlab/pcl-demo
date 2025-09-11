use dioxus::prelude::*;
use crate::application_state::*;

#[cfg(target_arch = "wasm32")]
use crate::file_menu::FileMenu;

#[cfg(feature = "mobile")]
use crate::mobile_file_menu::MobileFileMenu;

const DOCUMENT_CSS: Asset = asset!("/assets/styling/document.css");

/// Helper function to conditionally render the file menu
#[cfg(target_arch = "wasm32")]
fn render_file_menu(application_state: Signal<ApplicationState>) -> Element {
    rsx! {
        FileMenu { application_state }
        crate::shapes_ui::SvgCanvasDiv { }
    }
}

#[cfg(feature = "mobile")]
fn render_file_menu(application_state: Signal<ApplicationState>) -> Element {
    rsx! {
        MobileFileMenu { application_state }
    }
}

#[cfg(not(any(target_arch = "wasm32", feature = "mobile")))]
fn render_file_menu(_application_state: Signal<ApplicationState>) -> Element {
    rsx! {}
}

/// The UI element that describes a document.
#[component]
pub fn DocumentUI(application_state: Signal<ApplicationState>) -> Element {

    // Convert the document to something we can display.
    let html = application_state.read().the_only_document.to_html();

    rsx! {
        document::Link { rel: "stylesheet", href: DOCUMENT_CSS }
        
        // Show appropriate file menu for each platform
        {render_file_menu(application_state)}
        
        div {
            id: "document",
            dangerous_inner_html: html
        }
    }
}
