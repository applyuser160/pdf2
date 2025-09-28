use std::path::{Path, PathBuf};

use crate::{
    generator::{generate_pdf, GenerateError},
    parser::{parse_pdf, ParseError},
    structure::Document,
};

pub struct Pdf {
    path: PathBuf,
}

impl Pdf {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn read(&self) -> Result<Document, ParseError> {
        parse_pdf(Path::new(&self.path))
    }

    pub fn write(&self, document: &Document) -> Result<(), GenerateError> {
        generate_pdf(document, Path::new(&self.path))
    }
}
