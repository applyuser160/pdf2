"""
Type stubs for pdf2._core module.

This file provides type information for the Rust-based core module.
"""

from typing import List

class TextBlock:
    """Represents a single text block with its content and position."""

    text: str
    x: float
    y: float
    font_size: float

    def __init__(self, text: str, x: float, y: float, font_size: float) -> None: ...

class Image:
    """Represents an image with its data and position."""

    x: float
    y: float
    width: float
    height: float
    data: bytes
    format: str

    def __init__(
        self, x: float, y: float, width: float, height: float, data: bytes, format: str
    ) -> None: ...

class Page:
    """Represents a single page in the document."""

    width: float
    height: float
    text_blocks: List[TextBlock]
    images: List[Image]

    def __init__(
        self,
        width: float,
        height: float,
        text_blocks: List[TextBlock],
        images: List[Image],
    ) -> None: ...

class Document:
    """Represents the entire PDF document."""

    pages: List[Page]

    def __init__(self, pages: List[Page]) -> None: ...

def parse(path_str: str) -> Document:
    """Parse a PDF file and return a Document object.

    Args:
        path_str: Path to the PDF file to parse

    Returns:
        Document object containing the parsed PDF data

    Raises:
        ValueError: If the PDF file cannot be parsed
    """
    ...

def generate(doc: Document, path_str: str) -> None:
    """Generate a PDF file from a Document object.

    Args:
        doc: Document object to generate PDF from
        path_str: Path where the PDF file should be saved

    Raises:
        NotImplementedError: If PDF generation fails
    """
    ...

def merge_pdfs(paths: List[str], output_path: str) -> None:
    """Merge multiple PDF files into a single PDF.

    Args:
        paths: List of paths to the PDF files to merge
        output_path: Path where the merged PDF file should be saved

    Raises:
        ValueError: If a PDF file cannot be loaded
        IOError: If the merged PDF file cannot be saved
    """
    ...

def split_pdf(path: str, output_path: str, start_page: int, end_page: int) -> None:
    """Split a PDF file into a range of pages.

    Args:
        path: Path to the PDF file to split
        output_path: Path where the split PDF file should be saved
        start_page: The first page to include in the split PDF
        end_page: The last page to include in the split PDF

    Raises:
        ValueError: If the PDF file cannot be loaded
        IOError: If the split PDF file cannot be saved
    """
    ...

def rotate_pdf(path: str, output_path: str, angle: int) -> None:
    """Rotate all pages in a PDF by a specified angle.

    The angle must be a multiple of 90.

    Args:
        path: Path to the PDF file to rotate
        output_path: Path where the rotated PDF file should be saved
        angle: The angle to rotate the pages by

    Raises:
        ValueError: If the PDF file cannot be loaded or the angle is not a multiple of 90
        IOError: If the rotated PDF file cannot be saved
    """
    ...
