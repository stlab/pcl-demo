use dioxus::prelude::*;
use crate::application_state::*;
use crate::document::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
#[component]
pub fn Hero(document: ReadSignal<ApplicationState>) -> Element {

    let html = document.read().the_only_document.to_html();

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }
        div {
            id: "hero",
            dangerous_inner_html: html
        }
    }
}
