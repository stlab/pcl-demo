use dioxus::prelude::*;
use crate::application_state::*;

#[cfg(target_arch = "wasm32")]
use crate::file_menu::FileMenu;

const DOCUMENT_CSS: Asset = asset!("/assets/styling/document.css");

/// Helper function to conditionally render the file menu
#[cfg(target_arch = "wasm32")]
fn render_file_menu(application_state: Signal<ApplicationState>) -> Element {
    rsx! {
        FileMenu { application_state }
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
        
        // Only show file menu on web platform
        {render_file_menu(application_state)}
        
        div {
            id: "document",
            dangerous_inner_html: html
        }
    }
}
