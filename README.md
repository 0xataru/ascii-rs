# ASCII Art Converter 🎨

A modern web service for converting images to ASCII art with enhanced image processing algorithms and Clean Architecture, featuring a Rust WebAssembly frontend.

## ✨ Features

- 🖼️ **Image Upload** - Support for major formats (JPEG, PNG, GIF, WebP, BMP)
- 🎯 **Enhanced Algorithms** - Advanced image processing for sharper ASCII results
- ⚙️ **Customizable Parameters** - Width, detail level, contrast, blur adjustment
- 🌐 **REST API** - Full-featured web API with documented endpoints
- 🦀 **Rust WASM Frontend** - Modern web interface built with Yew framework
- 🏗️ **Clean Architecture** - Clear separation of layers with dependency injection
- 🚀 **Performance** - Asynchronous processing with Tokio
- 📊 **Monitoring** - Structured logging and health check endpoints

## 🔧 Tech Stack

### Backend
- **Rust** - Systems programming language
- **Axum** - Modern web framework
- **Tokio** - Async runtime
- **Image** - Image processing library
- **Serde** - Data serialization/deserialization
- **Thiserror** - Error handling
- **Tracing** - Structured logging

### Frontend
- **Rust + WebAssembly** - Frontend compiled to WASM
- **Yew** - Modern web framework for Rust
- **Trunk** - WASM web application bundler
- **Gloo** - Web APIs and utilities for WASM

## 🏗️ Architecture

The project follows Clean Architecture principles:

```
src/
├── domain/              # Business logic
│   ├── entities/        # Domain entities (ImageData, AsciiArt)
│   ├── repositories/    # Repository interfaces
│   └── value_objects/   # Value objects (ConversionConfig, ImageFormat)
├── application/         # Application logic
│   ├── use_cases/       # Use cases (business operations)
│   └── services/        # Domain services
├── infrastructure/     # Infrastructure
│   ├── repositories/   # Repository implementations
│   └── web/           # Web infrastructure
└── presentation/      # Presentation layer
    └── handlers/      # HTTP handlers

frontend/
├── src/
│   └── lib.rs         # Yew WASM application
├── index.html         # HTML template
├── Cargo.toml         # Frontend dependencies
└── Trunk.toml         # Build configuration
```

## 🚀 Quick Start

### Prerequisites

- **Rust** (latest stable version)
- **Trunk** (for frontend building)
- **WASM target** for Rust

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd ascii-converter
   ```

2. **Install Trunk and WASM target**
   ```bash
   cargo install trunk wasm-bindgen-cli
   rustup target add wasm32-unknown-unknown
   ```

3. **Build the frontend**
   ```bash
   cd frontend
   trunk build --release
   cd ..
   ```

4. **Run the server**
   ```bash
   cargo run
   ```

5. **Open in browser**
   ```
   http://localhost:3000
   ```

### Alternative: Development Mode

For development with auto-reload:

```bash
# Terminal 1: Run backend
cargo run

# Terminal 2: Run frontend with hot reload
cd frontend
trunk serve --port 8080
```

Then open `http://localhost:8080` for frontend with hot reload.

## 📚 API Documentation

### Base URL
```
http://localhost:3000
```

### Endpoints

#### Health Check
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "service": "ascii-converter",
  "version": "0.1.0"
}
```

#### Upload Image
```http
POST /api/upload
Content-Type: multipart/form-data
```

**Parameters:**
- `image` - Image file (form field)

**Response:**
```json
{
  "image_id": "uuid-string",
  "format": "PNG",
  "width": 840,
  "height": 859,
  "message": "Image uploaded successfully"
}
```

#### Convert to ASCII
```http
POST /api/convert/{image_id}?width=100&detail=high&contrast=1.2&blur=0.5
```

**Query Parameters:**
- `width` (optional) - ASCII art width in characters (default: 100)
- `detail` (optional) - Detail level: "high" or "low" (default: "high")
- `contrast` (optional) - Contrast factor (0.1-3.0, default: 1.2)
- `blur` (optional) - Blur sigma (0.0-5.0, default: 0.5)

**Response:**
```json
{
  "ascii_art_id": "uuid-string",
  "ascii_art": "ASCII art content...",
  "width": 100,
  "height": 43
}
```

## 🎨 Algorithm Improvements

### 1. Enhanced Filtering
- **Catmull-Rom** filter for resampling instead of Lanczos3
- Better detail preservation during resizing

### 2. Improved Contrast Processing
- Adaptive contrast enhancement before conversion
- Configurable contrast factor

### 3. Gaussian Blur
- Edge-preserving noise reduction
- Configurable sigma parameter

### 4. Adaptive Thresholding
- Histogram equalization for better brightness distribution
- Quantization to specified levels

### 5. Perceptual Mapping
- Gamma correction for better visual perception
- Optimized ASCII character sets

## 🧪 Testing

```bash
# Run backend tests
cargo test

# Check code formatting
cargo fmt

# Run linting
cargo clippy

# Test API endpoints
curl http://localhost:3000/health

# Upload test image
curl -X POST -F "image=@test.jpg" http://localhost:3000/api/upload

# Convert to ASCII
curl -X POST "http://localhost:3000/api/convert/{image_id}?width=80&detail=high"
```

## 🛠️ Development

### Project Structure

- `src/domain/` - Pure business logic, framework-independent
- `src/application/` - Use cases and application services
- `src/infrastructure/` - Interface implementations (repositories, web)
- `src/presentation/` - HTTP handlers and routes
- `frontend/` - Rust WASM web application

### Development Principles

1. **Dependency Inversion** - High-level modules don't depend on low-level ones
2. **Single Responsibility** - Each module has one reason to change
3. **Open/Closed** - Open for extension, closed for modification
4. **Interface Segregation** - Interfaces are client-specific

### Adding New Features

1. **Domain entities** - Add to `src/domain/entities/`
2. **Use cases** - Implement in `src/application/use_cases/`
3. **Repository interfaces** - Define in `src/domain/repositories/`
4. **Repository implementations** - Add to `src/infrastructure/repositories/`
5. **HTTP handlers** - Create in `src/presentation/handlers/`

## 📦 Deployment

### Docker (optional)

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN cd frontend && trunk build --release
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ascii-converter /usr/local/bin/
COPY --from=builder /app/frontend/dist /app/frontend/dist
WORKDIR /app
EXPOSE 3000
CMD ["ascii-converter"]
```

### Environment Variables

- `PORT` - Server port (default: 3000)
- `RUST_LOG` - Logging level (default: info)

## 🤝 Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Commit Convention

This project follows [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes
- `refactor:` - Code refactoring
- `test:` - Adding tests
- `chore:` - Maintenance tasks

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Support

If you have questions or suggestions:

- Create an [Issue](https://github.com/0xataru/ascii-converter/issues)
- Check [Discussions](https://github.com/0xataru/ascii-converter/discussions)

## 🎯 Roadmap

- [ ] Database persistence (PostgreSQL/SQLite)
- [ ] User authentication and saved conversions
- [ ] Batch processing for multiple images
- [ ] Additional output formats (SVG, HTML)
- [ ] Advanced image filters and effects
- [ ] Docker containerization
- [ ] CI/CD pipeline

---

Made with ❤️ and 🦀 by [Ataru](https://github.com/0xataru)