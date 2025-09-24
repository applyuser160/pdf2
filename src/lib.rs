use pyo3::prelude::*;
use std::path::Path;

pub mod pdf2;

use pdf2::{Document, Image, Page, TextBlock};

#[pyfunction]
fn parse(path_str: String) -> PyResult<Document> {
    let path = Path::new(&path_str);
    // Here, we map the custom Rust error to a PyErr.
    pdf2::parser::parse_pdf(path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse PDF: {}", e))
    })
}

#[pyfunction]
fn generate(doc: &Document, path_str: String) -> PyResult<()> {
    let path = Path::new(&path_str);
    pdf2::generator::generate_pdf(doc, path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(format!("{}", e)))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_class::<Document>()?;
    m.add_class::<Page>()?;
    m.add_class::<TextBlock>()?;
    m.add_class::<Image>()?;
    Ok(())
}
