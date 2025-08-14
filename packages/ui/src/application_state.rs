use crate::document::*;
use crate::default_document::*;

pub struct ApplicationState {
    pub the_only_document: Document
}

impl ApplicationState {

    pub fn initial() -> ApplicationState {
        ApplicationState {
            the_only_document: default_document()
        }
    }

}
