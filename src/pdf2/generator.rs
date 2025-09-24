use crate::pdf2::structure::Document;
use printpdf::{Mm, PdfDocument};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum GenerateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("PDF generation error: {0}")]
    Pdf(String),
}

pub fn generate_pdf(doc: &Document, path: &Path) -> Result<(), GenerateError> {
    let (pdf_doc, page1, layer1) =
        PdfDocument::new("Generated PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = pdf_doc.get_page(page1).get_layer(layer1);
    let font = pdf_doc
        .add_builtin_font(printpdf::BuiltinFont::Helvetica)
        .unwrap();

    let full_text = doc
        .pages
        .iter()
        .flat_map(|p| p.text_blocks.iter())
        .map(|tb| tb.text.as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    current_layer.use_text(full_text, 12.0, Mm(10.0), Mm(280.0), &font);

    pdf_doc
        .save(&mut BufWriter::new(File::create(path)?))
        .map_err(|e| GenerateError::Pdf(e.to_string()))?;

    Ok(())
}
