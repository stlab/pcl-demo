use crate::document::*;

/// The state of the entire application.
pub struct ApplicationState {

    /// The one document that every application has open.
    pub the_only_document: Document

}

impl ApplicationState {

    /// Returns the state of a newly-launched application
    pub fn new() -> Self {
        // Read the application's one file from the standard location.
        Self {
            the_only_document: Document::new_from_file("image.svg")
                .expect("Failed to read document.")
        }
    }

}
