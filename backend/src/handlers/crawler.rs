use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;
use crate::db::CassandraState;

use crate::crawler::{CrawlRequest, CrawlerError};

pub async fn crawl_website(
    State(state): State<CassandraState>,
    Json(request): Json<CrawlRequest>,
) -> impl IntoResponse {
    match crate::crawler::crawl_website(&request).await {
        Ok(result) => {
            // Serialize and store the result directly into Cassandra
            match serde_json::to_string(&result) {
                Ok(payload) => {
                    let id = Uuid::new_v4();
                    let query = format!(
                        "INSERT INTO {}.crawl_results (id, payload, created_at) VALUES (?, ?, toTimestamp(now()))",
                        state.keyspace
                    );
                    if let Err(e) = state.session.query(query, (id, payload)).await {
                        eprintln!("Failed to insert crawl result into Cassandra: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": "Failed to persist crawl result"})),
                        )
                            .into_response();
                    }
                    (StatusCode::OK, Json(result)).into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Serialization error: {}", e)})),
                )
                    .into_response(),
            }
        },
        Err(err) => {
            let (status, error_message) = match &err {
                CrawlerError::RequestError(e) => (StatusCode::BAD_REQUEST, format!("Request error: {}", e)),
                CrawlerError::UrlError(e) => (StatusCode::BAD_REQUEST, format!("Invalid URL: {}", e)),
                CrawlerError::SelectorError(e) => (StatusCode::BAD_REQUEST, format!("Selector error: {}", e)),
                CrawlerError::TimeoutError => (StatusCode::OK, "Crawling exceeded the time limit".to_string()),
                CrawlerError::DateParsingError(e) => (StatusCode::BAD_REQUEST, format!("Date parsing error: {}", e)),
                CrawlerError::SpiderError(e) => (StatusCode::BAD_REQUEST, format!("Spider error: {}", e)),
                CrawlerError::Other(e) => (StatusCode::BAD_REQUEST, format!("Other error: {}", e)),
            };
            
            (
                status,
                Json(json!({
                    "error": error_message
                })),
            )
                .into_response()
        }
    }
}