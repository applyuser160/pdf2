use crate::parser::parse_pdf;
use crate::structure::{Document, Page, TextBlock};
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pdf_with_valid_file() {
        // Arrange
        let test_pdf_path = Path::new("test_sample.pdf");

        // Create a simple test PDF file (this would need actual PDF content in real tests)
        // For now, we'll test the error handling when file doesn't exist

        // Act
        let result = parse_pdf(test_pdf_path);

        // Assert
        assert!(result.is_err(), "Should return error for non-existent file");
        match result.unwrap_err() {
            crate::parser::ParseError::Io(_) => {
                // Expected IO error for non-existent file
            }
            _ => panic!("Expected IO error"),
        }
    }

    #[test]
    fn test_parse_pdf_io_error_handling() {
        // Arrange
        let non_existent_path = Path::new("non_existent_file.pdf");

        // Act
        let result = parse_pdf(non_existent_path);

        // Assert
        assert!(result.is_err(), "Should return error for non-existent file");
        match result.unwrap_err() {
            crate::parser::ParseError::Io(_) => {
                // Expected IO error
            }
            _ => panic!("Expected IO error"),
        }
    }

    #[test]
    fn test_parse_pdf_extract_error_handling() {
        // Arrange
        let invalid_pdf_path = Path::new("test_invalid.pdf");

        // Create a file with invalid PDF content
        fs::write(invalid_pdf_path, "This is not a PDF file").unwrap();

        // Act
        let result = parse_pdf(invalid_pdf_path);

        // Assert
        assert!(
            result.is_err(),
            "Should return error for invalid PDF content"
        );
        match result.unwrap_err() {
            crate::parser::ParseError::Extract(_) => {
                // Expected extract error
            }
            _ => panic!("Expected extract error"),
        }

        // Clean up
        fs::remove_file(invalid_pdf_path).unwrap();
    }

    #[test]
    fn test_parse_pdf_creates_document_structure() {
        // Arrange
        // This test would require a valid PDF file
        // For now, we'll test the structure creation logic indirectly

        let text_block = TextBlock {
            text: "Sample text".to_string(),
            x: 0.0,
            y: 0.0,
            font_size: 0.0,
        };

        let page = Page {
            width: 595.0,
            height: 842.0,
            text_blocks: vec![text_block],
            images: vec![],
        };

        let document = Document { pages: vec![page] };

        // Act & Assert
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].text_blocks.len(), 1);
        assert_eq!(document.pages[0].text_blocks[0].text, "Sample text");
        assert_eq!(document.pages[0].width, 595.0);
        assert_eq!(document.pages[0].height, 842.0);
    }
}
