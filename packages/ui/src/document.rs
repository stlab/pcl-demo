use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

/// In-memory representation of a pcl-demo document.
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    /// The content in HTML form.
    html: String,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    /// Returns a new empty document.
    pub fn new() -> Self {
        Self {
            html: "<svg viewBox=\"0 0 70 70\" xmlns=\"http://www.w3.org/2000/svg\">\n<text x=\"35\" y=\"35\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-size=\"8\">New Document</text>\n</svg>".to_string()
        }
    }

    /// Returns the document at `p`.
    pub fn new_from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        let p: &Path = p.as_ref();

        let f = File::open(p).context(format!("Failed to open: {p:?}"))?;

        serde_json::from_reader(f).context(format!("Invalid json: {p:?}"))
    }

    /// Saves the document as `p`.
    pub fn save_to_file<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        let p: &Path = p.as_ref();

        let f = File::create(p).context(format!("Failed to create: {p:?}"))?;

        serde_json::to_writer_pretty(f, self).context(format!("Failed to write: {p:?}"))
    }
}
