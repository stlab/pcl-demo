use crate::application_state::ApplicationState;
use crate::platform::{
    delete_document, file_size, load_document, save_document, saved_files, share_document_mobile,
};
use crate::Document;
use dioxus::prelude::*;

// Mobile-specific imports
use serde_json::{from_str, to_string_pretty};
use std::path::PathBuf;

/// Individual menu item in the bottom sheet.
#[component]
fn MenuItem(icon: String, title: String, subtitle: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "mobile-menu-item",
            onclick: move |e| onclick.call(e),
            div { class: "menu-item-icon", "{icon}" }
            div {
                class: "menu-item-content",
                div { class: "menu-item-title", "{title}" }
                div { class: "menu-item-subtitle", "{subtitle}" }
            }
        }
    }
}

/// Menu item for creating a new document.
#[component]
fn NewMenuItem(mut state: Signal<ApplicationState>, mut menu_open: Signal<bool>) -> Element {
    let handle_click = move |_| {
        state.write().new_document();
        menu_open.set(false);
    };

    rsx! {
        MenuItem {
            icon: "📄",
            title: "New",
            subtitle: "Create a new document",
            onclick: handle_click,
        }
    }
}

/// Menu item for opening a saved document.
#[component]
fn OpenMenuItem(
    mut menu_open: Signal<bool>,
    mut file_list_open: Signal<bool>,
    mut saved_files_list: Signal<Vec<String>>,
    mut error_message: Signal<Option<String>>,
) -> Element {
    let file_count = saved_files_list.read().len();
    
    let handle_click = move |_| {
        match saved_files() {
            Ok(files) => {
                saved_files_list.set(files);
                error_message.set(None);
                file_list_open.set(true);
                menu_open.set(false);
            }
            Err(e) => {
                error_message.set(Some(format!("Failed to load saved files: {e}")));
                saved_files_list.set(vec![]);
                file_list_open.set(true);
                menu_open.set(false);
            }
        }
    };

    rsx! {
        MenuItem {
            icon: "📂",
            title: "Open",
            subtitle: "Browse saved documents ({file_count} files)",
            onclick: handle_click,
        }
    }
}

/// Menu item for saving the current document.
#[component]
fn SaveMenuItem(
    mut state: Signal<ApplicationState>,
    mut menu_open: Signal<bool>,
    mut saved_files_list: Signal<Vec<String>>,
    mut error_message: Signal<Option<String>>,
) -> Element {
    let handle_click = move |_| {
        let current_state = state.read();
        match to_string_pretty(&current_state.the_only_document) {
            Ok(json_content) => {
                let filename = current_state
                    .current_file_path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("document.json");

                match save_document(&json_content, filename) {
                    Ok(_) => match saved_files() {
                        Ok(files) => {
                            saved_files_list.set(files);
                            error_message.set(None);
                        }
                        Err(e) => error_message
                            .set(Some(format!("Failed to refresh file list after save: {e}"))),
                    },
                    Err(e) => error_message.set(Some(format!("Failed to save document: {e}"))),
                }
            }
            Err(e) => {
                error_message.set(Some(format!("Failed to serialize document for save: {e}")));
            }
        }
        menu_open.set(false);
    };

    rsx! {
        MenuItem {
            icon: "💾",
            title: "Save",
            subtitle: "Save current document",
            onclick: handle_click,
        }
    }
}

/// Menu item for saving the document with a new name.
#[component]
fn SaveAsMenuItem(
    mut state: Signal<ApplicationState>,
    mut menu_open: Signal<bool>,
    mut filename_prompt_open: Signal<bool>,
    mut filename_input: Signal<String>,
) -> Element {
    let handle_click = move |_| {
        let current_name = {
            let current_state = state.read();
            current_state
                .current_file_path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("document")
                .replace(".json", "")
        };

        filename_input.set(current_name);
        filename_prompt_open.set(true);
        menu_open.set(false);
    };

    rsx! {
        MenuItem {
            icon: "📋",
            title: "Save As",
            subtitle: "Save with new name",
            onclick: handle_click,
        }
    }
}

/// Menu item for sharing the document.
#[component]
fn ShareMenuItem(
    mut state: Signal<ApplicationState>,
    mut menu_open: Signal<bool>,
    mut error_message: Signal<Option<String>>,
) -> Element {
    let handle_click = move |_| {
        let current_state = state.read();
        match to_string_pretty(&current_state.the_only_document) {
            Ok(json_content) => {
                share_document_mobile(&json_content);
            }
            Err(e) => {
                error_message.set(Some(format!("Failed to serialize document for share: {e}")));
            }
        }
        menu_open.set(false);
    };

    rsx! {
        button {
            class: "mobile-menu-item mobile-menu-item-share",
            onclick: handle_click,
            div { class: "menu-item-icon", "📤" }
            div {
                class: "menu-item-content",
                div { class: "menu-item-title", "Share" }
                div { class: "menu-item-subtitle", "Share document with other apps" }
            }
        }
    }
}

