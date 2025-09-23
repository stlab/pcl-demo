use std::{fs::File, path::Path};
use anyhow::Context;
use serde::{Deserialize, Serialize};

/// In-memory representation of a pcl-demo document.
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    html: String
}

impl Document {

    /// Returns a new empty document.
    pub fn new() -> Self {
        Self {
            html: "<svg viewBox=\"0 0 70 70\" xmlns=\"http://www.w3.org/2000/svg\">\n<text x=\"35\" y=\"35\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-size=\"8\">New Document</text>\n</svg>".to_string()
        }
    }

    /// Returns the in-memory representation of the document at `p`.
    pub fn new_from_file<P: AsRef<Path>>(p: P) -> anyhow::Result<Self> {
        let p: &Path = p.as_ref();

        let f = File::open(p)
            .context(format!("Failed to open: {:?}", p))?;

        serde_json::from_reader(f)
            .context(format!("Invalid json: {:?}", p))
    }

    /// Saves the document as `p`.
    pub fn save_to_file<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<()> {
        let p: &Path = p.as_ref();
        
        let f = File::create(p)
            .context(format!("Failed to create: {:?}", p))?;

        serde_json::to_writer_pretty(f, self)
            .context(format!("Failed to write: {:?}", p))
    }

    /// Returns the HTML to render `self`.
    pub fn to_html(self: &Self) -> String { self.html.clone() }

}
