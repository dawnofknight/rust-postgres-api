# Rust Web Crawler Backend

Backend service built with Rust, Axum, and integrated Spider crawler. Provides a high-performance web crawling API with keyword matching, content analysis, and real-time results.

## 🚀 Features

### Core Functionality
- **Advanced Web Crawler**: Integrated Spider library for high-performance crawling
- **Keyword Matching**: Real-time content filtering with contextual snippets
- **Flexible Configuration**: Configurable page limits, date filtering, and crawl parameters
- **Real-time Results**: Immediate response with crawl statistics and matches
- **Robust Error Handling**: Comprehensive error handling for various crawling scenarios

### Technical Stack
- **Framework**: Rust with Axum web framework
- **Crawler**: Spider library integration
- **Database**: PostgreSQL with SQLx (ready for future persistence)
- **API**: RESTful JSON endpoints
- **CORS**: Full CORS support for frontend integration

## 🏗️ Project Structure

```
backend/
├── src/
│   ├── config/             # Application configuration
│   ├── crawler/            # Spider crawler integration and logic
│   │   └── mod.rs          # CrawlRequest, CrawlResult, error handling
│   ├── db/                 # Database connection utilities
│   ├── handlers/           # Request handlers
│   │   └── crawler.rs      # Main crawler endpoint handler
│   ├── models/             # Data models and schemas
│   ├── routes/             # API route definitions
│   │   └── mod.rs          # Route configuration
│   ├── kafka.rs            # Kafka integration (optional)
│   └── main.rs             # Application entry point
├── migrations/             # Database migrations
├── .env.example            # Example environment variables
├── Dockerfile              # Docker configuration
├── docker-compose.yml      # Docker Compose setup
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

## 🛠️ Prerequisites

- **Rust** (latest stable version)
- **PostgreSQL** (for future data persistence)
- **Docker & Docker Compose** (for containerized deployment)

## ⚡ Quick Start

### Local Development

1. **Clone and Setup**:
   ```bash
   cd backend
   cp .env.example .env  # Configure environment variables
   ```

2. **Run the Server**:
   ```bash
   cargo run
   ```
   
   Server starts on: `http://localhost:8081`

3. **Test the Crawler**:
   ```bash
   curl -X POST http://localhost:8081/crawl \
     -H "Content-Type: application/json" \
     -d '{
       "url": "https://example.com",
       "keywords": ["example", "test"],
       "max_pages": 5,
       "date_from": null,
       "date_to": null
     }'
   ```

### Docker Deployment

```bash
# Build and start containers
docker compose up -d

# View logs
docker compose logs -f app

# Stop containers
docker compose down
```

## 📡 API Endpoints

### Core Endpoints

| Method | Endpoint | Description | Status |
|--------|----------|-------------|---------|
| GET    | `/health` | Health check | ✅ Active |
| POST   | `/crawl` | Web crawler with keyword matching | ✅ Active |

### Legacy Endpoints (Disabled)
| Method | Endpoint | Description | Status |
|--------|----------|-------------|---------|
| GET/POST/PUT/DELETE | `/users/*` | User CRUD operations | ❌ Disabled |
| POST   | `/social/*` | Social media proxies | ❌ Disabled |

## 🕷️ Crawler API

### POST `/crawl`

Crawl websites with keyword filtering and content analysis.

#### Request Format

```json
{
  "url": "https://example.com",
  "keywords": ["keyword1", "keyword2"],
  "max_pages": 5,
  "date_from": null,
  "date_to": null
}
```

#### Request Parameters

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | string | ✅ | Target website URL to crawl |
| `keywords` | array[string] | ✅ | Keywords to search for in content |
| `max_pages` | integer | ❌ | Maximum pages to crawl (default: 10) |
| `date_from` | string/null | ❌ | Start date filter (ISO format or null) |
| `date_to` | string/null | ❌ | End date filter (ISO format or null) |

#### Response Format

