use lopdf::encryption::crypt_filters::{Aes128CryptFilter, CryptFilter};
use lopdf::{Document, EncryptionState, EncryptionVersion, Permissions};
use pyo3::prelude::*;
use std::collections::BTreeMap;
use std::sync::Arc;

#[pyfunction]
pub fn encrypt_pdf(
    input_path: String,
    output_path: String,
    user_password: String,
    owner_password: Option<String>,
) -> PyResult<()> {
    let mut doc = Document::load(&input_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to load PDF: {}", e))
    })?;

    let owner_pwd = owner_password.as_deref().unwrap_or(&user_password);

    let crypt_filter: Arc<dyn CryptFilter> = Arc::new(Aes128CryptFilter);
    let encryption_version = EncryptionVersion::V4 {
        document: &doc,
        encrypt_metadata: true,
        crypt_filters: BTreeMap::from([(b"StdCF".to_vec(), crypt_filter)]),
        stream_filter: b"StdCF".to_vec(),
        string_filter: b"StdCF".to_vec(),
        owner_password: owner_pwd,
        user_password: &user_password,
        permissions: Permissions::all(),
    };

    let state = EncryptionState::try_from(encryption_version).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to create encryption state: {}",
            e
        ))
    })?;
    doc.encrypt(&state).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to encrypt PDF: {}", e))
    })?;

    doc.save(&output_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to save encrypted PDF: {}",
            e
        ))
    })?;

    Ok(())
}
