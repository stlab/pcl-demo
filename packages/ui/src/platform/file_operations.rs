//! Platform-agnostic file operations interface
//!
//! This module provides a unified interface for file operations across different platforms,
//! factoring out cfg-dependent code to improve rust-analyzer support.

use std::path::{Path, PathBuf};
use std::fs;

// Web API imports (available on all platforms for development ease)
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Blob, Url, HtmlAnchorElement, Element};

// Other imports
use anyhow::{Result, anyhow};
use js_sys::Array;

/// Result type for file operations
pub type FileOperationResult<T> = Result<T>;

/// Save a document using the appropriate platform method
pub fn save_document(content: &str, filename: &str) -> FileOperationResult<()> {
    if cfg!(target_arch = "wasm32") {
        download_file(content, filename);
        Ok(())
    } else if cfg!(feature = "mobile") {
        save_document_to_storage(content, filename)
    } else {
        unreachable!("save_document should not be called on this platform")
    }
}

/// Load a document using the appropriate platform method
pub fn load_document(filename: &str) -> FileOperationResult<String> {
    if cfg!(feature = "mobile") {
        load_document_from_storage(filename)
            .ok_or_else(|| anyhow!("Failed to load document: {}", filename))
    } else {
        unreachable!("load_document should not be called on this platform")
    }
}

/// Delete a document using the appropriate platform method
pub fn delete_document(filename: &str) -> FileOperationResult<()> {
    if cfg!(feature = "mobile") {
        if delete_document_from_storage(filename) {
            Ok(())
        } else {
            Err(anyhow!("Failed to delete document: {}", filename))
        }
    } else {
        unreachable!("delete_document should not be called on this platform")
    }
}





// Helper functions for common operations

/// Get the full path for a file in the storage directory
fn get_file_path(filename: &str) -> PathBuf {
    let storage_dir = get_storage_directory();
    storage_dir.join(filename)
}

/// Helper to collect JSON files from a directory
fn collect_json_files_from_dir(storage_dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(storage_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        if filename_str.ends_with(".json") {
                            files.push(filename_str.to_string());
                        }
                    }
                }
            }
        }
    }
    
    files
}

/// Helper to create a storage directory with fallback
fn create_storage_dir_with_fallback(preferred_dir: &Path, fallback_dir: PathBuf) -> PathBuf {
    match fs::create_dir_all(preferred_dir) {
        Ok(_) => preferred_dir.to_path_buf(),
        Err(_) => fallback_dir,
    }
}

// Platform-specific implementation functions

fn download_file(content: &str, filename: &str) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    let array = Array::new();
    array.push(&JsValue::from_str(content));

    // Synthesize a link to the content and (programmatically) click
    // it.
    let blob = Blob::new_with_str_sequence(&array).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .unwrap()
        .dyn_into()
        .unwrap();
    
    anchor.set_href(&url);
    anchor.set_download(filename);
    let anchor_element: &Element = anchor.as_ref();
    anchor_element.set_attribute("style", "display: none").unwrap();
    document.body().unwrap().append_child(&anchor).unwrap();
    anchor.click();
    document.body().unwrap().remove_child(&anchor).unwrap();
    Url::revoke_object_url(&url).unwrap();
}

pub fn save_document_to_storage(content: &str, filename: &str) -> FileOperationResult<()> {
    let file_path = get_file_path(filename);
    
    fs::write(&file_path, content).map_err(|e| {
        anyhow!("Failed to save {} to {:?}: {}", filename, file_path, e)
    })?;
    
    Ok(())
}

pub fn load_document_from_storage(filename: &str) -> Option<String> {
    let file_path = get_file_path(filename);
    
    match fs::read_to_string(&file_path) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

pub fn delete_document_from_storage(filename: &str) -> bool {
    let file_path = get_file_path(filename);
    
    match fs::remove_file(&file_path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_saved_files() -> Vec<String> {
    let storage_dir = get_storage_directory();
    let mut files = collect_json_files_from_dir(&storage_dir);
    
    if files.is_empty() {
        initialize_sample_files();
        files = collect_json_files_from_dir(&storage_dir);
    }
    
    files.sort();
    files
}

pub fn get_file_size_impl(filename: &str) -> usize {
    let file_path = get_file_path(filename);
    
    match fs::metadata(&file_path) {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => 0,
    }
}

pub fn get_storage_directory() -> PathBuf {
    if cfg!(target_os = "ios") {
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
        create_storage_dir_with_fallback(&storage_dir, std::env::temp_dir())
    } else if cfg!(target_os = "android") {
        let storage_dir = if let Ok(current) = std::env::current_dir() {
            let mut dir = current;
            dir.push("mobile_documents");
            dir
        } else {
            let mut dir = std::env::temp_dir();
            dir.push("mobile_documents");
            dir
        };
        create_storage_dir_with_fallback(&storage_dir, std::env::temp_dir())
    } else {
        let mut storage_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        storage_dir.push("mobile_documents");
        let fallback = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        create_storage_dir_with_fallback(&storage_dir, fallback)
    }
}

pub fn initialize_sample_files() {
    let sample_circle = r#"{
  "html": "<svg viewBox=\"0 0 70 70\" xmlns=\"http://www.w3.org/2000/svg\">\n<circle cx=\"35\" cy=\"35\" r=\"25\" fill=\"lightblue\" stroke=\"darkblue\" stroke-width=\"2\"/>\n<text x=\"35\" y=\"40\" text-anchor=\"middle\" font-size=\"8\">Circle Doc</text>\n</svg>"
}"#;
    
    let sample_square = r#"{
  "html": "<svg viewBox=\"0 0 70 70\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"15\" y=\"15\" width=\"40\" height=\"40\" fill=\"lightcoral\" stroke=\"darkred\" stroke-width=\"2\"/>\n<text x=\"35\" y=\"40\" text-anchor=\"middle\" font-size=\"8\">Square Doc</text>\n</svg>"
}"#;
    
    let _ = save_document_to_storage(sample_circle, "sample_circle.json");
    let _ = save_document_to_storage(sample_square, "sample_square.json");
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