/// Bottom sheet menu with file actions.
#[component]
fn MenuBottomSheet(
    state: Signal<ApplicationState>,
    mut menu_open: Signal<bool>,
    file_list_open: Signal<bool>,
    filename_prompt_open: Signal<bool>,
    saved_files_list: Signal<Vec<String>>,
    error_message: Signal<Option<String>>,
    filename_input: Signal<String>,
) -> Element {
    let close_menu = move |_| menu_open.set(false);

    rsx! {
        div { class: "menu-overlay", onclick: close_menu }
        div {
            class: "bottom-sheet",
            div {
                class: "bottom-sheet-header",
                h3 { "File Menu" }
                button { class: "close-button", onclick: close_menu, "✕" }
            }
            div {
                class: "menu-actions",
                NewMenuItem { state, menu_open }
                OpenMenuItem { menu_open, file_list_open, saved_files_list, error_message }
                SaveMenuItem { state, menu_open, saved_files_list, error_message }
                SaveAsMenuItem { state, menu_open, filename_prompt_open, filename_input }
                ShareMenuItem { state, menu_open, error_message }
            }
        }
    }
}

/// Individual file item in the file list.
#[component]
fn FileItem(filename: String, on_open: EventHandler<String>, on_delete: EventHandler<String>) -> Element {
    let open_filename = filename.clone();
    let delete_filename = filename.clone();
    let size = file_size(&filename).unwrap_or(0);

    rsx! {
        div {
            class: "file-item",
            button {
                class: "file-item-button",
                onclick: move |_| on_open.call(open_filename.clone()),
                div { class: "file-item-icon", "📄" }
                div {
                    class: "file-item-info",
                    div { class: "file-item-name", "{filename}" }
                    div { class: "file-item-size", "{size} bytes" }
                }
            }
            button {
                class: "file-delete-button",
                onclick: move |_| on_delete.call(delete_filename.clone()),
                title: "Delete file",
                "🗑️"
            }
        }
    }
}

/// Modal showing list of saved files.
#[component]
fn FileListModal(
    mut state: Signal<ApplicationState>,
    mut file_list_open: Signal<bool>,
    mut saved_files_list: Signal<Vec<String>>,
    mut error_message: Signal<Option<String>>,
) -> Element {
    let close_file_list = move |_| file_list_open.set(false);

    let handle_file_open = move |filename: String| {
        match load_document(&filename) {
            Ok(content) => match from_str::<Document>(&content) {
                Ok(document) => {
                    state.write().the_only_document = document;
                    state.write().current_file_path = Some(PathBuf::from(&filename));
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!(
                        "Failed to parse document from file {filename}: {e}"
                    )));
                }
            },
            Err(e) => {
                error_message.set(Some(format!("Failed to load document {filename}: {e}")));
            }
        }
        file_list_open.set(false);
    };

    let handle_file_delete = move |filename: String| match delete_document(&filename) {
        Ok(_) => match saved_files() {
            Ok(files) => {
                saved_files_list.set(files);
                error_message.set(None);
            }
            Err(e) => error_message.set(Some(format!(
                "Failed to refresh file list after delete: {e}"
            ))),
        },
        Err(e) => error_message.set(Some(format!("Failed to delete document: {e}"))),
    };

    rsx! {
        div { class: "menu-overlay", onclick: close_file_list }
        div {
            class: "file-list-modal",
            div {
                class: "file-list-header",
                h3 { "Saved Documents" }
                button { class: "close-button", onclick: close_file_list, "✕" }
            }
            div {
                class: "file-list-content",
                if saved_files_list.read().is_empty() {
                    div {
                        class: "empty-state",
                        div { class: "empty-icon", "📄" }
                        div { class: "empty-title", "No saved documents" }
                        div { class: "empty-subtitle", "Create and save a document to see it here" }
                    }
                } else {
                    for filename in saved_files_list.read().iter() {
                        FileItem {
                            key: "{filename}",
                            filename: filename.clone(),
                            on_open: handle_file_open,
                            on_delete: handle_file_delete,
                        }
                    }
                }
            }
        }
    }
}

