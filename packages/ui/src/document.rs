use std::{fs::File, io::Read, io::Result};
use std::path::Path;

pub type Document = String;

pub trait DocumentMethods where Self: Sized {
    fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self>;
}

impl DocumentMethods for Document {
    fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut r = Self::new();
        File::open(path)?.read_to_string(&mut r).map(|_| r)
    }
}
