use crate::structure::{Document, Image, Page, TextBlock};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_block_creation() {
        // Arrange
        let text = "Test text content".to_string();
        let x = 10.0;
        let y = 280.0;
        let font_size = 12.0;

        // Act
        let text_block = TextBlock {
            text: text.clone(),
            x,
            y,
            font_size,
        };

        // Assert
        assert_eq!(text_block.text, text);
        assert_eq!(text_block.x, x);
        assert_eq!(text_block.y, y);
        assert_eq!(text_block.font_size, font_size);
    }

    #[test]
    fn test_text_block_new_method() {
        // Arrange
        let text = "Test text".to_string();
        let x = 15.0;
        let y = 300.0;
        let font_size = 14.0;

        // Act
        let text_block = TextBlock::new(text.clone(), x, y, font_size);

        // Assert
        assert_eq!(text_block.text, text);
        assert_eq!(text_block.x, x);
        assert_eq!(text_block.y, y);
        assert_eq!(text_block.font_size, font_size);
    }

    #[test]
    fn test_image_creation() {
        // Arrange
        let x = 50.0;
        let y = 100.0;
        let width = 200.0;
        let height = 150.0;
        let data = vec![1, 2, 3, 4, 5];
        let format = "jpeg".to_string();

        // Act
        let image = Image {
            x,
            y,
            width,
            height,
            data: data.clone(),
            format: format.clone(),
        };

        // Assert
        assert_eq!(image.x, x);
        assert_eq!(image.y, y);
        assert_eq!(image.width, width);
        assert_eq!(image.height, height);
        assert_eq!(image.data, data);
        assert_eq!(image.format, format);
    }

    #[test]
    fn test_image_new_method() {
        // Arrange
        let x = 25.0;
        let y = 75.0;
        let width = 100.0;
        let height = 80.0;
        let data = vec![10, 20, 30];
        let format = "png".to_string();

        // Act
        let image = Image::new(x, y, width, height, data.clone(), format.clone());

        // Assert
        assert_eq!(image.x, x);
        assert_eq!(image.y, y);
        assert_eq!(image.width, width);
        assert_eq!(image.height, height);
        assert_eq!(image.data, data);
        assert_eq!(image.format, format);
    }

    #[test]
    fn test_page_creation() {
        // Arrange
        let width = 210.0;
        let height = 297.0;
        let text_block = TextBlock::new("Page content".to_string(), 10.0, 280.0, 12.0);
        let image = Image::new(50.0, 100.0, 100.0, 80.0, vec![1, 2, 3], "jpeg".to_string());
        let text_blocks = vec![text_block];
        let images = vec![image];

        // Act
        let page = Page {
            width,
            height,
            text_blocks: text_blocks.clone(),
            images: images.clone(),
        };

        // Assert
        assert_eq!(page.width, width);
        assert_eq!(page.height, height);
        assert_eq!(page.text_blocks.len(), 1);
        assert_eq!(page.images.len(), 1);
        assert_eq!(page.text_blocks[0].text, "Page content");
        assert_eq!(page.images[0].format, "jpeg");
    }

    #[test]
    fn test_page_new_method() {
        // Arrange
        let width = 595.0;
        let height = 842.0;
        let text_block = TextBlock::new("New page content".to_string(), 20.0, 300.0, 14.0);
        let text_blocks = vec![text_block];
        let images = vec![];

        // Act
        let page = Page::new(width, height, text_blocks.clone(), images.clone());

        // Assert
        assert_eq!(page.width, width);
        assert_eq!(page.height, height);
        assert_eq!(page.text_blocks.len(), 1);
        assert_eq!(page.images.len(), 0);
        assert_eq!(page.text_blocks[0].text, "New page content");
    }

    #[test]
    fn test_document_creation() {
        // Arrange
        let page1 = Page::new(210.0, 297.0, vec![], vec![]);
        let page2 = Page::new(210.0, 297.0, vec![], vec![]);
        let pages = vec![page1, page2];

        // Act
        let document = Document {
            pages: pages.clone(),
        };

        // Assert
        assert_eq!(document.pages.len(), 2);
        assert_eq!(document.pages[0].width, 210.0);
        assert_eq!(document.pages[1].width, 210.0);
    }

    #[test]
    fn test_document_new_method() {
        // Arrange
        let text_block = TextBlock::new("Document content".to_string(), 10.0, 280.0, 12.0);
        let page = Page::new(210.0, 297.0, vec![text_block], vec![]);
        let pages = vec![page];

        // Act
        let document = Document::new(pages.clone());

        // Assert
        assert_eq!(document.pages.len(), 1);
        assert_eq!(document.pages[0].text_blocks.len(), 1);
        assert_eq!(document.pages[0].text_blocks[0].text, "Document content");
    }

    #[test]
    fn test_empty_document() {
        // Arrange
        let pages = vec![];

        // Act
        let document = Document::new(pages);

        // Assert
        assert_eq!(document.pages.len(), 0);
    }

    #[test]
    fn test_page_content_enum() {
        // Arrange
        let text_block = TextBlock::new("Enum test".to_string(), 10.0, 280.0, 12.0);
        let image = Image::new(50.0, 100.0, 100.0, 80.0, vec![1, 2, 3], "png".to_string());

        // Act
        let text_content = crate::structure::PageContent::Text(text_block);
        let image_content = crate::structure::PageContent::Image(image);

        // Assert
        match text_content {
            crate::structure::PageContent::Text(tb) => {
                assert_eq!(tb.text, "Enum test");
            }
            _ => panic!("Expected Text variant"),
        }

        match image_content {
            crate::structure::PageContent::Image(img) => {
                assert_eq!(img.format, "png");
            }
            _ => panic!("Expected Image variant"),
        }
    }
}
