use crate::domain::entities::AsciiArt;
use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

/// Repository interface for ASCII art storage
#[async_trait]
pub trait AsciiArtRepository: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    /// Save ASCII art
    async fn save(&self, ascii_art: &AsciiArt) -> Result<(), Self::Error>;

    /// Find ASCII art by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AsciiArt>, Self::Error>;

    /// Find ASCII art by image ID
    async fn find_by_image_id(&self, image_id: Uuid) -> Result<Vec<AsciiArt>, Self::Error>;

    /// Delete ASCII art by ID
    async fn delete(&self, id: Uuid) -> Result<(), Self::Error>;
}
