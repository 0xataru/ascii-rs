use crate::{
    application::{
        use_cases::{
            ConvertImageToAsciiUseCase, UploadImageUseCase,
            convert_image_to_ascii::ConvertImageRequest,
            upload_image::UploadImageRequest,
        },
    },
    domain::{
        entities::ascii_art::DetailLevel,
        repositories::{AsciiArtRepository, ImageRepository},
        value_objects::ConversionConfig,
    },
    infrastructure::{
        repositories::{InMemoryAsciiArtRepository, InMemoryImageRepository},
        web::{error::WebError, extractors::ImageUpload},
    },
};
use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Application state containing use cases
#[derive(Clone)]
pub struct AppState<IR: ImageRepository, AR: AsciiArtRepository> {
    pub upload_use_case: Arc<UploadImageUseCase<IR>>,
    pub convert_use_case: Arc<ConvertImageToAsciiUseCase<IR, AR>>,
}

/// Request for converting image to ASCII
#[derive(Debug, Deserialize)]
pub struct ConvertToAsciiRequest {
    pub width: Option<u32>,
    pub detail: Option<String>,
    pub contrast: Option<f32>,
    pub blur: Option<f32>,
}

/// Response for image upload
#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub image_id: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub message: String,
}

/// Response for ASCII conversion
#[derive(Debug, Serialize)]
pub struct ConvertResponse {
    pub ascii_art_id: String,
    pub ascii_art: String,
    pub width: u32,
    pub height: u32,
}

/// Upload image endpoint
pub async fn upload_image(
    State(state): State<AppState<InMemoryImageRepository, InMemoryAsciiArtRepository>>,
    upload: ImageUpload,
) -> Result<Json<UploadResponse>, WebError> {
    let request = UploadImageRequest {
        filename: upload.filename,
        content_type: upload.content_type,
        data: upload.data,
    };

    let response = state
        .upload_use_case
        .execute(request)
        .await
        .map_err(|e| WebError::BadRequest(e.to_string()))?;

    Ok(Json(UploadResponse {
        image_id: response.image_id.to_string(),
        format: response.format.to_string(),
        width: response.width,
        height: response.height,
        message: "Image uploaded successfully".to_string(),
    }))
}

/// Convert image to ASCII endpoint
pub async fn convert_to_ascii(
    State(state): State<AppState<InMemoryImageRepository, InMemoryAsciiArtRepository>>,
    Path(image_id): Path<String>,
    Query(params): Query<ConvertToAsciiRequest>,
) -> Result<Json<ConvertResponse>, WebError> {
    let image_id = Uuid::parse_str(&image_id)
        .map_err(|_| WebError::BadRequest("Invalid image ID format".to_string()))?;

    let detail_level = match params.detail.as_deref() {
        Some("low") => DetailLevel::Low,
        Some("high") | None => DetailLevel::High,
        _ => return Err(WebError::BadRequest("Invalid detail level. Use 'low' or 'high'".to_string())),
    };

    let width = params.width.unwrap_or(100);
    let contrast = params.contrast.unwrap_or(1.2);
    let blur = params.blur.unwrap_or(0.5);

    let config = ConversionConfig::with_params(width, detail_level, contrast, blur);

    if !config.is_valid() {
        return Err(WebError::BadRequest("Invalid conversion parameters".to_string()));
    }

    let request = ConvertImageRequest { image_id, config };

    let response = state
        .convert_use_case
        .execute(request)
        .await
        .map_err(|e| match e {
            crate::application::use_cases::convert_image_to_ascii::ConvertImageError::ImageNotFound => {
                WebError::NotFound("Image not found".to_string())
            }
            _ => WebError::InternalServerError(e.to_string()),
        })?;

    Ok(Json(ConvertResponse {
        ascii_art_id: response.ascii_art_id.to_string(),
        ascii_art: response.content,
        width: response.width,
        height: response.height,
    }))
}

/// Health check endpoint
pub async fn health_check() -> Result<Json<serde_json::Value>, WebError> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "ascii-converter",
        "version": env!("CARGO_PKG_VERSION")
    })))
}
