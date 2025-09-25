use super::encryption::encrypt_pdf;
use lopdf::{
    content::{Content, Operation},
    Dictionary, Document, Object, Stream,
};
use rand::random;
use std::fs;

struct TestFixture {
    input_path: String,
    output_path: String,
}

impl TestFixture {
    fn new() -> Self {
        let input_path = "test_input.pdf".to_string();
        let output_path = "test_output.pdf".to_string();
        TestFixture {
            input_path,
            output_path,
        }
    }

    fn create_dummy_pdf(&self) {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Font".to_vec())),
            ("Subtype", Object::Name(b"Type1".to_vec())),
            ("BaseFont", Object::Name(b"Courier".to_vec())),
        ]));
        let resources_id = doc.add_object(Dictionary::from_iter(vec![(
            "Font",
            Dictionary::from_iter(vec![("F1", font_id.into())]).into(),
        )]));
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 48.into()]),
                Operation::new("Td", vec![100.into(), 600.into()]),
                Operation::new("Tj", vec![Object::string_literal("Hello World!")]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(Dictionary::new(), content.encode().unwrap()));
        let page_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Page".to_vec())),
            ("Parent", pages_id.into()),
            ("Contents", content_id.into()),
            ("Resources", resources_id.into()),
            (
                "MediaBox",
                vec![0.into(), 0.into(), 595.into(), 842.into()].into(),
            ),
        ]));
        let pages = Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", vec![page_id.into()].into()),
            ("Count", 1.into()),
        ]);
        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Catalog".to_vec())),
            ("Pages", pages_id.into()),
        ]));
        doc.trailer.set("Root", catalog_id);

        let id1: [u8; 16] = random();
        let id2: [u8; 16] = random();
        doc.trailer.set(
            "ID",
            vec![
                Object::string_literal(id1.to_vec()),
                Object::string_literal(id2.to_vec()),
            ],
        );

        doc.save(&self.input_path).unwrap();
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.input_path);
        let _ = fs::remove_file(&self.output_path);
    }
}

#[test]
fn test_encrypt_pdf_with_correct_password() {
    // Arrange
    let fixture = TestFixture::new();
    fixture.create_dummy_pdf();
    let user_password = "password".to_string();

    // Act
    let result = encrypt_pdf(
        fixture.input_path.clone(),
        fixture.output_path.clone(),
        user_password.clone(),
        None,
    );

    // Assert
    assert!(result.is_ok());

    let mut doc = Document::load(&fixture.output_path).unwrap();
    assert!(doc.is_encrypted());
    assert!(doc.decrypt(&user_password).is_ok());
}

#[test]
fn test_encrypt_pdf_with_incorrect_password() {
    // Arrange
    let fixture = TestFixture::new();
    fixture.create_dummy_pdf();
    let user_password = "password".to_string();
    let wrong_password = "wrong_password".to_string();

    // Act
    let result = encrypt_pdf(
        fixture.input_path.clone(),
        fixture.output_path.clone(),
        user_password.clone(),
        None,
    );

    // Assert
    assert!(result.is_ok());

    let mut doc = Document::load(&fixture.output_path).unwrap();
    assert!(doc.is_encrypted());
    assert!(doc.decrypt(&wrong_password).is_err());
}
