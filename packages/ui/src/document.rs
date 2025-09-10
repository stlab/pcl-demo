use std::*;
use std::fs::*;
use anyhow::*;
use std::path::*;
use serde::*;
use serde_json;

/// In-memory representation of a pcl-demo document.
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    html: String
}

impl Document {

    /// Returns the in-memory representation of the pcl-demo document at `p`.
    pub fn new_from_file<P: AsRef<Path>>(p: P) -> anyhow::Result<Self> {
        let p: &Path = p.as_ref();

        let f = File::open(p)
            .context(format!("Failed to open: {:?}", p))?;

        serde_json::from_reader(f)
            .context(format!("Invalid json: {:?}", p))
    }

    /// Returns the HTML to render `self`.
    pub fn to_html(self: &Self) -> String { self.html.clone() }

}
