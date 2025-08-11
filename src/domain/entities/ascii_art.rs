use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ASCII art entity representing the converted result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsciiArt {
    pub id: Uuid,
    pub image_id: Uuid,
    pub content: String,
    pub width: u32,
    pub height: u32,
    pub detail_level: DetailLevel,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Level of detail for ASCII art conversion
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DetailLevel {
    Low,
    High,
}

impl DetailLevel {
    /// Get the ASCII character set for this detail level
    pub fn char_set(&self) -> &'static str {
        match self {
            DetailLevel::Low => " .-:=+*#%@",
            DetailLevel::High => " .`'^,:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$",
        }
    }
}

impl AsciiArt {
    /// Create a new AsciiArt instance
    pub fn new(
        image_id: Uuid,
        content: String,
        width: u32,
        height: u32,
        detail_level: DetailLevel,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            image_id,
            content,
            width,
            height,
            detail_level,
            created_at: chrono::Utc::now(),
        }
    }

    /// Check if the ASCII art is valid
    pub fn is_valid(&self) -> bool {
        !self.content.is_empty() && self.width > 0 && self.height > 0
    }

    /// Get the number of lines in the ASCII art
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }
}
