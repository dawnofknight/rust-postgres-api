# Scrapper Engine Monorepo

A monorepo containing a Rust backend API and React TypeScript frontend for web scraping functionality.

## Project Structure

```
.
├── backend/         # Rust backend API
├── frontend/        # React TypeScript frontend
├── docker-compose.yml
└── README.md
```

## Quick Start

### Development

1. **Backend Setup**:
   ```bash
   cd backend
   cp .env.example .env  # Configure your environment variables
   cargo run
   ```

2. **Frontend Setup**:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

### Docker Deployment

Run the entire stack with Docker Compose:

```bash
docker-compose up -d
```

- Backend API: http://localhost:8080
- Frontend: http://localhost:80
- PostgreSQL: localhost:5432

## Services

- **Backend**: Rust-based API for web scraping functionality
- **Frontend**: React TypeScript application to interact with the API
- **Database**: PostgreSQL for data storage

## Development Notes

- Backend and frontend can be developed independently
- Use Docker Compose for integrated testing
- Environment variables are managed separately in each service