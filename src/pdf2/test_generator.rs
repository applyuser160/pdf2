use crate::generator::generate_pdf;
use crate::structure::{Document, Page, TextBlock};
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pdf_creates_file() {
        // Arrange
        let text_block = TextBlock {
            text: "Test PDF content".to_string(),
            x: 10.0,
            y: 280.0,
            font_size: 12.0,
        };
        let page = Page {
            width: 210.0,
            height: 297.0,
            text_blocks: vec![text_block],
            images: vec![],
        };
        let document = Document { pages: vec![page] };
        let output_path = Path::new("test_output.pdf");

        // Clean up existing file
        if output_path.exists() {
            fs::remove_file(output_path).unwrap();
        }

        // Act
        let result = generate_pdf(&document, output_path);

        // Assert
        assert!(result.is_ok(), "PDF generation should succeed");
        assert!(output_path.exists(), "PDF file should be created");

        // Clean up
        if output_path.exists() {
            fs::remove_file(output_path).unwrap();
        }
    }

    #[test]
    fn test_generate_pdf_with_empty_document() {
        // Arrange
        let document = Document { pages: vec![] };
        let output_path = Path::new("test_empty.pdf");

        // Clean up existing file
        if output_path.exists() {
            fs::remove_file(output_path).unwrap();
        }

        // Act
        let result = generate_pdf(&document, output_path);

        // Assert
        assert!(
            result.is_ok(),
            "PDF generation should succeed even with empty document"
        );
        assert!(output_path.exists(), "PDF file should be created");

        // Clean up
        if output_path.exists() {
            fs::remove_file(output_path).unwrap();
        }
    }

    #[test]
    fn test_generate_pdf_with_multiple_pages() {
        // Arrange
        let text_block1 = TextBlock {
            text: "Page 1 content".to_string(),
            x: 10.0,
            y: 280.0,
            font_size: 12.0,
        };
        let text_block2 = TextBlock {
            text: "Page 2 content".to_string(),
            x: 10.0,
            y: 280.0,
            font_size: 12.0,
        };

        let page1 = Page {
            width: 210.0,
            height: 297.0,
            text_blocks: vec![text_block1],
            images: vec![],
        };
        let page2 = Page {
            width: 210.0,
            height: 297.0,
            text_blocks: vec![text_block2],
            images: vec![],
        };

        let document = Document {
            pages: vec![page1, page2],
        };
        let output_path = Path::new("test_multiple_pages.pdf");

        // Clean up existing file
        if output_path.exists() {
            fs::remove_file(output_path).unwrap();
        }

        // Act
        let result = generate_pdf(&document, output_path);

        // Assert
        assert!(
            result.is_ok(),
            "PDF generation should succeed with multiple pages"
        );
        assert!(output_path.exists(), "PDF file should be created");

        // Clean up
        if output_path.exists() {
            fs::remove_file(output_path).unwrap();
        }
    }
}
