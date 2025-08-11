use axum::{
    async_trait,
    extract::{FromRequest, Multipart, Request},
};
use crate::infrastructure::web::error::WebError;

/// Multipart form data for image upload
pub struct ImageUpload {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[async_trait]
impl<S> FromRequest<S> for ImageUpload
where
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state)
            .await
            .map_err(|_| WebError::BadRequest("Invalid multipart data".to_string()))?;

        let mut filename = None;
        let mut content_type = None;
        let mut data = None;

        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|_| WebError::BadRequest("Error reading multipart field".to_string()))?
        {
            let field_name = field.name().unwrap_or("").to_string();

            match field_name.as_str() {
                "image" => {
                    filename = field.file_name().map(|s| s.to_string());
                    content_type = field.content_type().map(|s| s.to_string());
                    data = Some(field.bytes().await.map_err(|_| {
                        WebError::BadRequest("Error reading image data".to_string())
                    })?);
                }
                _ => {
                    // Skip unknown fields
                }
            }
        }

        let filename = filename.ok_or_else(|| {
            WebError::BadRequest("Missing filename in image field".to_string())
        })?;

        let content_type = content_type.ok_or_else(|| {
            WebError::BadRequest("Missing content type in image field".to_string())
        })?;

        let data = data
            .ok_or_else(|| WebError::BadRequest("Missing image data".to_string()))?
            .to_vec();

        // Validate content type
        if !content_type.starts_with("image/") {
            return Err(WebError::UnsupportedMediaType);
        }

        // Validate file size (10MB limit)
        const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
        if data.len() > MAX_FILE_SIZE {
            return Err(WebError::PayloadTooLarge);
        }

        Ok(ImageUpload {
            filename,
            content_type,
            data,
        })
    }
}
