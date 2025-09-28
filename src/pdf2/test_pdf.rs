#[cfg(test)]
mod tests {
    use crate::pdf::Pdf;

    #[test]
    fn test_pdf_read() {
        let pdf = Pdf::new("test_final.pdf");
        let document = pdf.read();
        assert!(document.is_ok());
    }

    #[test]
    fn test_pdf_write() {
        let pdf = Pdf::new("test_final.pdf");
        let document = pdf.read();
        assert!(document.is_ok());
        let document = document.unwrap();
        let result = pdf.write(&document);
        assert!(result.is_ok());
    }
}
