//! Platform-agnostic file operations interface
//!
//! This module provides a unified interface for file operations across different platforms,
//! factoring out cfg-dependent code to improve rust-analyzer support.

use std::path::{Path, PathBuf};
use std::fs;

// Web API imports (available on all platforms for development ease)
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{window, Blob, Url, HtmlAnchorElement, Element};

// Other imports
use anyhow::{Result, Context};
use js_sys::Array;

/// Save a document using the appropriate platform method
pub fn save_document(content: &str, filename: &str) -> Result<()> {
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
pub fn load_document(filename: &str) -> Result<String> {
    if cfg!(feature = "mobile") {
        load_document_from_storage(filename)
            .with_context(|| format!("Failed to load document '{}'", filename))
    } else {
        unreachable!("load_document should not be called on this platform")
    }
}

/// Delete a document using the appropriate platform method
pub fn delete_document(filename: &str) -> Result<()> {
    if cfg!(feature = "mobile") {
        delete_document_from_storage(filename)
            .with_context(|| format!("Failed to delete document '{}'", filename))
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
fn collect_json_files_from_dir(storage_dir: &Path) -> Result<Vec<String>> {
    let entries = fs::read_dir(storage_dir)
        .with_context(|| format!("Failed to read directory {:?}", storage_dir))?;
    
    Ok(entries
        .flatten() // Convert Result<DirEntry, Error> to just DirEntry, skipping errors
        .filter_map(|entry| { // Extract filename and filter for .json files
            entry.path()
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| name.ends_with(".json"))
                .map(|name| name.to_string())
        })
        .collect())
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

pub fn save_document_to_storage(content: &str, filename: &str) -> Result<()> {
    let file_path = get_file_path(filename);
    
    fs::write(&file_path, content)
        .with_context(|| format!("Failed to save '{}' to {:?}", filename, file_path))?;
    
    Ok(())
}

pub fn load_document_from_storage(filename: &str) -> Result<String> {
    let file_path = get_file_path(filename);
    fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read file '{}'", filename))
}

pub fn delete_document_from_storage(filename: &str) -> Result<()> {
    let file_path = get_file_path(filename);
    fs::remove_file(&file_path)
        .with_context(|| format!("Failed to delete file '{}'", filename))
}

pub fn get_saved_files() -> Result<Vec<String>> {
    let storage_dir = get_storage_directory();
    let mut files = collect_json_files_from_dir(&storage_dir)?;
    
    if files.is_empty() {
        initialize_sample_files();
        files = collect_json_files_from_dir(&storage_dir)?;
    }
    
    files.sort();
    Ok(files)
}

pub fn get_file_size_impl(filename: &str) -> Result<usize> {
    let file_path = get_file_path(filename);
    fs::metadata(&file_path)
        .map(|metadata| metadata.len() as usize)
        .with_context(|| format!("Failed to get file size for '{}'", filename))
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
        fs::create_dir_all(&storage_dir)
            .map(|_| storage_dir)
            .unwrap_or_else(|_| std::env::temp_dir())
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
        fs::create_dir_all(&storage_dir)
            .map(|_| storage_dir)
            .unwrap_or_else(|_| std::env::temp_dir())
    } else {
        let mut storage_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        storage_dir.push("mobile_documents");
        let fallback = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        fs::create_dir_all(&storage_dir)
            .map(|_| storage_dir)
            .unwrap_or(fallback)
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

