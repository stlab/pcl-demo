use dioxus::prelude::*;

use ui::{ApplicationState, DocumentUI};

/// Runs the application.
fn main() {
    dioxus::launch(AppUI);
}

/// The application's top-level UI element.
#[component]
fn AppUI() -> Element {
    // The state of the whole application
    let state = use_signal(ApplicationState::new);

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }

        DocumentUI { application_state: state }

    }
}
