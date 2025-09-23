use dioxus::{desktop::WindowBuilder, prelude::*};
use dioxus::desktop::muda::{Menu, MenuItem, PredefinedMenuItem, Submenu, MenuId};
use dioxus::desktop::muda::accelerator::{Accelerator, Modifiers, Code};
use std::path::PathBuf;

use ui::{ApplicationState, DocumentUI};

/// The top-level stylesheet for the application.
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Shows an open file dialog and returns the selected path
fn show_open_dialog() -> Option<PathBuf> {
    use rfd::FileDialog;
    
    FileDialog::new()
        .add_filter("JSON Documents", &["json"])
        .add_filter("All Files", &["*"])
        .set_title("Open Document")
        .pick_file()
}

/// Shows a save file dialog and returns the selected path
fn show_save_dialog() -> Option<PathBuf> {
    use rfd::FileDialog;
    
    FileDialog::new()
        .add_filter("JSON Documents", &["json"])
        .add_filter("All Files", &["*"])
        .set_title("Save Document")
        .set_file_name("document.json")
        .save_file()
}

/// Runs the application.
fn main() {
    let menu_bar = create_menu_bar();

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

/// The standard modifier key used for menu keyboard shortcuts.
static BASE_MODIFIER: Modifiers = if cfg!(target_os = "macos") {
    Modifiers::META
} else {
    Modifiers::CONTROL
};

/// Returns an `Accelerator` triggered by `key` pressed with `BASE_MODIFIER`.
fn menu_key(key: Code) -> Option<Accelerator> {
    Some(Accelerator::new(Some(BASE_MODIFIER), key))
}

/// Returns an `Accelerator` triggered by `key` pressed with `modifiers`.
fn nonstandard_menu_key(key: Code, modifiers: Modifiers) -> Option<Accelerator> {
    Some(Accelerator::new(Some(modifiers), key))
}

/// Appends to `items` an enabled item with the given `id`, `text`, and `accelerator`.
fn append_item(items: &Submenu, id: &str, text: &str, accelerator: Option<Accelerator>) {
    items.append(&MenuItem::with_id(MenuId::new(id), text, true, accelerator)).unwrap();
}

/// Creates the application menu bar with File menu
fn create_menu_bar() -> Menu {
    let menu_bar = Menu::new();

    // On macOS, add an application menu to ensure File menu shows correctly
    // On Windows/Linux, this is not needed and File menu can be first
    #[cfg(target_os = "macos")]
    {
        let app_menu = Submenu::new("CodeLess", true);
        menu_bar.append(&app_menu).unwrap();
    }

    // Create File submenu
    let file_menu = Submenu::new("File", true);
    
    // Add File menu items with explicit IDs
    append_item(&file_menu, "new", "New", menu_key(Code::KeyN));
    append_item(&file_menu, "open", "Open", menu_key(Code::KeyO));
    append_item(&file_menu, "save", "Save", menu_key(Code::KeyS));
    append_item(&file_menu, "save_as", "Save As...", nonstandard_menu_key(Code::KeyS, BASE_MODIFIER | Modifiers::SHIFT));
    file_menu.append(&PredefinedMenuItem::separator()).unwrap();
    file_menu.append(&PredefinedMenuItem::quit(Some("Quit"))).unwrap();
    
    // Add File submenu to main menu
    menu_bar.append(&file_menu).unwrap();
    
    menu_bar
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
                if let Some(file_path) = show_open_dialog() {
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
                    if let Some(file_path) = show_save_dialog() {
                        match state.write().save_document_as(file_path.clone()) {
                            Ok(()) => println!("Document saved as: {:?}", file_path),
                            Err(e) => eprintln!("Failed to save file: {}", e),
                        }
                    }
                }
            }
            "save_as" => {
                println!("Showing Save As dialog");
                if let Some(file_path) = show_save_dialog() {
                    match state.write().save_document_as(file_path.clone()) {
                        Ok(()) => println!("Document saved as: {:?}", file_path),
                        Err(e) => eprintln!("Failed to save file: {}", e),
                    }
                }
            }
            _ => { unreachable!("unknown menu item {id.as_ref():?}") }
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        DocumentUI { application_state: state }

    }
}
