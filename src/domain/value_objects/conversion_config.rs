use crate::domain::entities::ascii_art::DetailLevel;
use serde::{Deserialize, Serialize};

/// Configuration for ASCII art conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub width: u32,
    pub detail_level: DetailLevel,
    pub contrast_factor: f32,
    pub blur_sigma: f32,
}

impl ConversionConfig {
    /// Create a new conversion configuration
    pub fn new(width: u32, detail_level: DetailLevel) -> Self {
        Self {
            width,
            detail_level,
            contrast_factor: 1.2,
            blur_sigma: 0.5,
        }
    }

    /// Create a configuration with custom parameters
    pub fn with_params(
        width: u32,
        detail_level: DetailLevel,
        contrast_factor: f32,
        blur_sigma: f32,
    ) -> Self {
        Self {
            width,
            detail_level,
            contrast_factor,
            blur_sigma,
        }
    }

    /// Validate the configuration
    pub fn is_valid(&self) -> bool {
        self.width > 0
            && self.width <= 1000 // reasonable upper limit
            && self.contrast_factor > 0.0
            && self.contrast_factor <= 3.0 // reasonable range
            && self.blur_sigma >= 0.0
            && self.blur_sigma <= 5.0 // reasonable range
    }
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self::new(100, DetailLevel::High)
    }
}
