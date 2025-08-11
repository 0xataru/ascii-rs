use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Image data entity representing an uploaded image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub id: Uuid,
    pub original_filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ImageData {
    /// Create a new ImageData instance
    pub fn new(
        original_filename: String,
        content_type: String,
        data: Vec<u8>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            original_filename,
            content_type,
            data,
            width,
            height,
        }
    }

    /// Get the aspect ratio of the image
    pub fn aspect_ratio(&self) -> f32 {
        self.height as f32 / self.width as f32
    }

    /// Check if the image is valid (has content)
    pub fn is_valid(&self) -> bool {
        !self.data.is_empty() && self.width > 0 && self.height > 0
    }
}
