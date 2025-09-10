use dioxus::{desktop::WindowBuilder, prelude::*};
use dioxus::desktop::muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};

use ui::{ApplicationState, DocumentUI};

/// The top-level stylesheet for the application.
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Runs the application.
fn main() {
    // Create the main menu bar
    let menu = create_menu_bar();

    // Nonstandard startup so the application window doesn't float on
    // top of those of other applications.
    dioxus::LaunchBuilder::desktop().with_cfg(
        dioxus::desktop::Config::default()
            .with_window(
                WindowBuilder::new()
                    .with_always_on_top(false))
            .with_menu(menu))
    .launch(AppUI);
}

/// Creates the application menu bar with File menu
fn create_menu_bar() -> Menu {
    let menu = Menu::new();
    
    // Create File submenu
    let file_submenu = Submenu::new("File", true);
    
    // Add File menu items
    let new_item = MenuItem::new("New", true, None);
    let open_item = MenuItem::new("Open...", true, Some(dioxus::desktop::muda::accelerator::Accelerator::new(Some(dioxus::desktop::muda::accelerator::Modifiers::CONTROL), dioxus::desktop::muda::accelerator::Code::KeyO)));
    let save_item = MenuItem::new("Save", true, Some(dioxus::desktop::muda::accelerator::Accelerator::new(Some(dioxus::desktop::muda::accelerator::Modifiers::CONTROL), dioxus::desktop::muda::accelerator::Code::KeyS)));
    let save_as_item = MenuItem::new("Save As...", true, Some(dioxus::desktop::muda::accelerator::Accelerator::new(Some(dioxus::desktop::muda::accelerator::Modifiers::CONTROL | dioxus::desktop::muda::accelerator::Modifiers::SHIFT), dioxus::desktop::muda::accelerator::Code::KeyS)));
    let separator = PredefinedMenuItem::separator();
    let quit_item = PredefinedMenuItem::quit(Some("Quit"));
    
    // Add items to File submenu
    file_submenu.append(&new_item).unwrap();
    file_submenu.append(&open_item).unwrap();
    file_submenu.append(&save_item).unwrap();
    file_submenu.append(&save_as_item).unwrap();
    file_submenu.append(&separator).unwrap();
    file_submenu.append(&quit_item).unwrap();
    
    // Add File submenu to main menu
    menu.append(&file_submenu).unwrap();
    
    menu
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
