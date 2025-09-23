use crate::structure::{Document, Page, TextBlock};
use std::path::Path;
use std::fs;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("PDF extraction error: {0}")]
    Extract(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn parse_pdf(path: &Path) -> Result<Document, ParseError> {
    let bytes = fs::read(path)?;
    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| ParseError::Extract(e.to_string()))?;

    // pdf-extract returns the text for the whole document in one string,
    // with pages separated by form feed characters (\f).
    // We will split the text by this character to create pages.
    let pages_text: Vec<&str> = text.split('\u{000C}').collect();

    let mut pages = Vec::new();
    for page_text in pages_text.iter() {
        // Only create a page if there is some text, to avoid empty pages at the end.
        if !page_text.trim().is_empty() {
            let text_block = TextBlock {
                text: page_text.to_string(),
                x: 0.0, // Positional info is not available from this library
                y: 0.0,
                font_size: 0.0,
            };
            let page = Page {
                width: 595.0, // Using standard A4 size as a placeholder
                height: 842.0,
                text_blocks: vec![text_block],
                images: vec![],
            };
            pages.push(page);
        }
    }

    Ok(Document { pages })
}
