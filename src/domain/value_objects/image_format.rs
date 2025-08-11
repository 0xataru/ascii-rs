use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Gif,
    Webp,
    Bmp,
}

impl ImageFormat {
    /// Get the MIME type for this format
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Png => "image/png",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Webp => "image/webp",
            ImageFormat::Bmp => "image/bmp",
        }
    }

    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Png => "png",
            ImageFormat::Gif => "gif",
            ImageFormat::Webp => "webp",
            ImageFormat::Bmp => "bmp",
        }
    }

    /// Parse format from MIME type
    pub fn from_mime_type(mime_type: &str) -> Option<Self> {
        match mime_type {
            "image/jpeg" | "image/jpg" => Some(ImageFormat::Jpeg),
            "image/png" => Some(ImageFormat::Png),
            "image/gif" => Some(ImageFormat::Gif),
            "image/webp" => Some(ImageFormat::Webp),
            "image/bmp" => Some(ImageFormat::Bmp),
            _ => None,
        }
    }

    /// Parse format from file extension
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
            "png" => Some(ImageFormat::Png),
            "gif" => Some(ImageFormat::Gif),
            "webp" => Some(ImageFormat::Webp),
            "bmp" => Some(ImageFormat::Bmp),
            _ => None,
        }
    }
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageFormat::Jpeg => write!(f, "JPEG"),
            ImageFormat::Png => write!(f, "PNG"),
            ImageFormat::Gif => write!(f, "GIF"),
            ImageFormat::Webp => write!(f, "WebP"),
            ImageFormat::Bmp => write!(f, "BMP"),
        }
    }
}
