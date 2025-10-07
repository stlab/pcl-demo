use dioxus::prelude::*;

use ui::{ApplicationState, DocumentUI};

/// The application's favicon.
const FAVICON: Asset = asset!("/assets/favicon.ico");
/// The application's top-level style.
const MAIN_CSS: Asset = asset!("/assets/main.css");

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
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        DocumentUI { application_state: state }

    }
}
