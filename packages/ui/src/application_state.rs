use crate::document::*;
use std::path::PathBuf;

/// The state of the entire application.
pub struct ApplicationState {

    /// The one document that every application has open.
    pub the_only_document: Document,

    /// Where the document will be saved (None for new unsaved documents).
    pub current_file_path: Option<PathBuf>,

}

impl ApplicationState {

    /// Returns the state of a newly-launched application
    pub fn new() -> Self {
        // Read the application's one file from the standard location.
        Self {
            the_only_document: Document::new_from_file("image.json")
                .expect("Failed to read document."),
            current_file_path: Some(PathBuf::from("image.json")),
        }
    }

    /// Creates a new document
    pub fn new_document(&mut self) {
        self.the_only_document = Document::new();
        self.current_file_path = None;
    }

    /// Loads a document from the specified path
    pub fn load_document(&mut self, path: PathBuf) -> anyhow::Result<()> {
        self.the_only_document = Document::new_from_file(&path)?;
        self.current_file_path = Some(path);
        Ok(())
    }

    /// Saves the current document to its current path
    pub fn save_document(&self) -> anyhow::Result<()> {
        if let Some(path) = &self.current_file_path {
            self.the_only_document.save_to_file(path)
        } else {
            Err(anyhow::anyhow!("No file path set - use Save As instead"))
        }
    }

    /// Saves the current document to a new path
    pub fn save_document_as(&mut self, path: PathBuf) -> anyhow::Result<()> {
        self.the_only_document.save_to_file(&path)?;
        self.current_file_path = Some(path);
        Ok(())
    }

}
