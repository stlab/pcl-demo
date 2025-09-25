use dioxus::{desktop::{WindowBuilder, Config, use_muda_event_handler}, prelude::*, LaunchBuilder};

use ui::{ApplicationState, DocumentUI};

mod platform;
use platform::{PlatformDialogs, PlatformMenu};

/// The top-level stylesheet for the application.
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Helper function to handle file operation results with consistent error reporting
fn handle_file_result<T, E: std::fmt::Display>(result: Result<T, E>, operation: &str) {
    match result {
        Ok(_) => {},
        Err(e) => eprintln!("Failed to {}: {}", operation, e),
    }
}

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
        match event.id.0.as_str() {
            "new" => {
                state.write().new_document();
            }
            "open" => {
                if let Some(file_path) = PlatformDialogs::show_open_dialog() {
                    handle_file_result(state.write().load_document(&file_path), "open file");
                }
            }
            "save" => {
                let can_save = state.read().current_file_path.is_some();
                if can_save {
                    handle_file_result(state.read().save_document(), "save file");
                } else {
                    if let Some(file_path) = PlatformDialogs::show_save_dialog() {
                        handle_file_result(state.write().save_document_as(&file_path), "save file");
                    }
                }
            }
            "save_as" => {
                if let Some(file_path) = PlatformDialogs::show_save_dialog() {
                    handle_file_result(state.write().save_document_as(&file_path), "save file");
                }
            }
            _ => { unreachable!("unknown menu item {:?}", event.id.as_ref()) }
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        DocumentUI { application_state: state }

    }
}
