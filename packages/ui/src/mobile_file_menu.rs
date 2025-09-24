use dioxus::prelude::*;
use crate::application_state::*;

// Mobile-specific imports
use std::path::PathBuf;
use std::fs;

const MOBILE_FILE_MENU_CSS: Asset = asset!("/assets/styling/mobile_file_menu.css");

/// Mobile file menu component with touch-optimized UI and real file operations
#[component]
pub fn MobileFileMenu(application_state: Signal<ApplicationState>) -> Element {
    
    let mut state = application_state;
    let mut menu_open = use_signal(|| false);
    let mut file_list_open = use_signal(|| false);
    let mut filename_prompt_open = use_signal(|| false);
    let mut filename_input = use_signal(|| String::new());
    let mut saved_files = use_signal(|| get_saved_files());
    
    // Handler for creating a new document
    let handle_new = move |_| {
        state.write().new_document();
        menu_open.set(false);
    };
    
    // Handler for opening the file list
    let handle_open = move |_| {
        saved_files.set(get_saved_files()); // Refresh file list
        file_list_open.set(true);
        menu_open.set(false);
    };
    
    // Handler for saving the current document
    let handle_save = move |_| {
        let current_state = state.read();
        if let Ok(json_content) = serde_json::to_string_pretty(&current_state.the_only_document) {
            let filename = current_state.current_file_path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("document.json");
            
            if save_document_to_storage(&json_content, filename) {
                saved_files.set(get_saved_files()); // Refresh file list
                println!("Mobile: Saved {}", filename);
            }
        }
        menu_open.set(false);
    };
    
    // Handler for save as (with user input for filename)
    let handle_save_as = move |_| {
        // Get current filename for default
        let current_name = {
            let current_state = state.read();
            current_state.current_file_path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("document")
                .replace(".json", "")
        };
        
        // Set default filename and show prompt
        filename_input.set(current_name);
        filename_prompt_open.set(true);
        menu_open.set(false);
    };
    
    // Handler for sharing the document (mobile-specific feature)
    let handle_share = move |_| {
        let current_state = state.read();
        if let Ok(json_content) = serde_json::to_string_pretty(&current_state.the_only_document) {
            share_document_mobile(&json_content);
        }
        menu_open.set(false);
    };
    
    // Toggle menu visibility
    let toggle_menu = move |_| {
        let current_state = *menu_open.read();
        menu_open.set(!current_state);
    };
    
    // Close menu when clicking outside
    let close_menu = move |_| {
        menu_open.set(false);
    };
    
    // Close file list
    let close_file_list = move |_| {
        file_list_open.set(false);
    };
    
    // Close filename prompt
    let close_filename_prompt = move |_| {
        filename_prompt_open.set(false);
    };
    
    // Save with entered filename
    let save_with_filename = move |_| {
        let filename = filename_input.read().clone();
        if !filename.trim().is_empty() {
            let json_content = {
                let current_state = state.read();
                serde_json::to_string_pretty(&current_state.the_only_document)
            };
            
            if let Ok(json_content) = json_content {
                let filename = if filename.ends_with(".json") {
                    filename
                } else {
                    format!("{}.json", filename)
                };
                
                if save_document_to_storage(&json_content, &filename) {
                    {
                        let mut app_state = state.write();
                        app_state.current_file_path = Some(std::path::PathBuf::from(&filename));
                    }
                    saved_files.set(get_saved_files()); // Refresh file list
                    println!("Mobile: Saved as {}", filename);
                }
            }
        }
        filename_prompt_open.set(false);
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
                                            let mut local_state = state.clone();
                                            let mut local_file_list_open = file_list_open.clone();
                                            move |_| {
                                                if let Some(content) = load_document_from_storage(&filename) {
                                                    match serde_json::from_str::<crate::Document>(&content) {
                                                        Ok(document) => {
                                                            local_state.write().the_only_document = document;
                                                            local_state.write().current_file_path = Some(std::path::PathBuf::from(&filename));
                                                            println!("Mobile: Opened {}", filename);
                                                        }
                                                        Err(e) => {
                                                            println!("Mobile: Failed to parse {}: {}", filename, e);
                                                        }
                                                    }
                                                }
                                                local_file_list_open.set(false);
                                            }
                                        },
                                        div {
                                            class: "file-item-icon",
                                            "📄"
                                        }
                                        div {
                                            class: "file-item-info",
                                            div { class: "file-item-name", "{filename}" }
                                            div { class: "file-item-size", "{get_file_size(filename)} bytes" }
                                        }
                                    }
                                    button {
                                        class: "file-delete-button",
                                        onclick: {
                                            let filename = filename.clone();
                                            let mut local_saved_files = saved_files.clone();
                                            move |_| {
                                                if delete_document_from_storage(&filename) {
                                                    local_saved_files.set(get_saved_files()); // Refresh file list
                                                    println!("Mobile: Deleted {}", filename);
                                                }
                                            }
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
                                oninput: move |event| {
                                    filename_input.set(event.value());
                                },
                                onkeypress: move |event| {
                                    if event.key() == dioxus::prelude::Key::Enter {
                                        let filename = filename_input.read().clone();
                                        if !filename.trim().is_empty() {
                                            let json_content = {
                                                let current_state = state.read();
                                                serde_json::to_string_pretty(&current_state.the_only_document)
                                            };
                                            
                                            if let Ok(json_content) = json_content {
                                                let filename = if filename.ends_with(".json") {
                                                    filename
                                                } else {
                                                    format!("{}.json", filename)
                                                };
                                                
                                                if save_document_to_storage(&json_content, &filename) {
                                                    {
                                                        let mut app_state = state.write();
                                                        app_state.current_file_path = Some(std::path::PathBuf::from(&filename));
                                                    }
                                                    saved_files.set(get_saved_files()); // Refresh file list
                                                    println!("Mobile: Saved as {}", filename);
                                                }
                                            }
                                        }
                                        filename_prompt_open.set(false);
                                    }
                                }
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

// Mobile-specific file operations using persistent storage


/// Gets the storage directory for mobile app documents
fn get_storage_directory() -> PathBuf {
    #[cfg(target_os = "ios")]
    {
        // On iOS, try to use the app's Documents directory
        // First try getting the home directory, then fall back to temp
        let storage_dir = if let Some(home) = std::env::var_os("HOME") {
            let mut dir = PathBuf::from(home);
            dir.push("Documents");
            dir.push("mobile_documents");
            dir
        } else {
            // Fallback to temp directory
            let mut dir = std::env::temp_dir();
            dir.push("mobile_documents");
            dir
        };
        
        // Try to create the directory
        match fs::create_dir_all(&storage_dir) {
            Ok(_) => {
                println!("iOS: Created storage directory at {:?}", storage_dir);
                storage_dir
            }
            Err(e) => {
                println!("iOS: Failed to create storage directory {:?}: {}", storage_dir, e);
                // Fallback to temp directory
                let temp_dir = std::env::temp_dir();
                println!("iOS: Using temp directory: {:?}", temp_dir);
                temp_dir
            }
        }
    }
    
    #[cfg(target_os = "android")]
    {
        // On Android, try to use internal storage
        let storage_dir = if let Ok(current) = std::env::current_dir() {
            let mut dir = current;
            dir.push("mobile_documents");
            dir
        } else {
            let mut dir = std::env::temp_dir();
            dir.push("mobile_documents");
            dir
        };
        
        match fs::create_dir_all(&storage_dir) {
            Ok(_) => {
                println!("Android: Created storage directory at {:?}", storage_dir);
                storage_dir
            }
            Err(e) => {
                println!("Android: Failed to create storage directory {:?}: {}", storage_dir, e);
                std::env::temp_dir()
            }
        }
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        // Desktop/other platforms
        let mut storage_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        storage_dir.push("mobile_documents");
        
        match fs::create_dir_all(&storage_dir) {
            Ok(_) => {
                println!("Desktop: Created storage directory at {:?}", storage_dir);
                storage_dir
            }
            Err(e) => {
                println!("Desktop: Failed to create storage directory {:?}: {}", storage_dir, e);
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            }
        }
    }
}

/// Gets the list of saved files from storage
fn get_saved_files() -> Vec<String> {
    let storage_dir = get_storage_directory();
    println!("Mobile: Looking for files in {:?}", storage_dir);
    
    // Read all .json files from the storage directory
    let mut files = Vec::new();
    
    match fs::read_dir(&storage_dir) {
        Ok(entries) => {
            println!("Mobile: Successfully opened directory, reading entries...");
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            if filename_str.ends_with(".json") {
                                println!("Mobile: Found file: {}", filename_str);
                                files.push(filename_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Mobile: Failed to read directory {:?}: {}", storage_dir, e);
        }
    }
    
    // Add sample files if directory is empty (first run)
    if files.is_empty() {
        println!("Mobile: No files found, initializing sample files...");
        initialize_sample_files();
        // Re-read after initialization
        match fs::read_dir(&storage_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if let Some(filename) = path.file_name() {
                            if let Some(filename_str) = filename.to_str() {
                                if filename_str.ends_with(".json") {
                                    println!("Mobile: Found file after init: {}", filename_str);
                                    files.push(filename_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Mobile: Failed to re-read directory after init: {}", e);
            }
        }
    }
    
    files.sort(); // Sort alphabetically
    println!("Mobile: Returning {} files: {:?}", files.len(), files);
    files
}

/// Saves a document to mobile storage
fn save_document_to_storage(content: &str, filename: &str) -> bool {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    println!("Mobile: Attempting to save {} to {:?}", filename, file_path);
    println!("Mobile: Content length: {} bytes", content.len());
    
    match fs::write(&file_path, content) {
        Ok(_) => {
            println!("Mobile: Successfully saved {} to {:?}", filename, file_path);
            
            // Verify the file was actually written
            match fs::metadata(&file_path) {
                Ok(metadata) => {
                    println!("Mobile: File size on disk: {} bytes", metadata.len());
                }
                Err(e) => {
                    println!("Mobile: Warning: Could not read metadata for saved file: {}", e);
                }
            }
            
            true
        }
        Err(e) => {
            println!("Mobile: Failed to save {} to {:?}: {}", filename, file_path, e);
            false
        }
    }
}

/// Loads a document from mobile storage
fn load_document_from_storage(filename: &str) -> Option<String> {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            println!("Mobile: Loaded {} from {:?}", filename, file_path);
            Some(content)
        }
        Err(e) => {
            println!("Mobile: Failed to load {}: {}", filename, e);
            None
        }
    }
}

/// Deletes a document from mobile storage
fn delete_document_from_storage(filename: &str) -> bool {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    match fs::remove_file(&file_path) {
        Ok(_) => {
            println!("Mobile: Deleted {} from {:?}", filename, file_path);
            true
        }
        Err(e) => {
            println!("Mobile: Failed to delete {}: {}", filename, e);
            false
        }
    }
}

/// Gets the file size in bytes
fn get_file_size(filename: &str) -> usize {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    match fs::metadata(&file_path) {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => 0,
    }
}

/// Initializes sample files for first-time users
fn initialize_sample_files() {
    let sample_circle = r#"{
  "html": "<svg viewBox=\"0 0 70 70\" xmlns=\"http://www.w3.org/2000/svg\">\n<circle cx=\"35\" cy=\"35\" r=\"25\" fill=\"lightblue\" stroke=\"darkblue\" stroke-width=\"2\"/>\n<text x=\"35\" y=\"40\" text-anchor=\"middle\" font-size=\"8\">Circle Doc</text>\n</svg>"
}"#;
    
    let sample_square = r#"{
  "html": "<svg viewBox=\"0 0 70 70\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"15\" y=\"15\" width=\"40\" height=\"40\" fill=\"lightcoral\" stroke=\"darkred\" stroke-width=\"2\"/>\n<text x=\"35\" y=\"40\" text-anchor=\"middle\" font-size=\"8\">Square Doc</text>\n</svg>"
}"#;
    
    // Create sample files
    save_document_to_storage(sample_circle, "sample_circle.json");
    save_document_to_storage(sample_square, "sample_square.json");
    
    println!("Mobile: Initialized sample files for first run");
}

/// Shares a document using the mobile platform's share sheet
fn share_document_mobile(content: &str) {
    #[cfg(target_os = "android")]
    {
        // Android-specific share logic
        // In a real app, this might use Android's Intent system
        println!("Android: Opening share sheet");
    }
    
    #[cfg(target_os = "ios")]
    {
        // iOS-specific share logic  
        // In a real app, this might use iOS's UIActivityViewController
        println!("iOS: Opening activity view controller");
    }
    
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        // Fallback for testing on other platforms
        println!("Mobile: Would share document ({} chars)", content.len());
    }
}
