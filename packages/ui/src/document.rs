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
        let mut f = File::create(p.as_ref().with_file_name("image.json"))?;
        let r = Document {
            html: std::fs::read_to_string(p)?
        };
        let j = serde_json::to_string_pretty(&r)?;
        let _ = f.write(j.as_bytes())?;

        Ok(r)
    }

    /// Returns the HTML to render `self`.
    pub fn to_html(self: &Self) -> String { self.html.clone() }

}
