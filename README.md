# ASCII Art Converter 🎨

Современный веб-сервис для конвертации изображений в ASCII-арт с улучшенными алгоритмами обработки изображений и архитектурой Clean Architecture.

## ✨ Возможности

- 🖼️ **Загрузка изображений** - Поддержка основных форматов (JPEG, PNG, GIF, WebP, BMP)
- 🎯 **Улучшенные алгоритмы** - Усовершенствованная обработка изображений для более четких результатов
- ⚙️ **Настраиваемые параметры** - Ширина, уровень детализации, контрастность, размытие
- 🌐 **REST API** - Полнофункциональный веб-API с документированными endpoints
- 🏗️ **Clean Architecture** - Четкое разделение слоев и dependency injection
- 🚀 **Производительность** - Асинхронная обработка с использованием Tokio
- 📊 **Мониторинг** - Логирование и health check endpoints

## 🔧 Технический стек

- **Rust** - Системный язык программирования
- **Axum** - Современный веб-фреймворк
- **Tokio** - Асинхронная среда выполнения
- **Image** - Библиотека для обработки изображений
- **Serde** - Сериализация/десериализация данных
- **Thiserror** - Обработка ошибок
- **Tracing** - Структурированное логирование

## 🏗️ Архитектура

Проект следует принципам Clean Architecture:

```
src/
├── domain/              # Бизнес-логика
│   ├── entities/        # Сущности (ImageData, AsciiArt)
│   ├── repositories/    # Интерфейсы репозиториев
│   └── value_objects/   # Объекты-значения (ConversionConfig, ImageFormat)
├── application/         # Логика приложения
│   ├── use_cases/       # Сценарии использования
│   └── services/        # Доменные сервисы
├── infrastructure/     # Инфраструктура
│   ├── repositories/   # Реализации репозиториев
│   └── web/           # Веб-инфраструктура
└── presentation/      # Слой представления
    └── handlers/      # HTTP обработчики
```

## 🚀 Быстрый старт

### Запуск сервера

```bash
# Клонировать репозиторий
git clone <repository-url>
cd ascii-converter

# Запустить сервер
cargo run

# Сервер будет доступен на http://localhost:3000
```

### Использование веб-интерфейса

1. Откройте `index.html` в браузере
2. Выберите изображение для загрузки
3. Настройте параметры конвертации
4. Нажмите "Convert to ASCII"
5. Скачайте результат

## 📚 API Documentation

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
- `image` - Файл изображения (form field)

**Response:**
```json
{
  "image_id": "uuid-string",
  "format": "JPEG",
  "width": 1920,
  "height": 1080,
  "message": "Image uploaded successfully"
}
```

#### Convert to ASCII
```http
POST /api/convert/{image_id}?width=100&detail=high&contrast=1.2&blur=0.5
```

**Query Parameters:**
- `width` (optional) - Ширина ASCII арта в символах (default: 100)
- `detail` (optional) - Уровень детализации: "high" или "low" (default: "high")
- `contrast` (optional) - Фактор контрастности (0.1-3.0, default: 1.2)
- `blur` (optional) - Размытие (0.0-5.0, default: 0.5)

**Response:**
```json
{
  "ascii_art_id": "uuid-string",
  "ascii_art": "ASCII art content...",
  "width": 100,
  "height": 43
}
```

## 🎨 Алгоритмические улучшения

### 1. Усовершенствованная фильтрация
- **Catmull-Rom** фильтр для ресэмплинга вместо Lanczos3
- Лучшее сохранение деталей при изменении размера

### 2. Улучшенная обработка контраста
- Адаптивное увеличение контраста перед конвертацией
- Настраиваемый фактор контрастности

### 3. Гауссово размытие
- Снижение шума с сохранением краев
- Настраиваемый параметр sigma

### 4. Адаптивная пороговая обработка
- Эквализация гистограммы для улучшения распределения яркости
- Квантизация на заданное количество уровней

### 5. Перцептивное маппирование
- Гамма-коррекция для лучшего восприятия
- Оптимизированный набор ASCII символов

## 🧪 Тестирование

```bash
# Запуск тестов
cargo test

# Запуск с подробным выводом
cargo test -- --nocapture

# Проверка форматирования
cargo fmt

# Линтинг
cargo clippy
```

## 📝 Примеры использования

### Curl запросы

```bash
# Health check
curl http://localhost:3000/health

# Upload image
curl -X POST \
  -F "image=@path/to/your/image.jpg" \
  http://localhost:3000/api/upload

# Convert to ASCII
curl -X POST \
  "http://localhost:3000/api/convert/{image_id}?width=80&detail=high&contrast=1.5"
```

### Программное использование

```rust
use ascii_converter::{
    application::services::AsciiConversionService,
    domain::{entities::ImageData, value_objects::ConversionConfig},
};

// Создание сервиса конвертации
let service = AsciiConversionService::new();

// Загрузка изображения
let image_data = ImageData::new(/* ... */);

// Конфигурация конвертации
let config = ConversionConfig::new(100, DetailLevel::High);

// Конвертация
let ascii_art = service.convert_to_ascii(&image_data, &config).await?;
```

## 🛠️ Разработка

### Структура проекта

- `src/domain/` - Чистая бизнес-логика, не зависит от внешних фреймворков
- `src/application/` - Сценарии использования и сервисы приложения
- `src/infrastructure/` - Реализации интерфейсов (репозитории, веб)
- `src/presentation/` - HTTP обработчики и маршруты

### Принципы

1. **Dependency Inversion** - Высокоуровневые модули не зависят от низкоуровневых
2. **Single Responsibility** - Каждый модуль имеет одну причину для изменения
3. **Open/Closed** - Открыт для расширения, закрыт для модификации
4. **Interface Segregation** - Интерфейсы специфичны для клиентов

## 📄 Лицензия

Этот проект находится под лицензией MIT. См. файл [LICENSE](LICENSE) для получения подробной информации.

## 🤝 Вклад в развитие

Мы приветствуем вклад в развитие проекта! Пожалуйста:

1. Форкните проект
2. Создайте ветку для новой функции (`git checkout -b feature/amazing-feature`)
3. Зафиксируйте изменения (`git commit -m 'feat: add amazing feature'`)
4. Отправьте ветку (`git push origin feature/amazing-feature`)
5. Откройте Pull Request

## 📞 Поддержка

Если у вас есть вопросы или предложения, пожалуйста:

- Создайте [Issue](https://github.com/your-repo/ascii-converter/issues)
- Посмотрите [Discussions](https://github.com/your-repo/ascii-converter/discussions)
