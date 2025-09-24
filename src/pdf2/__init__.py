"""
PDF2 - A Python library for PDF generation and parsing.

This module provides functionality to create, manipulate, and generate PDF documents
using Rust-based implementations for high performance.
"""

from ._core import (
    Document,
    Page,
    TextBlock,
    Image,
    parse,
    generate,
)

__version__ = "0.1.0"
__all__ = [
    "Document",
    "Page",
    "TextBlock",
    "Image",
    "parse",
    "generate",
]
