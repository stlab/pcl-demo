use dioxus::prelude::*;
use crate::application_state::*;
use crate::platform::{get_saved_files, get_file_size_impl, save_document, load_document, delete_document, share_document_mobile};
use crate::Document;

// Mobile-specific imports
use std::path::PathBuf;
use serde_json::{to_string_pretty, from_str};

const MOBILE_FILE_MENU_CSS: Asset = asset!("/assets/styling/mobile_file_menu.css");

/// Mobile file menu component with touch-optimized UI and real file operations
#[component]
pub fn MobileFileMenu(application_state: Signal<ApplicationState>) -> Element {
    
    let mut state = application_state;
    let mut menu_open = use_signal(|| false);
    let mut file_list_open = use_signal(|| false);
    let mut filename_prompt_open = use_signal(|| false);
    let mut filename_input = use_signal(|| String::new());
    let mut saved_files = use_signal(|| get_saved_files().unwrap_or_default());
    
    let handle_new = move |_| {
        state.write().new_document();
        menu_open.set(false);
    };
    
    let handle_open = move |_| {
        match get_saved_files() {
            Ok(files) => {
                saved_files.set(files);
                file_list_open.set(true);
                menu_open.set(false);
            }
            Err(e) => {
                eprintln!("Failed to load saved files: {}", e);
                // Still open the dialog but with empty list
                saved_files.set(vec![]);
                file_list_open.set(true);
                menu_open.set(false);
            }
        }
    };
    
    let handle_save = move |_| {
        let current_state = state.read();
        if let Ok(json_content) = to_string_pretty(&current_state.the_only_document) {
            let filename = current_state.current_file_path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("document.json");
            
            match save_document(&json_content, filename) {
                Ok(_) => {
                    match get_saved_files() {
                        Ok(files) => saved_files.set(files),
                        Err(e) => eprintln!("Failed to refresh file list after save: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to save document: {}", e),
            }
        }
        menu_open.set(false);
    };
    
    let handle_save_as = move |_| {
        let current_name = {
            let current_state = state.read();
            current_state.current_file_path
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
    
    let handle_share = move |_| {
        let current_state = state.read();
        if let Ok(json_content) = to_string_pretty(&current_state.the_only_document) {
            share_document_mobile(&json_content);
        }
        menu_open.set(false);
    };
    
    let toggle_menu = move |_| {
        let current_state = *menu_open.read();
        menu_open.set(!current_state);
    };
    
    let close_menu = move |_| {
        menu_open.set(false);
    };
    
    let close_file_list = move |_| {
        file_list_open.set(false);
    };
    
    let close_filename_prompt = move |_| {
        filename_prompt_open.set(false);
    };
    
    let save_with_filename = move |_| {
        let filename = filename_input.read().clone();
        if !filename.trim().is_empty() {
            let json_content = {
                let current_state = state.read();
                to_string_pretty(&current_state.the_only_document)
            };
            
            if let Ok(json_content) = json_content {
                let filename = if filename.ends_with(".json") {
                    filename
                } else {
                    format!("{}.json", filename)
                };
                
                                                if let Ok(_) = save_document(&json_content, &filename) {
                    {
                        let mut app_state = state.write();
                        app_state.current_file_path = Some(PathBuf::from(&filename));
                    }
                    match get_saved_files() {
                        Ok(files) => saved_files.set(files),
                        Err(e) => eprintln!("Failed to refresh file list: {}", e),
                    }
                    println!("Mobile: Saved as {}", filename);
                }
            }
        }
        filename_prompt_open.set(false);
    };
    
    let mut handle_file_open = move |filename: String| {
        if let Ok(content) = load_document(&filename) {
            match from_str::<Document>(&content) {
                Ok(document) => {
                    state.write().the_only_document = document;
                    state.write().current_file_path = Some(PathBuf::from(&filename));
                }
                Err(_) => {}
            }
        }
        file_list_open.set(false);
    };
    
    let mut handle_file_delete = move |filename: String| {
        match delete_document(&filename) {
            Ok(_) => {
                match get_saved_files() {
                    Ok(files) => saved_files.set(files),
                    Err(e) => eprintln!("Failed to refresh file list after delete: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to delete document: {}", e),
        }
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
                
                if let Ok(json_content) = json_content {
                    let filename = if filename.ends_with(".json") {
                        filename
                    } else {
                        format!("{}.json", filename)
                    };
                    
                    match save_document(&json_content, &filename) {
                        Ok(_) => {
                            {
                                let mut app_state = state.write();
                                app_state.current_file_path = Some(PathBuf::from(&filename));
                            }
                            match get_saved_files() {
                                Ok(files) => saved_files.set(files),
                                Err(e) => eprintln!("Failed to refresh file list after save: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Failed to save document: {}", e),
                    }
                }
            }
            filename_prompt_open.set(false);
        }
    };
    

    rsx! {
        document::Link { rel: "stylesheet", href: MOBILE_FILE_MENU_CSS }
        
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
            
            // Bottom sheet menu (shown when menu_open is true)
            if *menu_open.read() {
                div {
                    class: "menu-overlay",
                    onclick: close_menu
                }
                div {
                    class: "bottom-sheet",
                    div {
                        class: "bottom-sheet-header",
                        h3 { "File Menu" }
                        button {
                            class: "close-button",
                            onclick: close_menu,
                            "✕"
                        }
                    }
                    div {
                        class: "menu-actions",
                        
                        button {
                            class: "mobile-menu-item",
                            onclick: handle_new,
                            div {
                                class: "menu-item-icon",
                                "📄"
                            }
                            div {
                                class: "menu-item-content",
                                div { class: "menu-item-title", "New" }
                                div { class: "menu-item-subtitle", "Create a new document" }
                            }
                        }
                        
                        button {
                            class: "mobile-menu-item",
                            onclick: handle_open,
                            div {
                                class: "menu-item-icon",
                                "📂"
                            }
                            div {
                                class: "menu-item-content",
                                div { class: "menu-item-title", "Open" }
                                div { class: "menu-item-subtitle", "Browse saved documents ({saved_files.read().len()} files)" }
                            }
                        }
                        
                        button {
                            class: "mobile-menu-item",
                            onclick: handle_save,
                            div {
                                class: "menu-item-icon",
                                "💾"
                            }
                            div {
                                class: "menu-item-content",
                                div { class: "menu-item-title", "Save" }
                                div { class: "menu-item-subtitle", "Save current document" }
                            }
                        }
                        
                        button {
                            class: "mobile-menu-item",
                            onclick: handle_save_as,
                            div {
                                class: "menu-item-icon",
                                "📋"
                            }
                            div {
                                class: "menu-item-content",
                                div { class: "menu-item-title", "Save As" }
                                div { class: "menu-item-subtitle", "Save with new name" }
                            }
                        }
                        
                        button {
                            class: "mobile-menu-item mobile-menu-item-share",
                            onclick: handle_share,
                            div {
                                class: "menu-item-icon",
                                "📤"
                            }
                            div {
                                class: "menu-item-content",
                                div { class: "menu-item-title", "Share" }
                                div { class: "menu-item-subtitle", "Share document with other apps" }
                            }
                        }
                    }
                }
            }
            
            // File list modal (shown when file_list_open is true)
            if *file_list_open.read() {
                div {
                    class: "menu-overlay",
                    onclick: close_file_list
                }
                div {
                    class: "file-list-modal",
                    div {
                        class: "file-list-header",
                        h3 { "Saved Documents" }
                        button {
                            class: "close-button",
                            onclick: close_file_list,
                            "✕"
                        }
                    }
                    div {
                        class: "file-list-content",
                        if saved_files.read().is_empty() {
                            div {
                                class: "empty-state",
                                div { class: "empty-icon", "📄" }
                                div { class: "empty-title", "No saved documents" }
                                div { class: "empty-subtitle", "Create and save a document to see it here" }
                            }
                        } else {
                            for filename in saved_files.read().iter() {
                                div {
                                    class: "file-item",
                                    button {
                                        class: "file-item-button",
                                        onclick: {
                                            let filename = filename.clone();
                                            move |_| handle_file_open(filename.clone())
                                        },
                                        div {
                                            class: "file-item-icon",
                                            "📄"
                                        }
                                        div {
                                            class: "file-item-info",
                                            div { class: "file-item-name", "{filename}" }
                                            div { class: "file-item-size", "{get_file_size_impl(filename).unwrap_or(0)} bytes" }
                                        }
                                    }
                                    button {
                                        class: "file-delete-button",
                                        onclick: {
                                            let filename = filename.clone();
                                            move |_| handle_file_delete(filename.clone())
                                        },
                                        title: "Delete file",
                                        "🗑️"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Filename prompt modal (shown when filename_prompt_open is true)
            if *filename_prompt_open.read() {
                div {
                    class: "menu-overlay",
                    onclick: close_filename_prompt
                }
                div {
                    class: "filename-prompt-modal",
                    div {
                        class: "filename-prompt-header",
                        h3 { "Save As" }
                        button {
                            class: "close-button",
                            onclick: close_filename_prompt,
                            "✕"
                        }
                    }
                    div {
                        class: "filename-prompt-content",
                        div {
                            class: "filename-prompt-field",
                            label { 
                                r#for: "filename-input",
                                "Filename:"
                            }
                            input {
                                id: "filename-input",
                                class: "filename-input",
                                r#type: "text",
                                value: "{filename_input}",
                                placeholder: "Enter filename",
                                oninput: handle_filename_input,
                                onkeypress: handle_filename_keypress
                            }
                            div { class: "filename-hint", ".json extension will be added automatically" }
                        }
                        div {
                            class: "filename-prompt-buttons",
                            button {
                                class: "filename-button filename-cancel",
                                onclick: close_filename_prompt,
                                "Cancel"
                            }
                            button {
                                class: "filename-button filename-save",
                                onclick: save_with_filename,
                                "Save"
                            }
                        }
                    }
                }
            }
        }
    }
}