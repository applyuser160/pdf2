#[path = "pdf2/generator.rs"]
pub mod generator;
#[path = "pdf2/metadata.rs"]
pub mod metadata;
#[path = "pdf2/parser.rs"]
pub mod parser;
#[path = "pdf2/structure.rs"]
pub mod structure;

#[cfg(test)]
#[path = "pdf2/test_generator.rs"]
pub mod test_generator;
#[cfg(test)]
#[path = "pdf2/test_parser.rs"]
pub mod test_parser;
#[cfg(test)]
#[path = "pdf2/test_structure.rs"]
pub mod test_structure;
#[cfg(test)]
#[path = "pdf2/test_metadata.rs"]
pub mod test_metadata;

use pyo3::prelude::*;
use std::path::Path;

use structure::{Document, Image, Page, TextBlock};

#[pyfunction]
fn parse(path_str: String) -> PyResult<Document> {
    let path = Path::new(&path_str);
    // Here, we map the custom Rust error to a PyErr.
    parser::parse_pdf(path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse PDF: {}", e))
    })
}

#[pyfunction]
fn generate(doc: &Document, path_str: String) -> PyResult<()> {
    let path = Path::new(&path_str);
    generator::generate_pdf(doc, path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(format!("{}", e)))
}

#[pyfunction]
fn get_metadata(path_str: String) -> PyResult<metadata::Metadata> {
    let path = Path::new(&path_str);
    metadata::get_metadata(path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to get metadata: {}", e))
    })
}

#[pyfunction]
fn set_metadata(path_str: String, meta: &metadata::Metadata) -> PyResult<()> {
    let path = Path::new(&path_str);
    metadata::set_metadata(path, meta).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to set metadata: {}", e))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pdf2(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_function(wrap_pyfunction!(get_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(set_metadata, m)?)?;
    m.add_class::<Document>()?;
    m.add_class::<Page>()?;
    m.add_class::<TextBlock>()?;
    m.add_class::<Image>()?;
    m.add_class::<metadata::Metadata>()?;
    Ok(())
}
