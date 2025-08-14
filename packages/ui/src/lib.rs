//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

mod application_state;
pub use application_state::*;

mod document;
pub use document::*;

// Private modules
mod default_document;
