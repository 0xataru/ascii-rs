use crate::domain::{entities::AsciiArt, repositories::AsciiArtRepository};
use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum InMemoryAsciiArtRepositoryError {
    #[error("ASCII art not found")]
    NotFound,
}

/// In-memory implementation of AsciiArtRepository for development/testing
#[derive(Clone)]
pub struct InMemoryAsciiArtRepository {
    storage: Arc<RwLock<HashMap<Uuid, AsciiArt>>>,
}

impl InMemoryAsciiArtRepository {
    /// Create a new in-memory ASCII art repository
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryAsciiArtRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AsciiArtRepository for InMemoryAsciiArtRepository {
    type Error = InMemoryAsciiArtRepositoryError;

    async fn save(&self, ascii_art: &AsciiArt) -> Result<(), Self::Error> {
        let mut storage = self.storage.write().await;
        storage.insert(ascii_art.id, ascii_art.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AsciiArt>, Self::Error> {
        let storage = self.storage.read().await;
        Ok(storage.get(&id).cloned())
    }

    async fn find_by_image_id(&self, image_id: Uuid) -> Result<Vec<AsciiArt>, Self::Error> {
        let storage = self.storage.read().await;
        let results: Vec<AsciiArt> = storage
            .values()
            .filter(|ascii_art| ascii_art.image_id == image_id)
            .cloned()
            .collect();
        Ok(results)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Self::Error> {
        let mut storage = self.storage.write().await;
        storage.remove(&id);
        Ok(())
    }
}
