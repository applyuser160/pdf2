use super::metadata::{get_metadata, set_metadata, Metadata};
use chrono::Utc;
use lopdf::Document;
use printpdf::{Mm, PdfDocument};
use std::fs;
use std::io::BufWriter;

#[test]
fn test_get_and_set_metadata() {
    // Arrange
    let (doc, _, _) = PdfDocument::new("Test PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let now = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

    let initial_metadata = Metadata {
        title: Some("Initial Title".to_string()),
        author: Some("Initial Author".to_string()),
        subject: Some("Initial Subject".to_string()),
        keywords: Some("Initial, Keywords".to_string()),
        creator: Some("Initial Creator".to_string()),
        producer: Some("Initial Producer".to_string()),
        creation_date: Some(now),
        mod_date: Some(now),
    };

    let test_pdf_path = "test_metadata.pdf";
    doc.save(&mut BufWriter::new(
        fs::File::create(test_pdf_path).unwrap(),
    ))
    .unwrap();

    // Act: Set initial metadata
    set_metadata(test_pdf_path, &initial_metadata).unwrap();

    // Assert: Get metadata and check if it matches the initial metadata
    let retrieved_metadata = get_metadata(test_pdf_path).unwrap();
    assert_eq!(retrieved_metadata.title, initial_metadata.title);
    assert_eq!(retrieved_metadata.author, initial_metadata.author);
    assert_eq!(retrieved_metadata.subject, initial_metadata.subject);
    assert_eq!(retrieved_metadata.keywords, initial_metadata.keywords);
    assert_eq!(retrieved_metadata.creator, initial_metadata.creator);
    assert_eq!(retrieved_metadata.producer, initial_metadata.producer);
    assert_eq!(
        retrieved_metadata.creation_date.unwrap().timestamp(),
        initial_metadata.creation_date.unwrap().timestamp()
    );
    assert_eq!(
        retrieved_metadata.mod_date.unwrap().timestamp(),
        initial_metadata.mod_date.unwrap().timestamp()
    );

    // Arrange: Update metadata
    let updated_metadata = Metadata {
        title: Some("Updated Title".to_string()),
        author: Some("Updated Author".to_string()),
        ..Default::default()
    };

    // Act: Set updated metadata
    set_metadata(test_pdf_path, &updated_metadata).unwrap();

    // Assert: Get metadata and check if it matches the updated metadata
    let retrieved_metadata_after_update = get_metadata(test_pdf_path).unwrap();
    assert_eq!(
        retrieved_metadata_after_update.title,
        updated_metadata.title
    );
    assert_eq!(
        retrieved_metadata_after_update.author,
        updated_metadata.author
    );

    // Cleanup
    fs::remove_file(test_pdf_path).unwrap();
}

#[test]
fn test_get_metadata_no_info_dict() {
    // Arrange
    let (doc, _, _) = PdfDocument::new("Test PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let test_pdf_path = "test_no_info.pdf";
    doc.save(&mut BufWriter::new(
        fs::File::create(test_pdf_path).unwrap(),
    ))
    .unwrap();

    // Remove the Info dictionary to simulate a PDF without one
    let mut doc_no_info = Document::load(test_pdf_path).unwrap();
    doc_no_info.trailer.remove(b"Info");
    doc_no_info.save(test_pdf_path).unwrap();

    // Act
    let metadata = get_metadata(test_pdf_path).unwrap();

    // Assert
    assert!(metadata.title.is_none());
    assert!(metadata.author.is_none());

    // Cleanup
    fs::remove_file(test_pdf_path).unwrap();
}
