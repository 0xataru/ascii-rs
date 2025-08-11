use crate::domain::entities::ImageData;
use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

/// Repository interface for image storage
#[async_trait]
pub trait ImageRepository: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    /// Save image data
    async fn save(&self, image: &ImageData) -> Result<(), Self::Error>;

    /// Find image by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ImageData>, Self::Error>;

    /// Delete image by ID
    async fn delete(&self, id: Uuid) -> Result<(), Self::Error>;
}
