use ascii_converter::{
    application::{
        services::AsciiConversionService,
        use_cases::{ConvertImageToAsciiUseCase, UploadImageUseCase},
    },
    infrastructure::{
        repositories::{InMemoryAsciiArtRepository, InMemoryImageRepository},
        web::create_app,
    },
    presentation::handlers::ascii_handlers::AppState,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Create repositories
    let image_repo = Arc::new(InMemoryImageRepository::new());
    let ascii_art_repo = Arc::new(InMemoryAsciiArtRepository::new());

    // Create services
    let conversion_service = Arc::new(AsciiConversionService::new());

    // Create use cases
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
    let upload_use_case = Arc::new(UploadImageUseCase::new(
        Arc::clone(&image_repo),
        MAX_FILE_SIZE,
    ));

    let convert_use_case = Arc::new(ConvertImageToAsciiUseCase::new(
        Arc::clone(&image_repo),
        Arc::clone(&ascii_art_repo),
        conversion_service,
    ));

    // Create application state
    let state = AppState {
        upload_use_case,
        convert_use_case,
    };

    // Create application
    let app = create_app(state);

    // Get port from environment variable or default to 3000
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Start server
    let listener = TcpListener::bind(&addr).await?;
    info!("🚀 ASCII Converter API server starting on http://{}", addr);

    info!("Available endpoints:");
    info!("  GET  /health                    - Health check");
    info!(
        "  POST /api/upload                - Upload image (multipart/form-data with 'image' field)"
    );
    info!("  POST /api/convert/:image_id     - Convert image to ASCII art");
    info!("    Query parameters:");
    info!("      ?width=100                  - ASCII width (default: 100)");
    info!("      ?detail=high|low            - Detail level (default: high)");
    info!("      ?contrast=1.2               - Contrast factor (default: 1.2)");
    info!("      ?blur=0.5                   - Blur sigma (default: 0.5)");

    axum::serve(listener, app).await?;

    Ok(())
}
