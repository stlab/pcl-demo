use crate::document::*;

pub struct ApplicationState {
    pub the_only_document: Document
}

impl ApplicationState {

    pub fn initial() -> ApplicationState {
        ApplicationState {
            the_only_document: Document::new_from_file("image.svg")
                .expect("Failed to read document.")
        }
    }

}
