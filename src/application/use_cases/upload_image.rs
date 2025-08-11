use crate::domain::{
    entities::ImageData,
    repositories::ImageRepository,
    value_objects::ImageFormat,
};
use image::GenericImageView;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UploadImageError {
    #[error("Unsupported image format")]
    UnsupportedFormat,
    #[error("Invalid image data")]
    InvalidImageData,
    #[error("Image too large (max size: {max_size} bytes)")]
    ImageTooLarge { max_size: usize },
    #[error("Repository error: {0}")]
    Repository(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Use case for uploading and validating images
pub struct UploadImageUseCase<R: ImageRepository> {
    repository: Arc<R>,
    max_file_size: usize,
}

#[derive(Debug)]
pub struct UploadImageRequest {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct UploadImageResponse {
    pub image_id: Uuid,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
}

impl<R: ImageRepository> UploadImageUseCase<R> {
    /// Create a new upload image use case
    pub fn new(repository: Arc<R>, max_file_size: usize) -> Self {
        Self {
            repository,
            max_file_size,
        }
    }

    /// Execute the upload image use case
    pub async fn execute(
        &self,
        request: UploadImageRequest,
    ) -> Result<UploadImageResponse, UploadImageError> {
        // Validate file size
        if request.data.len() > self.max_file_size {
            return Err(UploadImageError::ImageTooLarge {
                max_size: self.max_file_size,
            });
        }

        // Validate content type
        let format = ImageFormat::from_mime_type(&request.content_type)
            .ok_or(UploadImageError::UnsupportedFormat)?;

        // Validate image data by attempting to decode it
        let img = image::load_from_memory(&request.data)
            .map_err(|_| UploadImageError::InvalidImageData)?;

        let (width, height) = img.dimensions();

        // Create image entity
        let image_data = ImageData::new(
            request.filename,
            request.content_type,
            request.data,
            width,
            height,
        );

        // Save to repository
        self.repository
            .save(&image_data)
            .await
            .map_err(|e| UploadImageError::Repository(Box::new(e)))?;

        Ok(UploadImageResponse {
            image_id: image_data.id,
            format,
            width,
            height,
        })
    }
}
