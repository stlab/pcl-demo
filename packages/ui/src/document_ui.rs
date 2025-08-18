use dioxus::prelude::*;
use crate::application_state::*;

const DOCUMENT_CSS: Asset = asset!("/assets/styling/document.css");

/// The UI element that describes a document.
#[component]
pub fn DocumentUI(application_state: ReadSignal<ApplicationState>) -> Element {

    // Convert the document to something we can display.
    let html = application_state.read().the_only_document.to_html();

    rsx! {
        document::Link { rel: "stylesheet", href: DOCUMENT_CSS }
        div {
            id: "document",
            dangerous_inner_html: html
        }
    }
}