```json
{
  "url": "https://example.com",
  "pages_crawled": 5,
  "total_matches": 3,
  "crawl_duration_seconds": 2.5,
  "matches": [
    {
      "url": "https://example.com/page1",
      "title": "Example Page",
      "keyword": "example",
      "context": "This is an example of content...",
      "match_count": 2
    }
  ]
}
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `url` | string | Original crawl URL |
| `pages_crawled` | integer | Number of pages successfully crawled |
| `total_matches` | integer | Total keyword matches found |
| `crawl_duration_seconds` | float | Time taken to complete crawl |
| `matches` | array | Array of keyword matches with context |

### Example Requests

#### Basic Crawl
```bash
curl -X POST http://localhost:8081/crawl \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "keywords": ["rust", "programming"],
    "max_pages": 3
  }'
```

#### News Site Crawl
```bash
curl -X POST http://localhost:8081/crawl \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://www.newsnow.co.uk/h/",
    "keywords": ["bangsamoro"],
    "max_pages": 5,
    "date_from": null,
    "date_to": null
  }'
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file based on `.env.example`:

```bash
# Server Configuration
SERVER_PORT=8081

# Database (for future use)
DATABASE_URL=postgresql://user:password@localhost/crawler_db

# Optional: Kafka Integration
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC_CRAWL=crawl_results

# Optional: Cassandra Integration
CASSANDRA_CONTACT_POINTS=127.0.0.1
CASSANDRA_KEYSPACE=scraper
```

### Key Configuration Notes

- **Port**: Server runs on port `8081` by default
- **Database**: PostgreSQL connection ready for future persistence features
- **Kafka**: Optional integration for crawl result streaming
- **CORS**: Configured for frontend integration (all origins in development)

## 🚀 Recent Updates & Improvements

### ✅ Completed Features
- **Spider Integration**: Successfully integrated Spider crawler library
- **Streamlined API**: Simplified `/crawl` endpoint with essential parameters only
- **Real-time Processing**: Immediate crawl results with timing metrics
- **Error Handling**: Comprehensive error handling for network issues, parsing errors
- **Frontend Compatibility**: API designed to work seamlessly with React frontend

### 🔧 Technical Improvements
- **Simplified Request Structure**: Removed unused parameters (max_depth, follow_pagination, etc.)
- **Better Response Format**: Structured JSON responses with metadata
- **Performance Metrics**: Built-in timing and statistics
- **Robust Error Messages**: Clear error responses for debugging

### 🧪 Tested Scenarios
- ✅ Basic website crawling (example.com)
- ✅ News site crawling (NewsNow)
- ✅ Keyword matching and filtering
- ✅ Page limit enforcement
- ✅ Error handling for invalid URLs
- ✅ Frontend-backend integration

## 🐳 Docker Support

### Multi-stage Build
- Optimized Docker image with multi-stage build
- Minimal runtime image for production deployment
- Automatic dependency caching for faster builds

### Docker Compose
- Complete stack with backend + database
- Automatic service discovery and networking
- Persistent data volumes
- Health checks and restart policies

## 🔮 Future Enhancements

### Planned Features
- **Database Persistence**: Store crawl results in PostgreSQL
- **User Authentication**: API key-based access control
- **Rate Limiting**: Request throttling and quota management
- **Scheduled Crawls**: Cron-like scheduled crawling
- **Advanced Filtering**: Content type, language, and domain filtering
- **Export Options**: CSV, JSON, and XML export formats

### Performance Optimizations
- **Caching Layer**: Redis integration for frequently crawled sites
- **Parallel Processing**: Multi-threaded crawling for better performance
- **Resource Management**: Memory and CPU usage optimization
- **Monitoring**: Metrics and logging integration

## 🤝 Development

### Adding New Features

1. **Models**: Define data structures in `src/models/`
2. **Handlers**: Implement request handlers in `src/handlers/`
3. **Routes**: Register new routes in `src/routes/mod.rs`
4. **Testing**: Add tests for new functionality

### Code Structure
- **Modular Design**: Clear separation of concerns
- **Error Handling**: Consistent error types and responses
- **Configuration**: Environment-based configuration
- **Documentation**: Comprehensive inline documentation

## 📝 License

MIT License - see LICENSE file for details.