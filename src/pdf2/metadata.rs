use chrono::{DateTime, FixedOffset, TimeZone};
use lopdf::{Document, Object};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[pyclass]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    #[pyo3(get, set)]
    pub title: Option<String>,
    #[pyo3(get, set)]
    pub author: Option<String>,
    #[pyo3(get, set)]
    pub subject: Option<String>,
    #[pyo3(get, set)]
    pub keywords: Option<String>,
    #[pyo3(get, set)]
    pub creator: Option<String>,
    #[pyo3(get, set)]
    pub producer: Option<String>,
    #[pyo3(get, set)]
    pub creation_date: Option<DateTime<FixedOffset>>,
    #[pyo3(get, set)]
    pub mod_date: Option<DateTime<FixedOffset>>,
}

#[pymethods]
impl Metadata {
    #[new]
    fn new() -> Self {
        Metadata::default()
    }
}

pub fn get_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata, String> {
    let doc = Document::load(path).map_err(|e| e.to_string())?;
    let info_dict = match doc.trailer.get(b"Info") {
        Ok(obj) => match doc.get_object(obj.as_reference().unwrap()) {
            Ok(Object::Dictionary(dict)) => dict.clone(),
            _ => return Ok(Metadata::default()),
        },
        _ => return Ok(Metadata::default()),
    };

    let mut metadata = Metadata::default();

    if let Ok(title) = info_dict.get(b"Title") {
        metadata.title = Some(string_from_pdf_object(title));
    }
    if let Ok(author) = info_dict.get(b"Author") {
        metadata.author = Some(string_from_pdf_object(author));
    }
    if let Ok(subject) = info_dict.get(b"Subject") {
        metadata.subject = Some(string_from_pdf_object(subject));
    }
    if let Ok(keywords) = info_dict.get(b"Keywords") {
        metadata.keywords = Some(string_from_pdf_object(keywords));
    }
    if let Ok(creator) = info_dict.get(b"Creator") {
        metadata.creator = Some(string_from_pdf_object(creator));
    }
    if let Ok(producer) = info_dict.get(b"Producer") {
        metadata.producer = Some(string_from_pdf_object(producer));
    }
    if let Ok(creation_date) = info_dict.get(b"CreationDate") {
        metadata.creation_date = parse_pdf_date(&string_from_pdf_object(creation_date));
    }
    if let Ok(mod_date) = info_dict.get(b"ModDate") {
        metadata.mod_date = parse_pdf_date(&string_from_pdf_object(mod_date));
    }
    Ok(metadata)
}

pub fn set_metadata<P: AsRef<Path>>(path: P, metadata: &Metadata) -> Result<(), String> {
    let mut doc = Document::load(&path).map_err(|e| e.to_string())?;
    let info_dict_id = match doc.trailer.get(b"Info") {
        Ok(obj) => obj.as_reference().unwrap(),
        Err(_) => {
            let info_dict = lopdf::Dictionary::new();
            doc.add_object(Object::Dictionary(info_dict))
        }
    };
    doc.trailer.set("Info", Object::Reference(info_dict_id));

    let info_dict = doc
        .get_object_mut(info_dict_id)
        .and_then(|obj| obj.as_dict_mut())
        .map_err(|e| e.to_string())?;

    if let Some(title) = &metadata.title {
        info_dict.set("Title", Object::string_literal(title.clone()));
    }
    if let Some(author) = &metadata.author {
        info_dict.set("Author", Object::string_literal(author.clone()));
    }
    if let Some(subject) = &metadata.subject {
        info_dict.set("Subject", Object::string_literal(subject.clone()));
    }
    if let Some(keywords) = &metadata.keywords {
        info_dict.set("Keywords", Object::string_literal(keywords.clone()));
    }
    if let Some(creator) = &metadata.creator {
        info_dict.set("Creator", Object::string_literal(creator.clone()));
    }
    if let Some(producer) = &metadata.producer {
        info_dict.set("Producer", Object::string_literal(producer.clone()));
    }
    if let Some(creation_date) = &metadata.creation_date {
        info_dict.set(
            "CreationDate",
            Object::string_literal(creation_date.format("D:%Y%m%d%H%M%S%z").to_string()),
        );
    }
    if let Some(mod_date) = &metadata.mod_date {
        info_dict.set(
            "ModDate",
            Object::string_literal(mod_date.format("D:%Y%m%d%H%M%S%z").to_string()),
        );
    }

    let temp_file = tempfile::Builder::new()
        .prefix("pdf_")
        .suffix(".pdf")
        .tempfile()
        .map_err(|e| e.to_string())?;
    doc.save(temp_file.path()).map_err(|e| e.to_string())?;
    std::fs::copy(temp_file.path(), path).map_err(|e| e.to_string())?;
    Ok(())
}

fn string_from_pdf_object(obj: &Object) -> String {
    let bytes = obj.as_str().unwrap();
    String::from_utf8_lossy(bytes).to_string()
}

fn parse_pdf_date(date_str: &str) -> Option<DateTime<FixedOffset>> {
    let mut s = date_str;
    if s.starts_with("D:") {
        s = &s[2..];
    }

    let (s, offset) = if let Some(stripped) = s.strip_suffix('Z') {
        (stripped, FixedOffset::east_opt(0).unwrap())
    } else if let Some(pos) = s.rfind('+') {
        let (date_part, tz_part) = s.split_at(pos);
        let tz_part = &tz_part[1..].replace('\'', "");
        let (hours, minutes) = tz_part.split_at(2);
        let hours: i32 = hours.parse().ok()?;
        let minutes: i32 = minutes.parse().ok()?;
        (date_part, FixedOffset::east_opt(hours * 3600 + minutes * 60).unwrap())
    } else if let Some(pos) = s.rfind('-') {
        let (date_part, tz_part) = s.split_at(pos);
        let tz_part = &tz_part[1..].replace('\'', "");
        let (hours, minutes) = tz_part.split_at(2);
        let hours: i32 = hours.parse().ok()?;
        let minutes: i32 = minutes.parse().ok()?;
        (date_part, FixedOffset::west_opt(hours * 3600 + minutes * 60).unwrap())
    } else {
        (s, FixedOffset::east_opt(0).unwrap())
    };

    let parsed = chrono::NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S").ok()?;
    Some(offset.from_local_datetime(&parsed).unwrap())
}