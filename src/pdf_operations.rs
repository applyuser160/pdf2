use lopdf::{dictionary, Document as LoDocument, Object, ObjectId};
use pyo3::prelude::*;
use std::collections::BTreeMap;

fn deep_copy_object(
    doc: &LoDocument,
    object_id: ObjectId,
    new_doc: &mut LoDocument,
    copied_objects: &mut BTreeMap<ObjectId, ObjectId>,
) -> Result<ObjectId, lopdf::Error> {
    if let Some(new_id) = copied_objects.get(&object_id) {
        return Ok(*new_id);
    }

    let obj = doc.get_object(object_id)?.clone();
    let new_id = new_doc.add_object(obj.clone());
    copied_objects.insert(object_id, new_id);

    match obj {
        Object::Dictionary(mut dict) => {
            for (_, value) in dict.iter_mut() {
                if let Object::Reference(id) = value {
                    *value =
                        Object::Reference(deep_copy_object(doc, *id, new_doc, copied_objects)?);
                }
            }
            new_doc.objects.insert(new_id, Object::Dictionary(dict));
        }
        Object::Stream(mut stream) => {
            for (_, value) in stream.dict.iter_mut() {
                if let Object::Reference(id) = value {
                    *value =
                        Object::Reference(deep_copy_object(doc, *id, new_doc, copied_objects)?);
                }
            }
            new_doc.objects.insert(new_id, Object::Stream(stream));
        }
        Object::Array(mut arr) => {
            for item in arr.iter_mut() {
                if let Object::Reference(id) = item {
                    *item = Object::Reference(deep_copy_object(doc, *id, new_doc, copied_objects)?);
                }
            }
            new_doc.objects.insert(new_id, Object::Array(arr));
        }
        _ => {}
    }

    Ok(new_id)
}

#[pyfunction]
pub fn merge_pdfs(paths: Vec<String>, output_path: String) -> PyResult<()> {
    let mut merged_doc = LoDocument::with_version("1.7");
    let mut page_ids = vec![];

    for path in paths {
        let doc = LoDocument::load(path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to load PDF: {}", e))
        })?;
        let mut copied_objects = BTreeMap::new();

        for page_id in doc.get_pages().values() {
            let new_page_id = deep_copy_object(
                &doc,
                *page_id,
                &mut merged_doc,
                &mut copied_objects,
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to copy page: {}", e))
            })?;
            page_ids.push(new_page_id);
        }
    }

    let page_count = page_ids.len() as i32;
    let pages_id = merged_doc.add_object(lopdf::dictionary! {
        "Type" => "Pages",
        "Kids" => page_ids.into_iter().map(lopdf::Object::Reference).collect::<Vec<_>>(),
        "Count" => page_count,
    });

    let catalog_id = merged_doc.add_object(lopdf::dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    merged_doc.trailer.set("Root", catalog_id);
    merged_doc.compress();
    merged_doc.save(&output_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save merged PDF: {}", e))
    })?;

    Ok(())
}

#[pyfunction]
pub fn rotate_pdf(path: String, output_path: String, angle: i32) -> PyResult<()> {
    if angle % 90 != 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Angle must be a multiple of 90",
        ));
    }

    let mut doc = LoDocument::load(&path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to load PDF: {}", e))
    })?;

    for (_, page_id) in doc.get_pages() {
        if let Ok(page_dict) = doc
            .get_object_mut(page_id)
            .and_then(|obj| obj.as_dict_mut())
        {
            let current_rotation = page_dict
                .get(b"Rotate")
                .and_then(|obj| obj.as_i64())
                .unwrap_or(0) as i32;
            let new_rotation = (current_rotation + angle) % 360;
            page_dict.set(b"Rotate", lopdf::Object::Integer(new_rotation as i64));
        }
    }

    doc.save(&output_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save rotated PDF: {}", e))
    })?;

    Ok(())
}

#[pyfunction]
pub fn split_pdf(
    path: String,
    output_path: String,
    start_page: u32,
    end_page: u32,
) -> PyResult<()> {
    let doc = LoDocument::load(&path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to load PDF: {}", e))
    })?;
    let mut new_doc = LoDocument::with_version("1.7");
    let mut page_ids = vec![];
    let mut copied_objects = BTreeMap::new();

    for page_num in start_page..=end_page {
        if let Some(page_id) = doc.get_pages().get(&page_num) {
            let new_page_id = deep_copy_object(&doc, *page_id, &mut new_doc, &mut copied_objects)
                .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to copy page: {}", e))
            })?;
            page_ids.push(new_page_id);
        }
    }

    let page_count = page_ids.len() as i32;
    let pages_id = new_doc.add_object(lopdf::dictionary! {
        "Type" => "Pages",
        "Kids" => page_ids.into_iter().map(lopdf::Object::Reference).collect::<Vec<_>>(),
        "Count" => page_count,
    });

    let catalog_id = new_doc.add_object(lopdf::dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    new_doc.trailer.set("Root", catalog_id);
    new_doc.compress();
    new_doc.save(&output_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save split PDF: {}", e))
    })?;

    Ok(())
}
