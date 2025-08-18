use std::{fs::File, io::Read, io::Result};
use std::path::Path;

/// In-memory representation of a pcl-demo document.
pub struct Document {
    html: String
}

impl Document {

    /// Returns the in-memory representation of the pcl-demo document at `p`.
    pub fn new_from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        let mut r = Document { html: String::new() };
        File::open(p)?.read_to_string(&mut r.html).map(|_| r)
    }

    /// Returns the HTML to render `self`.
    pub fn to_html(self: &Self) -> String { self.html.clone() }

}
