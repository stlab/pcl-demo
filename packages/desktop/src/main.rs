use dioxus::{desktop::WindowBuilder, prelude::*};

use ui::{ApplicationState, DocumentUI};

/// The top-level stylesheet for the application.
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Runs the application.
fn main() {

    // Nonstandard startup so the application window doesn't float on
    // top of those of other applications.
    dioxus::LaunchBuilder::desktop().with_cfg(
        dioxus::desktop::Config::default()
            .with_window(
                WindowBuilder::new().with_always_on_top(false)))
    .launch(AppUI);
}

/// The top-level UI element.
#[component]
fn AppUI() -> Element {

    // The state of the whole application
    let state = use_signal(|| ApplicationState::initial());

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        DocumentUI { application_state: state }

    }
}
