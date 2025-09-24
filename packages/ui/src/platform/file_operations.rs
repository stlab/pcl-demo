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

/// Web-specific file operations implementation
pub struct WebFileOperations;

impl FileOperations for WebFileOperations {
    fn save_document(&self, content: &str, filename: &str) -> FileOperationResult<()> {
        // Web platform uses browser download
        download_file(content, filename);
        Ok(())
    }
    
    fn load_document(&self, _filename: &str) -> FileOperationResult<String> {
        // Web platform loads through file input element
        // This is handled by the FileMenu component directly
        Err(anyhow::anyhow!("Web platform loads files through file input"))
    }
    
    fn delete_document(&self, _filename: &str) -> FileOperationResult<()> {
        // Web platform doesn't support direct file deletion
        Err(anyhow::anyhow!("Web platform doesn't support file deletion"))
    }
    
    fn get_saved_documents(&self) -> FileOperationResult<Vec<String>> {
        // Web platform doesn't have persistent storage for file listing
        Ok(vec![])
    }
    
    fn get_file_size(&self, _filename: &str) -> usize {
        // Web platform doesn't track file sizes
        0
    }
    
    fn show_open_dialog(&self) -> Option<PathBuf> {
        // Web platform uses file input element, not native dialog
        None
    }
    
    fn show_save_dialog(&self) -> Option<PathBuf> {
        // Web platform uses browser download, not native dialog
        None
    }
    
    fn share_document(&self, content: &str) {
        // Web platform could use Web Share API in the future
        println!("Web: Would share document ({} chars)", content.len());
    }
}

/// Mobile-specific file operations implementation
pub struct MobileFileOperations;

impl FileOperations for MobileFileOperations {
    fn save_document(&self, content: &str, filename: &str) -> FileOperationResult<()> {
        save_document_to_storage(content, filename)
    }
    
    fn load_document(&self, filename: &str) -> FileOperationResult<String> {
        load_document_from_storage(filename)
            .ok_or_else(|| anyhow::anyhow!("Failed to load document: {}", filename))
    }
    
    fn delete_document(&self, filename: &str) -> FileOperationResult<()> {
        if delete_document_from_storage(filename) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to delete document: {}", filename))
        }
    }
    
    fn get_saved_documents(&self) -> FileOperationResult<Vec<String>> {
        Ok(get_saved_files())
    }
    
    fn get_file_size(&self, filename: &str) -> usize {
        get_file_size_impl(filename)
    }
    
    fn show_open_dialog(&self) -> Option<PathBuf> {
        // Mobile uses its own file list UI
        None
    }
    
    fn show_save_dialog(&self) -> Option<PathBuf> {
        // Mobile uses its own filename input UI
        None
    }
    
    fn share_document(&self, content: &str) {
        share_document_mobile(content);
    }
}

/// Desktop-specific file operations implementation
pub struct DesktopFileOperations;

impl FileOperations for DesktopFileOperations {
    fn save_document(&self, _content: &str, _filename: &str) -> FileOperationResult<()> {
        // Desktop uses application state for saving
        Err(anyhow::anyhow!("Desktop platform saves through application state"))
    }
    
    fn load_document(&self, _filename: &str) -> FileOperationResult<String> {
        // Desktop uses application state for loading
        Err(anyhow::anyhow!("Desktop platform loads through application state"))
    }
    
    fn delete_document(&self, _filename: &str) -> FileOperationResult<()> {
        // Desktop uses filesystem directly through application state
        Err(anyhow::anyhow!("Desktop platform deletes through filesystem"))
    }
    
    fn get_saved_documents(&self) -> FileOperationResult<Vec<String>> {
        // Desktop doesn't need a saved documents list
        Ok(vec![])
    }
    
    fn get_file_size(&self, _filename: &str) -> usize {
        // Desktop gets file size through filesystem
        0
    }
    
    fn show_open_dialog(&self) -> Option<PathBuf> {
        show_open_dialog_impl()
    }
    
    fn show_save_dialog(&self) -> Option<PathBuf> {
        show_save_dialog_impl()
    }
    
    fn share_document(&self, content: &str) {
        println!("Desktop: Would share document ({} chars)", content.len());
    }
}

/// Platform-specific file operations implementation
pub enum PlatformFileOperations {
    Web(WebFileOperations),
    Mobile(MobileFileOperations),
    Desktop(DesktopFileOperations),
}

impl FileOperations for PlatformFileOperations {
    fn save_document(&self, content: &str, filename: &str) -> FileOperationResult<()> {
        match self {
            PlatformFileOperations::Web(ops) => ops.save_document(content, filename),
            PlatformFileOperations::Mobile(ops) => ops.save_document(content, filename),
            PlatformFileOperations::Desktop(ops) => ops.save_document(content, filename),
        }
    }
    
    fn load_document(&self, filename: &str) -> FileOperationResult<String> {
        match self {
            PlatformFileOperations::Web(ops) => ops.load_document(filename),
            PlatformFileOperations::Mobile(ops) => ops.load_document(filename),
            PlatformFileOperations::Desktop(ops) => ops.load_document(filename),
        }
    }
    
    fn delete_document(&self, filename: &str) -> FileOperationResult<()> {
        match self {
            PlatformFileOperations::Web(ops) => ops.delete_document(filename),
            PlatformFileOperations::Mobile(ops) => ops.delete_document(filename),
            PlatformFileOperations::Desktop(ops) => ops.delete_document(filename),
        }
    }
    
    fn get_saved_documents(&self) -> FileOperationResult<Vec<String>> {
        match self {
            PlatformFileOperations::Web(ops) => ops.get_saved_documents(),
            PlatformFileOperations::Mobile(ops) => ops.get_saved_documents(),
            PlatformFileOperations::Desktop(ops) => ops.get_saved_documents(),
        }
    }
    
