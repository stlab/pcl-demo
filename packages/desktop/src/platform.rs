//! Platform-specific abstractions for desktop application
//!
//! This module factors out cfg-dependent code to improve rust-analyzer support.

use dioxus::desktop::muda::accelerator::{Accelerator, Code, Modifiers};
use dioxus::desktop::muda::{Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu};
use rfd::FileDialog;

/// Platform-specific modifier key configuration.
pub struct PlatformModifiers {
    /// The base modifier key for menu shortcuts (Cmd on macOS, Ctrl elsewhere)
    pub base: Modifiers,
}

impl PlatformModifiers {
    /// Returns the platform-appropriate modifier configuration.
    pub fn new() -> Self {
        Self {
            base: if cfg!(target_os = "macos") {
                Modifiers::META
            } else {
                Modifiers::CONTROL
            },
        }
    }

    /// Returns an accelerator triggered by `key` with `base` modifier.
    pub fn menu_key(&self, key: Code) -> Option<Accelerator> {
        Some(Accelerator::new(Some(self.base), key))
    }

    /// Returns an accelerator triggered by `key` with `modifiers`.
    #[allow(dead_code)]
    pub fn custom_key(&self, key: Code, modifiers: Modifiers) -> Option<Accelerator> {
        Some(Accelerator::new(Some(modifiers), key))
    }

    /// Returns an accelerator triggered by `key` with `base` plus `additional` modifiers.
    pub fn extended_key(&self, key: Code, additional: Modifiers) -> Option<Accelerator> {
        Some(Accelerator::new(Some(self.base | additional), key))
    }
}

/// Platform-specific file dialog operations.
pub struct PlatformDialogs;

impl PlatformDialogs {
    /// Presents an open file dialog and returns the user's selection (or `None` if canceled).
    pub fn file_from_open_dialog() -> Option<std::path::PathBuf> {
        FileDialog::new()
            .add_filter("JSON Documents", &["json"])
            .add_filter("All Files", &["*"])
            .set_title("Open Document")
            .pick_file()
    }

    /// Presents a save file dialog and returns the user's selection (or `None` if canceled).
    pub fn path_from_save_dialog() -> Option<std::path::PathBuf> {
        FileDialog::new()
            .add_filter("JSON Documents", &["json"])
            .add_filter("All Files", &["*"])
            .set_title("Save Document")
            .set_file_name("document.json")
            .save_file()
    }
}

/// Platform-specific menu creation utilities.
pub struct PlatformMenu;

impl PlatformMenu {
    /// Returns the application menu bar.
    pub fn create_menu_bar() -> Menu {
        let menu_bar = Menu::new();
        let modifiers = PlatformModifiers::new();

        // Add platform-specific app menu on macOS
        add_app_menu_if_needed(&menu_bar);

        // Create File submenu
        let file_menu = Submenu::new("File", true);

        // Add File menu items with explicit IDs
        append_menu_item(&file_menu, "new", "New", modifiers.menu_key(Code::KeyN));
        append_menu_item(&file_menu, "open", "Open", modifiers.menu_key(Code::KeyO));
        append_menu_item(&file_menu, "save", "Save", modifiers.menu_key(Code::KeyS));
        append_menu_item(
            &file_menu,
            "save_as",
            "Save As...",
            modifiers.extended_key(Code::KeyS, Modifiers::SHIFT),
        );
        file_menu
            .append(&PredefinedMenuItem::separator())
            .expect("Failed to append separator to File menu");
        file_menu
            .append(&PredefinedMenuItem::quit(Some("Quit")))
            .expect("Failed to append Quit menu item");

        // Add File submenu to main menu
        menu_bar
            .append(&file_menu)
            .expect("Failed to append File menu to menu bar");

        menu_bar
    }
}

/// Adds application menu on macOS to ensure File menu shows correctly.
fn add_app_menu_if_needed(menu_bar: &dioxus::desktop::muda::Menu) {
    if cfg!(target_os = "macos") {
        let app_menu = Submenu::new("CodeLess", true);
        menu_bar
            .append(&app_menu)
            .expect("Failed to append app menu on macOS");
    }
    // No app menu needed on other platforms
}

/// Appends a menu item with the given parameters.
fn append_menu_item(
    submenu: &dioxus::desktop::muda::Submenu,
    id: &str,
    text: &str,
    accelerator: Option<Accelerator>,
) {
    submenu
        .append(&MenuItem::with_id(MenuId::new(id), text, true, accelerator))
        .expect(&format!("Failed to append menu item '{}' to submenu", text));
}
