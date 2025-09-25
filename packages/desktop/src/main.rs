use dioxus::{desktop::{WindowBuilder, Config, use_muda_event_handler, muda::MenuEvent}, prelude::*, LaunchBuilder};

use ui::{ApplicationState, DocumentUI};

mod platform;
use platform::{PlatformDialogs, PlatformMenu};

/// The top-level stylesheet for the application.
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Runs the application.
fn main() {
    let menu_bar = PlatformMenu::create_menu_bar();

    // Nonstandard startup so the application window doesn't float on
    // top of those of other applications.
    LaunchBuilder::desktop().with_cfg(
        Config::default()
            .with_window(
                WindowBuilder::new()
                    .with_always_on_top(false))
            .with_menu(menu_bar))
    .launch(AppUI);
}


/// The top-level UI element.
#[component]
fn AppUI() -> Element {

    // The state of the whole application
    let mut state = use_signal(|| ApplicationState::new());

    // Handle menu events
    use_muda_event_handler(move |event| {
        let MenuEvent { id } = event;
        match id.0.as_str() {
            "new" => {
                state.write().new_document();
            }
            "open" => {
                if let Some(file_path) = PlatformDialogs::show_open_dialog() {
                    match state.write().load_document(file_path.clone()) {
                        Ok(()) => {},
                        Err(e) => eprintln!("Failed to open file: {}", e),
                    }
                }
            }
            "save" => {
                let can_save = state.read().current_file_path.is_some();
                if can_save {
                    match state.read().save_document() {
                        Ok(()) => {},
                        Err(e) => eprintln!("Failed to save file: {}", e),
                    }
                } else {
                    if let Some(file_path) = PlatformDialogs::show_save_dialog() {
                        match state.write().save_document_as(file_path.clone()) {
                            Ok(()) => {},
                            Err(e) => eprintln!("Failed to save file: {}", e),
                        }
                    }
                }
            }
            "save_as" => {
                if let Some(file_path) = PlatformDialogs::show_save_dialog() {
                    match state.write().save_document_as(file_path.clone()) {
                        Ok(()) => {},
                        Err(e) => eprintln!("Failed to save file: {}", e),
                    }
                }
            }
            _ => { unreachable!("unknown menu item {:?}", id.as_ref()) }
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        DocumentUI { application_state: state }

    }
}
