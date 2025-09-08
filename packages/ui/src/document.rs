use std::fs::*;
use std::io::*;
use std::path::*;
use serde::{Deserialize, Serialize};
use serde_json::*;

/// In-memory representation of a pcl-demo document.
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    html: String
}

impl Document {

    /// Returns the in-memory representation of the pcl-demo document at `p`.
    pub fn new_from_file<P: AsRef<Path>>(p: P) -> std::io::Result<Self> {
        let r = serde_json::from_reader(File::open(p)?)?;
        Ok(r)
    }

    /// Returns the HTML to render `self`.
    pub fn to_html(self: &Self) -> String { self.html.clone() }

}
