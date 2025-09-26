use super::pdf_operations::*;
use lopdf::{dictionary, Document as LoDocument};
use std::fs;

fn create_dummy_pdf(text: &str, page_count: u32) -> LoDocument {
    let mut doc = LoDocument::with_version("1.7");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! { "F1" => font_id, },
    });
    let mut page_ids = vec![];
    for i in 0..page_count {
        let content = format!("BT /F1 24 Tf 100 700 Td ({} {}) Tj ET", text, i + 1);
        let content_id = doc.add_object(lopdf::Stream::new(
            dictionary! {},
            content.as_bytes().to_vec(),
        ));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
            "Contents" => content_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
            "Rotate" => lopdf::Object::Integer(0),
        });
        page_ids.push(page_id.into());
    }

    doc.objects.insert(
        pages_id,
        dictionary! {
            "Type" => "Pages",
            "Kids" => page_ids,
            "Count" => page_count as i32,
        }
        .into(),
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc
}

#[test]
fn test_merge_pdfs() {
    // Arrange
    let mut doc1 = create_dummy_pdf("Doc1", 1);
    let mut doc2 = create_dummy_pdf("Doc2", 1);
    doc1.save("test_merge_1.pdf").unwrap();
    doc2.save("test_merge_2.pdf").unwrap();
    let paths = vec![
        "test_merge_1.pdf".to_string(),
        "test_merge_2.pdf".to_string(),
    ];

    // Act
    merge_pdfs(paths, "merged.pdf".to_string()).unwrap();

    // Assert
    let merged_doc = LoDocument::load("merged.pdf").unwrap();
    assert_eq!(merged_doc.get_pages().len(), 2);

    let page1_content = merged_doc
        .get_page_content(merged_doc.get_pages().get(&1).unwrap().clone())
        .unwrap();
    let page1_text = String::from_utf8(page1_content).unwrap();
    assert!(page1_text.contains("Doc1"));

    let page2_content = merged_doc
        .get_page_content(merged_doc.get_pages().get(&2).unwrap().clone())
        .unwrap();
    let page2_text = String::from_utf8(page2_content).unwrap();
    assert!(page2_text.contains("Doc2"));

    // Clean up
    fs::remove_file("test_merge_1.pdf").unwrap();
    fs::remove_file("test_merge_2.pdf").unwrap();
    fs::remove_file("merged.pdf").unwrap();
}

#[test]
fn test_split_pdf() {
    // Arrange
    let mut doc = create_dummy_pdf("Page", 3);
    doc.save("test_split.pdf").unwrap();

    // Act
    split_pdf("test_split.pdf".to_string(), "split.pdf".to_string(), 2, 3).unwrap();

    // Assert
    let split_doc = LoDocument::load("split.pdf").unwrap();
    assert_eq!(split_doc.get_pages().len(), 2);

    let page1_content = split_doc
        .get_page_content(split_doc.get_pages().get(&1).unwrap().clone())
        .unwrap();
    let page1_text = String::from_utf8(page1_content).unwrap();
    assert!(page1_text.contains("Page 2"));

    let page2_content = split_doc
        .get_page_content(split_doc.get_pages().get(&2).unwrap().clone())
        .unwrap();
    let page2_text = String::from_utf8(page2_content).unwrap();
    assert!(page2_text.contains("Page 3"));

    // Clean up
    fs::remove_file("test_split.pdf").unwrap();
    fs::remove_file("split.pdf").unwrap();
}

#[test]
fn test_rotate_pdf() {
    // Arrange
    let mut doc = create_dummy_pdf("Page", 1);
    doc.save("test_rotate.pdf").unwrap();

    // Act
    rotate_pdf("test_rotate.pdf".to_string(), "rotated.pdf".to_string(), 90).unwrap();

    // Assert
    let rotated_doc = LoDocument::load("rotated.pdf").unwrap();
    let page_id = rotated_doc.get_pages().get(&1).unwrap().clone();
    let page_dict = rotated_doc.get_object(page_id).unwrap().as_dict().unwrap();
    let rotation = page_dict.get(b"Rotate").unwrap().as_i64().unwrap();
    assert_eq!(rotation, 90);

    // Clean up
    fs::remove_file("test_rotate.pdf").unwrap();
    fs::remove_file("rotated.pdf").unwrap();
}
