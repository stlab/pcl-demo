//! Platform-agnostic file operations interface
//!
//! This module provides a unified interface for file operations across different platforms,
//! factoring out cfg-dependent code to improve rust-analyzer support.

use std::path::PathBuf;
use std::fs;

// Web API imports (available on all platforms for development ease)
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Blob, Url, HtmlAnchorElement};

/// Result type for file operations
pub type FileOperationResult<T> = anyhow::Result<T>;

/// Trait for platform-specific file operations
pub trait FileOperations {
    /// Save a document to storage with the given filename
    fn save_document(&self, content: &str, filename: &str) -> FileOperationResult<()>;
    
    /// Load a document from storage
    fn load_document(&self, filename: &str) -> FileOperationResult<String>;
    
    /// Delete a document from storage
    fn delete_document(&self, filename: &str) -> FileOperationResult<()>;
    
    /// Get list of saved documents
    fn get_saved_documents(&self) -> FileOperationResult<Vec<String>>;
    
    /// Get file size in bytes
    fn get_file_size(&self, filename: &str) -> usize;
    
    /// Show platform-specific file picker for opening
    fn show_open_dialog(&self) -> Option<PathBuf>;
    
    /// Show platform-specific file picker for saving
    fn show_save_dialog(&self) -> Option<PathBuf>;
    
    /// Share document using platform-specific sharing mechanism
    fn share_document(&self, content: &str);
}

/// Unified file operations implementation that works on all platforms
pub struct UnifiedFileOperations;

impl FileOperations for UnifiedFileOperations {
    fn save_document(&self, content: &str, filename: &str) -> FileOperationResult<()> {
        if cfg!(target_arch = "wasm32") {
            // Web platform uses browser download
            download_file(content, filename);
            Ok(())
        } else if cfg!(feature = "mobile") {
            // Mobile platform uses persistent storage
            save_document_to_storage(content, filename)
        } else {
            // Desktop uses application state for saving
            Err(anyhow::anyhow!("Desktop platform saves through application state"))
        }
    }
    
    fn load_document(&self, filename: &str) -> FileOperationResult<String> {
        if cfg!(target_arch = "wasm32") {
            // Web platform loads through file input element
            Err(anyhow::anyhow!("Web platform loads files through file input"))
        } else if cfg!(feature = "mobile") {
            // Mobile platform loads from persistent storage
            load_document_from_storage(filename)
                .ok_or_else(|| anyhow::anyhow!("Failed to load document: {}", filename))
        } else {
            // Desktop uses application state for loading
            Err(anyhow::anyhow!("Desktop platform loads through application state"))
        }
    }
    
    fn delete_document(&self, filename: &str) -> FileOperationResult<()> {
        if cfg!(target_arch = "wasm32") {
            // Web platform doesn't support direct file deletion
            Err(anyhow::anyhow!("Web platform doesn't support file deletion"))
        } else if cfg!(feature = "mobile") {
            // Mobile platform deletes from persistent storage
            if delete_document_from_storage(filename) {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to delete document: {}", filename))
            }
        } else {
            // Desktop uses filesystem directly through application state
            Err(anyhow::anyhow!("Desktop platform deletes through filesystem"))
        }
    }
    
    fn get_saved_documents(&self) -> FileOperationResult<Vec<String>> {
        if cfg!(target_arch = "wasm32") {
            // Web platform doesn't have persistent storage for file listing
            Ok(vec![])
        } else if cfg!(feature = "mobile") {
            // Mobile platform lists files from persistent storage
            Ok(get_saved_files())
        } else {
            // Desktop doesn't need a saved documents list
            Ok(vec![])
        }
    }
    
    fn get_file_size(&self, filename: &str) -> usize {
        if cfg!(target_arch = "wasm32") {
            // Web platform doesn't track file sizes
            0
        } else if cfg!(feature = "mobile") {
            // Mobile platform gets size from persistent storage
            get_file_size_impl(filename)
        } else {
            // Desktop gets file size through filesystem
            0
        }
    }
    
    fn show_open_dialog(&self) -> Option<PathBuf> {
        if cfg!(target_arch = "wasm32") {
            // Web platform uses file input element, not native dialog
            None
        } else if cfg!(feature = "mobile") {
            // Mobile uses its own file list UI
            None
        } else {
            // Desktop uses native file dialogs
            show_open_dialog_impl()
        }
    }
    
    fn show_save_dialog(&self) -> Option<PathBuf> {
        if cfg!(target_arch = "wasm32") {
            // Web platform uses browser download, not native dialog
            None
        } else if cfg!(feature = "mobile") {
            // Mobile uses its own filename input UI
            None
        } else {
            // Desktop uses native file dialogs
            show_save_dialog_impl()
        }
    }
    
    fn share_document(&self, content: &str) {
        if cfg!(target_arch = "wasm32") {
            // Web platform could use Web Share API in the future
            println!("Web: Would share document ({} chars)", content.len());
        } else if cfg!(feature = "mobile") {
            // Mobile platform uses platform-specific sharing
            share_document_mobile(content);
        } else {
            // Desktop sharing
            println!("Desktop: Would share document ({} chars)", content.len());
        }
    }
}

/// Get the unified file operations implementation
pub fn get_file_operations() -> UnifiedFileOperations {
    UnifiedFileOperations
}

// Platform-specific implementation functions

fn download_file(content: &str, filename: &str) {
    // On web platforms, this will trigger a download
    // On non-web platforms, the web APIs will be no-ops
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    // Create a blob with the content
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(content));
    
    let blob = Blob::new_with_str_sequence(&array).unwrap();
    
    // Create a download link
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .unwrap()
        .dyn_into()
        .unwrap();
    
    anchor.set_href(&url);
    anchor.set_download(filename);
    let anchor_element: &web_sys::Element = anchor.as_ref();
    anchor_element.set_attribute("style", "display: none").unwrap();
    
    // Append, click, and remove
    document.body().unwrap().append_child(&anchor).unwrap();
    anchor.click();
    document.body().unwrap().remove_child(&anchor).unwrap();
    
    // Clean up the object URL
    Url::revoke_object_url(&url).unwrap();
}

