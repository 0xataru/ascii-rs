use crate::domain::{entities::ImageData, repositories::ImageRepository};
use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum InMemoryImageRepositoryError {
    #[error("Image not found")]
    NotFound,
}

/// In-memory implementation of ImageRepository for development/testing
#[derive(Clone)]
pub struct InMemoryImageRepository {
    storage: Arc<RwLock<HashMap<Uuid, ImageData>>>,
}

impl InMemoryImageRepository {
    /// Create a new in-memory image repository
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryImageRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ImageRepository for InMemoryImageRepository {
    type Error = InMemoryImageRepositoryError;

    async fn save(&self, image: &ImageData) -> Result<(), Self::Error> {
        let mut storage = self.storage.write().await;
        storage.insert(image.id, image.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ImageData>, Self::Error> {
        let storage = self.storage.read().await;
        Ok(storage.get(&id).cloned())
    }

    async fn delete(&self, id: Uuid) -> Result<(), Self::Error> {
        let mut storage = self.storage.write().await;
        storage.remove(&id);
        Ok(())
    }
}
