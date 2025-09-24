use serde::{Deserialize, Serialize};

// Using pyo3's prelude to get access to the #[pyclass] macro if needed later.
// And to derive PyObjectProtocol for our structs.
use pyo3::prelude::*;

/// Represents a single text block with its content and position.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct TextBlock {
    #[pyo3(get, set)]
    pub text: String,
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub font_size: f32,
    // We can add more properties like font name, color, etc. later.
}

/// Represents an image with its data and position.
/// For now, we'll just store the raw image data and its format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Image {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub width: f32,
    #[pyo3(get, set)]
    pub height: f32,
    // The raw image data.
    pub data: Vec<u8>,
    // E.g., "jpeg", "png", etc.
    #[pyo3(get, set)]
    pub format: String,
}

/// An enum to represent any element that can be on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageContent {
    Text(TextBlock),
    Image(Image),
    // We can add Shape, Table, etc. later.
}

/// Represents a single page in the document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Page {
    #[pyo3(get, set)]
    pub width: f32,
    #[pyo3(get, set)]
    pub height: f32,
    // pub contents: Vec<PageContent>, // This will cause issues with pyo3 if PageContent is not a pyclass
    // For now, let's keep it simple and add specific vectors for each type.
    #[pyo3(get, set)]
    pub text_blocks: Vec<TextBlock>,
    #[pyo3(get, set)]
    pub images: Vec<Image>,
}

/// Represents the entire PDF document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Document {
    #[pyo3(get, set)]
    pub pages: Vec<Page>,
}

#[pymethods]
impl TextBlock {
    #[new]
    fn new(text: String, x: f32, y: f32, font_size: f32) -> Self {
        TextBlock {
            text,
            x,
            y,
            font_size,
        }
    }
}

#[pymethods]
impl Image {
    #[new]
    fn new(x: f32, y: f32, width: f32, height: f32, data: Vec<u8>, format: String) -> Self {
        Image {
            x,
            y,
            width,
            height,
            data,
            format,
        }
    }
}

#[pymethods]
impl Page {
    #[new]
    fn new(width: f32, height: f32, text_blocks: Vec<TextBlock>, images: Vec<Image>) -> Self {
        Page {
            width,
            height,
            text_blocks,
            images,
        }
    }
}

#[pymethods]
impl Document {
    #[new]
    fn new(pages: Vec<Page>) -> Self {
        Document { pages }
    }
}
