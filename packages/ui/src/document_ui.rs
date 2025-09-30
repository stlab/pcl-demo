use dioxus::prelude::*;
use crate::application_state::ApplicationState;
use crate::platform::render_file_menu;

const DOCUMENT_CSS: Asset = asset!("/assets/styling/document.css");

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