/// Modal for entering a filename when saving.
#[component]
fn FilenamePromptModal(
    mut state: Signal<ApplicationState>,
    mut filename_prompt_open: Signal<bool>,
    mut filename_input: Signal<String>,
    mut saved_files_list: Signal<Vec<String>>,
    mut error_message: Signal<Option<String>>,
) -> Element {
    let close_prompt = move |_| filename_prompt_open.set(false);

    let save_with_filename = move |_| {
        let filename = filename_input.read().clone();
        if !filename.trim().is_empty() {
            let json_content = {
                let current_state = state.read();
                to_string_pretty(&current_state.the_only_document)
            };

            match json_content {
                Ok(json_content) => {
                    let filename = if filename.ends_with(".json") {
                        filename
                    } else {
                        format!("{filename}.json")
                    };

                    match save_document(&json_content, &filename) {
                        Ok(_) => {
                            {
                                let mut app_state = state.write();
                                app_state.current_file_path = Some(PathBuf::from(&filename));
                            }
                            match saved_files() {
                                Ok(files) => {
                                    saved_files_list.set(files);
                                    error_message.set(None);
                                }
                                Err(e) => error_message
                                    .set(Some(format!("Failed to refresh file list: {e}"))),
                            }
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Failed to save document: {e}")));
                        }
                    }
                }
                Err(e) => {
                    error_message.set(Some(format!(
                        "Failed to serialize document for save with filename: {e}"
                    )));
                }
            }
        }
        filename_prompt_open.set(false);
    };

    let handle_filename_input = move |event: FormEvent| {
        filename_input.set(event.value());
    };

    let handle_filename_keypress = move |event: KeyboardEvent| {
        if event.key() == Key::Enter {
            let filename = filename_input.read().clone();
            if !filename.trim().is_empty() {
                let json_content = {
                    let current_state = state.read();
                    to_string_pretty(&current_state.the_only_document)
                };

                match json_content {
                    Ok(json_content) => {
                        let filename = if filename.ends_with(".json") {
                            filename
                        } else {
                            format!("{filename}.json")
                        };

                        match save_document(&json_content, &filename) {
                            Ok(_) => {
                                {
                                    let mut app_state = state.write();
                                    app_state.current_file_path = Some(PathBuf::from(&filename));
                                }
                                match saved_files() {
                                    Ok(files) => saved_files_list.set(files),
                                    Err(e) => error_message.set(Some(format!(
                                        "Failed to refresh file list after save: {e}"
                                    ))),
                                }
                            }
                            Err(e) => {
                                error_message.set(Some(format!("Failed to save document: {e}")))
                            }
                        }
                    }
                    Err(e) => {
                        error_message.set(Some(format!(
                            "Failed to serialize document for keypress save: {e}"
                        )));
                    }
                }
            }
            filename_prompt_open.set(false);
        }
    };

    rsx! {
        div { class: "menu-overlay", onclick: close_prompt }
        div {
            class: "filename-prompt-modal",
            div {
                class: "filename-prompt-header",
                h3 { "Save As" }
                button { class: "close-button", onclick: close_prompt, "✕" }
            }
            div {
                class: "filename-prompt-content",
                div {
                    class: "filename-prompt-field",
                    label { r#for: "filename-input", "Filename:" }
                    input {
                        id: "filename-input",
                        class: "filename-input",
                        r#type: "text",
                        value: "{filename_input}",
                        placeholder: "Enter filename",
                        oninput: handle_filename_input,
                        onkeypress: handle_filename_keypress,
                    }
                    div { class: "filename-hint", ".json extension will be added automatically" }
                }
                div {
                    class: "filename-prompt-buttons",
                    button { class: "filename-button filename-cancel", onclick: close_prompt, "Cancel" }
                    button { class: "filename-button filename-save", onclick: save_with_filename, "Save" }
                }
            }
        }
    }
}

/// The mobile app's file menu.
#[component]
pub fn MobileFileMenu(application_state: Signal<ApplicationState>) -> Element {
    let state = application_state;
    let mut menu_open = use_signal(|| false);
    let file_list_open = use_signal(|| false);
    let filename_prompt_open = use_signal(|| false);
    let filename_input = use_signal(String::new);
    let saved_files_list = use_signal(|| saved_files().unwrap_or_default());
    let error_message = use_signal(|| None::<String>);

    let toggle_menu = move |_| {
        let current = menu_open();
        menu_open.set(!current);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/styling/mobile_file_menu.css") }

        // Mobile file menu UI
        div {
            class: "mobile-file-menu",

            // Floating Action Button for menu
            button {
                class: "fab-menu-button",
                onclick: toggle_menu,
                title: "File Menu",
                "⋮" // Three dots menu icon
            }

            // Bottom sheet menu
            if *menu_open.read() {
                MenuBottomSheet {
                    state,
                    menu_open,
                    file_list_open,
                    filename_prompt_open,
                    saved_files_list,
                    error_message,
                    filename_input,
                }
            }

            // File list modal
            if *file_list_open.read() {
                FileListModal {
                    state,
                    file_list_open,
                    saved_files_list,
                    error_message,
                }
            }

            // Filename prompt modal
            if *filename_prompt_open.read() {
                FilenamePromptModal {
                    state,
                    filename_prompt_open,
                    filename_input,
                    saved_files_list,
                    error_message,
                }
            }
        }
    }
}
