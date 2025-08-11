use crate::{
    application::services::AsciiConversionService,
    domain::{
        entities::AsciiArt,
        repositories::{AsciiArtRepository, ImageRepository},
        value_objects::ConversionConfig,
    },
};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ConvertImageError {
    #[error("Image not found")]
    ImageNotFound,
    #[error("Invalid conversion configuration")]
    InvalidConfig,
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
    #[error("Repository error: {0}")]
    Repository(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Use case for converting images to ASCII art
pub struct ConvertImageToAsciiUseCase<IR: ImageRepository, AR: AsciiArtRepository> {
    image_repository: Arc<IR>,
    ascii_art_repository: Arc<AR>,
    conversion_service: Arc<AsciiConversionService>,
}

#[derive(Debug)]
pub struct ConvertImageRequest {
    pub image_id: Uuid,
    pub config: ConversionConfig,
}

#[derive(Debug)]
pub struct ConvertImageResponse {
    pub ascii_art_id: Uuid,
    pub content: String,
    pub width: u32,
    pub height: u32,
}

impl<IR: ImageRepository, AR: AsciiArtRepository> ConvertImageToAsciiUseCase<IR, AR> {
    /// Create a new convert image use case
    pub fn new(
        image_repository: Arc<IR>,
        ascii_art_repository: Arc<AR>,
        conversion_service: Arc<AsciiConversionService>,
    ) -> Self {
        Self {
            image_repository,
            ascii_art_repository,
            conversion_service,
        }
    }

    /// Execute the convert image use case
    pub async fn execute(
        &self,
        request: ConvertImageRequest,
    ) -> Result<ConvertImageResponse, ConvertImageError> {
        // Validate configuration
        if !request.config.is_valid() {
            return Err(ConvertImageError::InvalidConfig);
        }

        // Get image from repository
        let image_data = self
            .image_repository
            .find_by_id(request.image_id)
            .await
            .map_err(|e| ConvertImageError::Repository(Box::new(e)))?
            .ok_or(ConvertImageError::ImageNotFound)?;

        // Convert image to ASCII
        let ascii_content = self
            .conversion_service
            .convert_to_ascii(&image_data, &request.config)
            .await
            .map_err(|e| ConvertImageError::ConversionFailed(e.to_string()))?;

        // Calculate ASCII dimensions
        let lines: Vec<&str> = ascii_content.lines().collect();
        let height = lines.len() as u32;
        let width = lines.first().map(|line| line.len()).unwrap_or(0) as u32;

        // Create ASCII art entity
        let ascii_art = AsciiArt::new(
            request.image_id,
            ascii_content,
            width,
            height,
            request.config.detail_level,
        );

        // Save ASCII art
        self.ascii_art_repository
            .save(&ascii_art)
            .await
            .map_err(|e| ConvertImageError::Repository(Box::new(e)))?;

        Ok(ConvertImageResponse {
            ascii_art_id: ascii_art.id,
            content: ascii_art.content,
            width: ascii_art.width,
            height: ascii_art.height,
        })
    }
}
