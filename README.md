# Web Crawler Engine Monorepo

A comprehensive monorepo containing a Rust backend API, React TypeScript frontend, and integrated Spider crawler for advanced web scraping and content analysis functionality.

## Project Architecture

This project consists of three main components:

```
.
├── backend/         # Rust backend API with crawler integration
├── frontend/        # React TypeScript frontend with modern UI
├── spider/          # Advanced Spider web crawler library
├── docker-compose.yml
└── README.md
```

## Features

### 🕷️ Advanced Web Crawling
- **Multi-domain crawling** with configurable depth and page limits
- **Keyword-based content filtering** and matching
- **Date range filtering** for time-sensitive content
- **Real-time crawling progress** and results
- **Robust error handling** and retry mechanisms

### 🚀 Modern Tech Stack
- **Backend**: Rust with Axum framework, PostgreSQL integration
- **Frontend**: React 18 + TypeScript + Vite for fast development
- **Crawler**: Spider library with advanced crawling capabilities
- **Database**: PostgreSQL for persistent data storage
- **API**: RESTful endpoints with JSON responses

### 🎯 Current Functionality
- Web crawler API endpoint (`/crawl`) with keyword matching
- Clean, responsive frontend interface
- Real-time crawling results display
- Configurable crawling parameters (pages, dates, keywords)

## Quick Start

### Development Setup

1. **Backend Setup**:
   ```bash
   cd backend
   cp .env.example .env  # Configure your environment variables
   cargo run
   ```
   Server runs on: `http://localhost:8081`

2. **Frontend Setup**:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```
   Frontend runs on: `http://localhost:5173` (or `http://localhost:5174` if 5173 is busy)

### Testing the Crawler

Test the crawler API directly:

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

Run the entire stack with Docker Compose:

```bash
docker-compose up -d
```

- Backend API: http://localhost:8080
- Frontend: http://localhost:80
- PostgreSQL: localhost:5432

## API Endpoints

### POST `/crawl`
Crawl a website with keyword filtering.

**Request Body:**
```json
{
  "url": "https://example.com",
  "keywords": ["keyword1", "keyword2"],
  "max_pages": 5,
  "date_from": null,
  "date_to": null
}
```

**Response:**
```json
{
  "url": "https://example.com",
  "pages_crawled": 5,
  "total_matches": 0,
  "crawl_duration_seconds": 2.5,
  "matches": []
}
```

## Services

- **Backend**: Rust-based API with integrated Spider crawler
- **Frontend**: React TypeScript SPA with modern UI components
- **Database**: PostgreSQL for crawl results and metadata storage
- **Crawler**: Spider library for high-performance web crawling

## Recent Updates

### ✅ Completed Features
- **Crawler Integration**: Successfully integrated Spider crawler with backend
- **API Refactoring**: Streamlined API to match actual crawler capabilities
- **Frontend Revamp**: Updated UI to align with working API structure
- **Testing**: Comprehensive testing with real websites (NewsNow, Example.com)
- **Error Handling**: Robust error handling for various crawling scenarios

### 🔧 Technical Improvements
- Simplified `CrawlRequest` interface to essential fields only
- Removed unused form fields from frontend (max_depth, follow_pagination, etc.)
- Updated API client to match exact backend expectations
- Improved form validation and user experience
- Added real-world examples and placeholders

## Development Notes

- Backend and frontend can be developed independently
- Use Docker Compose for integrated testing
- Environment variables are managed separately in each service
- The Spider crawler is integrated directly into the backend
- Frontend automatically adapts to different ports if default is busy
- All API responses include timing and metadata for performance monitoring