    fn get_file_size(&self, filename: &str) -> usize {
        match self {
            PlatformFileOperations::Web(ops) => ops.get_file_size(filename),
            PlatformFileOperations::Mobile(ops) => ops.get_file_size(filename),
            PlatformFileOperations::Desktop(ops) => ops.get_file_size(filename),
        }
    }
    
    fn show_open_dialog(&self) -> Option<PathBuf> {
        match self {
            PlatformFileOperations::Web(ops) => ops.show_open_dialog(),
            PlatformFileOperations::Mobile(ops) => ops.show_open_dialog(),
            PlatformFileOperations::Desktop(ops) => ops.show_open_dialog(),
        }
    }
    
    fn show_save_dialog(&self) -> Option<PathBuf> {
        match self {
            PlatformFileOperations::Web(ops) => ops.show_save_dialog(),
            PlatformFileOperations::Mobile(ops) => ops.show_save_dialog(),
            PlatformFileOperations::Desktop(ops) => ops.show_save_dialog(),
        }
    }
    
    fn share_document(&self, content: &str) {
        match self {
            PlatformFileOperations::Web(ops) => ops.share_document(content),
            PlatformFileOperations::Mobile(ops) => ops.share_document(content),
            PlatformFileOperations::Desktop(ops) => ops.share_document(content),
        }
    }
}

/// Get the appropriate file operations implementation for the current platform
pub fn get_file_operations() -> PlatformFileOperations {
    if cfg!(target_arch = "wasm32") {
        PlatformFileOperations::Web(WebFileOperations)
    } else if cfg!(feature = "mobile") {
        PlatformFileOperations::Mobile(MobileFileOperations)
    } else {
        PlatformFileOperations::Desktop(DesktopFileOperations)
    }
}

// Platform-specific implementation functions

#[cfg(target_arch = "wasm32")]
fn download_file(content: &str, filename: &str) {
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

#[cfg(not(target_arch = "wasm32"))]
fn download_file(_content: &str, _filename: &str) {
    // Not applicable for non-web platforms
}

#[cfg(feature = "mobile")]
fn save_document_to_storage(content: &str, filename: &str) -> FileOperationResult<()> {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    println!("Mobile: Attempting to save {} to {:?}", filename, file_path);
    println!("Mobile: Content length: {} bytes", content.len());
    
    fs::write(&file_path, content).map_err(|e| {
        anyhow::anyhow!("Failed to save {} to {:?}: {}", filename, file_path, e)
    })?;
    
    println!("Mobile: Successfully saved {} to {:?}", filename, file_path);
    
    // Verify the file was actually written
    if let Ok(metadata) = fs::metadata(&file_path) {
        println!("Mobile: File size on disk: {} bytes", metadata.len());
    }
    
    Ok(())
}

#[cfg(not(feature = "mobile"))]
fn save_document_to_storage(_content: &str, _filename: &str) -> FileOperationResult<()> {
    Err(anyhow::anyhow!("Mobile storage not available on this platform"))
}

#[cfg(feature = "mobile")]
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

#[cfg(not(feature = "mobile"))]
fn load_document_from_storage(_filename: &str) -> Option<String> {
    None
}

#[cfg(feature = "mobile")]
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

#[cfg(not(feature = "mobile"))]
fn delete_document_from_storage(_filename: &str) -> bool {
    false
}

#[cfg(feature = "mobile")]
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
        if let Ok(entries) = fs::read_dir(&storage_dir) {
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
    }
    
    files.sort(); // Sort alphabetically
    println!("Mobile: Returning {} files: {:?}", files.len(), files);
    files
}

#[cfg(not(feature = "mobile"))]
fn get_saved_files() -> Vec<String> {
    vec![]
}

#[cfg(feature = "mobile")]
fn get_file_size_impl(filename: &str) -> usize {
    let storage_dir = get_storage_directory();
    let file_path = storage_dir.join(filename);
    
    match fs::metadata(&file_path) {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => 0,
    }
}

#[cfg(not(feature = "mobile"))]
fn get_file_size_impl(_filename: &str) -> usize {
    0
}

#[cfg(feature = "mobile")]
fn get_storage_directory() -> PathBuf {
    
    #[cfg(target_os = "ios")]
    {
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

#[cfg(feature = "mobile")]
fn initialize_sample_files() {
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

#[cfg(feature = "mobile")]
fn share_document_mobile(content: &str) {
    #[cfg(target_os = "android")]
    {
        println!("Android: Opening share sheet");
    }
    
    #[cfg(target_os = "ios")]
    {
        println!("iOS: Opening activity view controller");
    }
    
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        println!("Mobile: Would share document ({} chars)", content.len());
    }
}

#[cfg(not(feature = "mobile"))]
fn share_document_mobile(content: &str) {
    println!("Share not available on non-mobile platforms ({} chars)", content.len());
}

#[cfg(not(any(target_arch = "wasm32", feature = "mobile")))]
fn show_open_dialog_impl() -> Option<PathBuf> {
    // Desktop file dialog functionality is handled in the desktop package
    None
}

#[cfg(any(target_arch = "wasm32", feature = "mobile"))]
fn show_open_dialog_impl() -> Option<PathBuf> {
    None
}

#[cfg(not(any(target_arch = "wasm32", feature = "mobile")))]
fn show_save_dialog_impl() -> Option<PathBuf> {
    // Desktop file dialog functionality is handled in the desktop package
    None
}

#[cfg(any(target_arch = "wasm32", feature = "mobile"))]
fn show_save_dialog_impl() -> Option<PathBuf> {
    None
}
