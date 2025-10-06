use scylla::{Session, SessionBuilder};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct CassandraState {
    pub session: Arc<Session>,
    pub keyspace: String,
}

pub async fn init_db() -> Result<CassandraState, scylla::transport::errors::NewSessionError> {
    // Read Cassandra configuration from environment
    let contact_points = std::env::var("CASSANDRA_CONTACT_POINTS")
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let keyspace = std::env::var("CASSANDRA_KEYSPACE")
        .unwrap_or_else(|_| "scraper".to_string());

    // Initialize Scylla/Cassandra session with retry/backoff (container may take time to be ready)
    let nodes: Vec<String> = contact_points
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let mut attempt = 0;
    let max_attempts = 40; // ~120s total with 3s delay
    let delay = Duration::from_secs(3);

    let session = loop {
        match SessionBuilder::new().known_nodes(nodes.clone()).build().await {
            Ok(s) => break s,
            Err(e) => {
                attempt += 1;
                eprintln!("Cassandra connection attempt {} failed: {}", attempt, e);
                if attempt >= max_attempts {
                    return Err(e);
                }
                sleep(delay).await;
            }
        }
    };

    // Ensure keyspace exists (SimpleStrategy for local/dev)
    let _ = session
        .query(
            format!(
                "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{'class': 'SimpleStrategy', 'replication_factor': 1}}",
                keyspace
            ),
            &[]
        )
        .await;

    // Ensure table exists for crawl results
    let _ = session
        .query(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.crawl_results (id uuid PRIMARY KEY, payload text, created_at timestamp)",
                keyspace
            ),
            &[]
        )
        .await;

    // Ensure table exists for social proxy results
    let _ = session
        .query(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.social_results (id uuid PRIMARY KEY, source text, request_path text, params text, payload text, created_at timestamp)",
                keyspace
            ),
            &[]
        )
        .await;

    Ok(CassandraState { session: Arc::new(session), keyspace })
}

use scylla::transport::errors::QueryError;
use uuid::Uuid;

pub async fn insert_social_result(
    session: Arc<Session>,
    keyspace: String,
    source: String,
    request_path: String,
    params_json: Option<String>,
    payload_json: String,
) -> Result<(), QueryError> {
    let id = Uuid::new_v4();
    let query = format!(
        "INSERT INTO {}.social_results (id, source, request_path, params, payload, created_at) VALUES (?, ?, ?, ?, ?, toTimestamp(now()))",
        keyspace
    );
    session
        .query(query, (id, source, request_path, params_json, payload_json))
        .await
        .map(|_| ())
}