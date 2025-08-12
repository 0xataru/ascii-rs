use crate::{
    infrastructure::repositories::{InMemoryAsciiArtRepository, InMemoryImageRepository},
    presentation::handlers::{
        ascii_handlers::{AppState, *},
        health_check,
    },
};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

/// Type alias for concrete application state
pub type ConcreteAppState = AppState<InMemoryImageRepository, InMemoryAsciiArtRepository>;

/// Create application routes
pub fn create_routes() -> Router<ConcreteAppState> {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // API routes
        .route("/api/upload", post(upload_image))
        .route("/api/convert/:image_id", post(convert_to_ascii))
        // Static frontend (built with Trunk into frontend/dist)
        .nest_service(
            "/",
            ServeDir::new("frontend/dist").append_index_html_on_directories(true),
        )
        // CORS layer for web frontend
        .layer(CorsLayer::permissive())
}

/// Create the full application with state
pub fn create_app(state: ConcreteAppState) -> Router {
    create_routes().with_state(state)
}
