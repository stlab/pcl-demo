use dioxus::{desktop::WindowBuilder, prelude::*};

use ui::{ApplicationState, Hero};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {

    dioxus::LaunchBuilder::desktop().with_cfg(
        dioxus::desktop::Config::default()
            .with_window(
                WindowBuilder::new().with_always_on_top(false)))
    .launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    let mut state = use_signal(|| ApplicationState::initial());

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Hero { document: state }

    }
}
