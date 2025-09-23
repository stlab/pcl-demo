use std::io::Result;
use std::path::Path;

/// In-memory representation of a pcl-demo document.
pub struct Document {
    html: String
}

impl Document {

    /// Returns the in-memory representation of the pcl-demo document at `p`.
    pub fn new_from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        Ok(Document {
          html: std::fs::read_to_string(p)?
        })
    }

    /// Returns the HTML to render `self`.
    pub fn to_html(self: &Self) -> String { self.html.clone() }

}
