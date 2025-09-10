use dioxus::{desktop::WindowBuilder, prelude::*};
use dioxus::desktop::muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use dioxus::desktop::muda::accelerator::{Accelerator, Modifiers, Code};

use ui::{ApplicationState, DocumentUI};

/// The top-level stylesheet for the application.
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Runs the application.
fn main() {
    // Create the main menu bar
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
    
    // Add File menu items
    let new_item = MenuItem::new("New", true, menu_key(Code::KeyN));
    let open_item = MenuItem::new("Open...", true, menu_key(Code::KeyO));
    let save_item = MenuItem::new("Save", true, menu_key(Code::KeyS));
    let save_as_item = MenuItem::new("Save As...", true, nonstandard_menu_key(Code::KeyS, BASE_MODIFIER | Modifiers::SHIFT));
    let separator = PredefinedMenuItem::separator();
    let quit_item = PredefinedMenuItem::quit(Some("Quit"));
    
    // Add items to File submenu
    file_menu.append(&new_item).unwrap();
    file_menu.append(&open_item).unwrap();
    file_menu.append(&save_item).unwrap();
    file_menu.append(&save_as_item).unwrap();
    file_menu.append(&separator).unwrap();
    file_menu.append(&quit_item).unwrap();
    
    // Add File submenu to main menu
    menu_bar.append(&file_menu).unwrap();
    
    menu_bar
}

/// The top-level UI element.
#[component]
fn AppUI() -> Element {

    // The state of the whole application
    let state = use_signal(|| ApplicationState::new());

    // Handle menu events
    dioxus::desktop::use_muda_event_handler(move |event| {
        let dioxus::desktop::muda::MenuEvent { id } = event;
        match id.0.as_str() {
            "New" => {
                println!("New file requested");
                // TODO: Implement new file functionality
            }
            "Open..." => {
                println!("Open file requested");
                // TODO: Implement open file functionality
            }
            "Save" => {
                println!("Save file requested");
                // TODO: Implement save file functionality
            }
            "Save As..." => {
                println!("Save As file requested");
                // TODO: Implement save as file functionality
            }
            _ => {}
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        DocumentUI { application_state: state }

    }
}
