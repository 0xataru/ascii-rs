use crate::{
    infrastructure::repositories::{InMemoryAsciiArtRepository, InMemoryImageRepository},
    presentation::handlers::{
        ascii_handlers::{AppState, *},
        health_check,
    },
};
use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

/// Type alias for concrete application state
pub type ConcreteAppState = AppState<InMemoryImageRepository, InMemoryAsciiArtRepository>;

async fn debug_frontend() -> Result<Html<String>, StatusCode> {
    // Check if frontend files exist
    let dist_exists = std::path::Path::new("frontend/dist").exists();
    let index_exists = std::path::Path::new("frontend/dist/index.html").exists();
    
    let debug_info = format!(
        r#"<html><body>
        <h1>Frontend Debug Info</h1>
        <p>frontend/dist exists: {}</p>
        <p>frontend/dist/index.html exists: {}</p>
        <p>Working directory: {:?}</p>
        <p>Files in frontend/dist:</p>
        <ul>
        {}
        </ul>
        <a href="/">Back to main site</a>
        </body></html>"#,
        dist_exists,
        index_exists,
        std::env::current_dir().unwrap_or_default(),
        if dist_exists {
            std::fs::read_dir("frontend/dist")
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .map(|e| format!("<li>{}</li>", e.file_name().to_string_lossy()))
                        .collect::<Vec<_>>()
                        .join("")
                })
                .unwrap_or_else(|_| "<li>Error reading directory</li>".to_string())
        } else {
            "<li>Directory does not exist</li>".to_string()
        }
    );
    
    Ok(Html(debug_info))
}

/// Create application routes
pub fn create_routes() -> Router<ConcreteAppState> {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Debug route
        .route("/debug", get(debug_frontend))
        // API routes
        .route("/api/upload", post(upload_image))
        .route("/api/convert/:image_id", post(convert_to_ascii))
        // CORS layer for web frontend
        .layer(CorsLayer::permissive())
        // Static frontend (built with Trunk into frontend/dist) - MUST BE LAST
        .fallback_service(ServeDir::new("frontend/dist"))
}

/// Create the full application with state
pub fn create_app(state: ConcreteAppState) -> Router {
    create_routes().with_state(state)
}
