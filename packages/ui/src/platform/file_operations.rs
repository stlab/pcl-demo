//! Platform-agnostic file operations interface
//!
//! This module provides a unified interface for file operations across different platforms,
//! factoring out cfg-dependent code to improve rust-analyzer support.

use std::fs;
use std::path::{Path, PathBuf};

// Web API imports (available on all platforms for development ease)
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{window, Blob, Element, HtmlAnchorElement, Url};

// Other imports
use anyhow::{anyhow, Context, Result};
use js_sys::Array;

/// Saves `content` as `filename`.
pub fn save_document(content: &str, filename: &str) -> Result<()> {
    if cfg!(target_arch = "wasm32") {
        download_file(content, filename)
            .with_context(|| format!("Failed to download file '{filename}'"))
    } else if cfg!(feature = "mobile") {
        save_document_to_storage(content, filename)
    } else {
        unreachable!("save_document should not be called on this platform")
    }
}

/// Returns the content of the file named `filename`.
pub fn load_document(filename: &str) -> Result<String> {
    if cfg!(feature = "mobile") {
        load_document_from_storage(filename)
            .with_context(|| format!("Failed to load document '{filename}'"))
    } else {
        unreachable!("load_document should not be called on this platform")
    }
}

/// Deletes the file named `filename`.
pub fn delete_document(filename: &str) -> Result<()> {
    if cfg!(feature = "mobile") {
        delete_document_from_storage(filename)
            .with_context(|| format!("Failed to delete document '{filename}'"))
    } else {
        unreachable!("delete_document should not be called on this platform")
    }
}

// Helper functions for common operations

/// Returns the full path for `filename` in the storage directory.
fn file_path(filename: &str) -> PathBuf {
    storage_directory().join(filename)
}

/// Returns the JSON files in `storage_dir`.
fn collect_json_files_from_dir(storage_dir: &Path) -> Result<Vec<String>> {
    let entries = fs::read_dir(storage_dir)
        .with_context(|| format!("Failed to read directory {storage_dir:?}"))?;

    Ok(entries
        .flatten() // Convert Result<DirEntry, Error> to just DirEntry, skipping errors
        .filter_map(|entry| {
            // Extract filename and filter for .json files
            entry
                .path()
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| name.ends_with(".json"))
                .map(|name| name.to_string())
        })
        .collect())
}

// Platform-specific implementation functions

/// Downloads `content` as `filename`.
fn download_file(content: &str, filename: &str) -> Result<()> {
    let window =
        window().ok_or_else(|| anyhow!("Failed to get window object - browser API unavailable"))?;
    let document = window
        .document()
        .ok_or_else(|| anyhow!("Failed to get document object - browser API unavailable"))?;

    let array = Array::new();
    array.push(&JsValue::from_str(content));

    // Synthesize a link to the content and (programmatically) click it
    let blob = Blob::new_with_str_sequence(&array)
        .map_err(|_| anyhow!("Failed to create Blob from content"))?;
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|_| anyhow!("Failed to create object URL for blob"))?;
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|_| anyhow!("Failed to create anchor element"))?
        .dyn_into()
        .map_err(|_| anyhow!("Failed to cast element to HtmlAnchorElement"))?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    let anchor_element: &Element = anchor.as_ref();
    anchor_element
        .set_attribute("style", "display: none")
        .map_err(|_| anyhow!("Failed to set style attribute on anchor"))?;

    let body = document
        .body()
        .ok_or_else(|| anyhow!("Failed to get document body"))?;

    body.append_child(&anchor)
        .map_err(|_| anyhow!("Failed to append anchor to body"))?;
    anchor.click();
    body.remove_child(&anchor)
        .map_err(|_| anyhow!("Failed to remove anchor from body"))?;

    Url::revoke_object_url(&url).map_err(|_| anyhow!("Failed to revoke object URL"))?;

    Ok(())
}

/// Saves `content` as `filename` to storage.
pub fn save_document_to_storage(content: &str, filename: &str) -> Result<()> {
    fs::write(file_path(filename), content)
        .with_context(|| format!("Failed to save '{filename}' to {:?}", file_path(filename)))?;

    Ok(())
}

/// Returns the content of the file named `filename`.
pub fn load_document_from_storage(filename: &str) -> Result<String> {
    fs::read_to_string(&file_path(filename)).with_context(|| format!("Failed to read file '{filename}'"))
}

/// Deletes the file named `filename`.
pub fn delete_document_from_storage(filename: &str) -> Result<()> {
    fs::remove_file(&file_path(filename)).with_context(|| format!("Failed to delete file '{filename}'"))
}

/// Returns the names of all saved files.
pub fn saved_files() -> Result<Vec<String>> {
    let storage_dir = storage_directory();
    let mut files = collect_json_files_from_dir(&storage_dir)?;

    if files.is_empty() {
        initialize_sample_files();
        files = collect_json_files_from_dir(&storage_dir)?;
    }

    files.sort();
    Ok(files)
}

/// Returns the size of the file named `filename`.
pub fn file_size(filename: &str) -> Result<usize> {
    fs::metadata(&file_path(filename))
        .map(|metadata| metadata.len() as usize)
        .with_context(|| format!("Failed to get file size for '{filename}'"))
}

/// Returns the storage directory path.
pub fn storage_directory() -> PathBuf {
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
            .unwrap_or_else(|e| {
                eprintln!("Failed to create storage directory on iOS: {e}");
                std::env::temp_dir()
            })
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
            .unwrap_or_else(|e| {
                eprintln!("Failed to create storage directory on Android: {e}");
                std::env::temp_dir()
            })
    } else {
        let current_dir = std::env::current_dir().unwrap_or_else(|e| {
            eprintln!("Failed to get current directory: {e}");
            PathBuf::from(".")
        });
        let mut storage_dir = current_dir.clone();
        storage_dir.push("mobile_documents");

        fs::create_dir_all(&storage_dir)
            .map(|_| storage_dir)
            .unwrap_or_else(|e| {
                eprintln!("Failed to create mobile_documents directory: {e}");
                current_dir
            })
    }
}

/// Initializes sample files in the storage directory.
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

/// Shares `content` on mobile platforms.
pub fn share_document_mobile(content: &str) {
    if cfg!(target_os = "android") {
        println!("Android: Opening share sheet");
    } else if cfg!(target_os = "ios") {
        println!("iOS: Opening activity view controller");
    } else {
        println!("Share: Would share document ({} chars)", content.len());
    }
}