pub fn save_document_to_storage(content: &str, filename: &str) -> FileOperationResult<()> {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    println!("Storage: Attempting to save {} to {:?}", filename, file_path);
    println!("Storage: Content length: {} bytes", content.len());
    
    fs::write(&file_path, content).map_err(|e| {
        anyhow::anyhow!("Failed to save {} to {:?}: {}", filename, file_path, e)
    })?;
    
    println!("Storage: Successfully saved {} to {:?}", filename, file_path);
    
    // Verify the file was actually written
    if let Ok(metadata) = fs::metadata(&file_path) {
        println!("Storage: File size on disk: {} bytes", metadata.len());
    }
    
    Ok(())
}

pub fn load_document_from_storage(filename: &str) -> Option<String> {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            println!("Storage: Loaded {} from {:?}", filename, file_path);
            Some(content)
        }
        Err(e) => {
            println!("Storage: Failed to load {}: {}", filename, e);
            None
        }
    }
}

pub fn delete_document_from_storage(filename: &str) -> bool {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    match fs::remove_file(&file_path) {
        Ok(_) => {
            println!("Storage: Deleted {} from {:?}", filename, file_path);
            true
        }
        Err(e) => {
            println!("Storage: Failed to delete {}: {}", filename, e);
            false
        }
    }
}

pub fn get_saved_files() -> Vec<String> {
    let storage_dir = get_storage_directory();
    println!("Storage: Looking for files in {:?}", storage_dir);
    
    // Read all .json files from the storage directory
    let mut files = Vec::new();
    
    match fs::read_dir(&storage_dir) {
        Ok(entries) => {
            println!("Storage: Successfully opened directory, reading entries...");
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            if filename_str.ends_with(".json") {
                                println!("Storage: Found file: {}", filename_str);
                                files.push(filename_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Storage: Failed to read directory {:?}: {}", storage_dir, e);
            // Return empty list if directory doesn't exist or can't be read
            return vec![];
        }
    }
    
    // Add sample files if directory is empty (first run)
    if files.is_empty() {
        println!("Storage: No files found, initializing sample files...");
        initialize_sample_files();
        // Re-read after initialization
        if let Ok(entries) = fs::read_dir(&storage_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            if filename_str.ends_with(".json") {
                                println!("Storage: Found file after init: {}", filename_str);
                                files.push(filename_str.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    files.sort(); // Sort alphabetically
    println!("Storage: Returning {} files: {:?}", files.len(), files);
    files
}

pub fn get_file_size_impl(filename: &str) -> usize {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    match fs::metadata(&file_path) {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => 0,
    }
}

pub fn get_storage_directory() -> PathBuf {
    if cfg!(target_os = "ios") {
        // On iOS, try to use the app's Documents directory
        let storage_dir = if let Some(home) = std::env::var_os("HOME") {
            let mut dir = PathBuf::from(home);
            dir.push("Documents");
            dir.push("mobile_documents");
            dir
        } else {
            let mut dir = std::env::temp_dir();
            dir.push("mobile_documents");
            dir
        };
        
        match fs::create_dir_all(&storage_dir) {
            Ok(_) => {
                println!("iOS: Created storage directory at {:?}", storage_dir);
                storage_dir
            }
            Err(e) => {
                println!("iOS: Failed to create storage directory {:?}: {}", storage_dir, e);
                let temp_dir = std::env::temp_dir();
                println!("iOS: Using temp directory: {:?}", temp_dir);
                temp_dir
            }
        }
    } else if cfg!(target_os = "android") {
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
    } else {
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

pub fn initialize_sample_files() {
    let sample_circle = r#"{
  "html": "<svg viewBox=\"0 0 70 70\" xmlns=\"http://www.w3.org/2000/svg\">\n<circle cx=\"35\" cy=\"35\" r=\"25\" fill=\"lightblue\" stroke=\"darkblue\" stroke-width=\"2\"/>\n<text x=\"35\" y=\"40\" text-anchor=\"middle\" font-size=\"8\">Circle Doc</text>\n</svg>"
}"#;
    
    let sample_square = r#"{
  "html": "<svg viewBox=\"0 0 70 70\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"15\" y=\"15\" width=\"40\" height=\"40\" fill=\"lightcoral\" stroke=\"darkred\" stroke-width=\"2\"/>\n<text x=\"35\" y=\"40\" text-anchor=\"middle\" font-size=\"8\">Square Doc</text>\n</svg>"
}"#;
    
    // Create sample files
    let _ = save_document_to_storage(sample_circle, "sample_circle.json");
    let _ = save_document_to_storage(sample_square, "sample_square.json");
    
    println!("Mobile: Initialized sample files for first run");
}

pub fn share_document_mobile(content: &str) {
    if cfg!(target_os = "android") {
        println!("Android: Opening share sheet");
    } else if cfg!(target_os = "ios") {
        println!("iOS: Opening activity view controller");
    } else {
        println!("Share: Would share document ({} chars)", content.len());
    }
}

fn show_open_dialog_impl() -> Option<PathBuf> {
    // Desktop file dialog functionality is handled in the desktop package
    // Web and mobile platforms use their own UI for file selection
    None
}

fn show_save_dialog_impl() -> Option<PathBuf> {
    // Desktop file dialog functionality is handled in the desktop package
    // Web and mobile platforms use their own UI for file saving
    None
}
