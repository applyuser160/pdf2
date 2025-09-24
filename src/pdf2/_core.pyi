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
