use dioxus::prelude::*;
use crate::application_state::*;
use crate::document::*;

const DOCUMENT_CSS: Asset = asset!("/assets/styling/document.css");

#[component]
pub fn DocumentUI(application_state: ReadSignal<ApplicationState>) -> Element {

    let html = application_state.read().the_only_document.to_html();

    rsx! {
        document::Link { rel: "stylesheet", href: DOCUMENT_CSS }
        div {
            id: "document",
            dangerous_inner_html: html
        }
    }
}
