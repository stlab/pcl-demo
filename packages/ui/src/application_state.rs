use crate::document::*;

/// The state of the entire application.
pub struct ApplicationState {

    /// The one document that every application has open.
    pub the_only_document: Document

}

impl ApplicationState {

    ///
    pub fn new() -> ApplicationState {
        ApplicationState {
            the_only_document: Document::new_from_file("image.svg")
                .expect("Failed to read document.")
        }
    }

}
