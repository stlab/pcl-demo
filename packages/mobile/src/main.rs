use dioxus::prelude::*;

use ui::{ApplicationState, DocumentUI};

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Runs the application.
fn main() {
    dioxus::launch(AppUI);
}

/// The top-level UI element.
#[component]
fn AppUI() -> Element {
    // The state of the whole application
    let state = use_signal(ApplicationState::new);

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        DocumentUI { application_state: state }

    }
}
