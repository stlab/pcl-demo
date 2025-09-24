use dioxus::{desktop::WindowBuilder, prelude::*};

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
    dioxus::LaunchBuilder::desktop().with_cfg(
        dioxus::desktop::Config::default()
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
    dioxus::desktop::use_muda_event_handler(move |event| {
        let dioxus::desktop::muda::MenuEvent { id } = event;
        println!("Menu event received with ID: '{}'", id.0);
        match id.0.as_str() {
            "new" => {
                println!("Creating new document");
                state.write().new_document();
            }
            "open" => {
                println!("Opening file dialog");
                if let Some(file_path) = PlatformDialogs::show_open_dialog() {
                    match state.write().load_document(file_path.clone()) {
                        Ok(()) => println!("Successfully opened: {file_path:?}"),
                        Err(e) => eprintln!("Failed to open file: {}", e),
                    }
                }
            }
            "save" => {
                println!("Saving document");
                let can_save = state.read().current_file_path.is_some();
                if can_save {
                    match state.read().save_document() {
                        Ok(()) => println!("Document saved successfully"),
                        Err(e) => eprintln!("Failed to save file: {}", e),
                    }
                } else {
                    // No current file path, show Save As dialog
                    if let Some(file_path) = PlatformDialogs::show_save_dialog() {
                        match state.write().save_document_as(file_path.clone()) {
                            Ok(()) => println!("Document saved as: {:?}", file_path),
                            Err(e) => eprintln!("Failed to save file: {}", e),
                        }
                    }
                }
            }
            "save_as" => {
                println!("Showing Save As dialog");
                if let Some(file_path) = PlatformDialogs::show_save_dialog() {
                    match state.write().save_document_as(file_path.clone()) {
                        Ok(()) => println!("Document saved as: {:?}", file_path),
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
