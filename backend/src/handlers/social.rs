use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::db::{CassandraState, insert_social_result};

const TIKHUB_TWITTER_BASE: &str = "https://api.tikhub.io/api/v1/twitter/web/";
const TIKHUB_TIKTOK_BASE: &str = "https://api.tikhub.io/api/v1/tiktok/web/";
const RAPIDAPI_INSTAGRAM_HOST: &str = "instagram-scraper-api2.p.rapidapi.com";
const RAPIDAPI_TWITTER_V24_HOST: &str = "twitter-v24.p.rapidapi.com";

#[derive(Deserialize)]
pub struct ProxyRequest {
    pub path: String,
    #[serde(default)]
    pub params: Option<Value>,
    #[serde(default)]
    pub method: Option<String>,
}

#[derive(Serialize)]
pub struct ProxyResponse {
    pub status: u16,
    pub data: Value,
}

fn params_to_query(params: &Option<Value>) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(Value::Object(map)) = params {
        for (k, v) in map.iter() {
            let val = match v {
                Value::String(s) => s.clone(),
                _ => v.to_string(),
            };
            query.push((k.clone(), val));
        }
    }
    query
}

// For TikHub Twitter, ensure `keyword` is used (map `q` -> `keyword`) and default `search_type=Top`.
fn tikhub_twitter_query(params: &Option<Value>) -> Vec<(String, String)> {
    let mut query: Vec<(String, String)> = Vec::new();
    let mut has_search_type = false;

    if let Some(Value::Object(map)) = params {
        // Prefer `keyword`, fallback to `q`
        if let Some(v) = map.get("keyword") {
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push(("keyword".to_string(), val));
        } else if let Some(v) = map.get("q") {
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push(("keyword".to_string(), val));
        }

        // Carry `search_type` if provided
        if let Some(v) = map.get("search_type") {
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push(("search_type".to_string(), val));
            has_search_type = true;
        }

        // Include remaining params except `q`, `keyword`, `search_type`
        for (k, v) in map.iter() {
            if k == "q" || k == "keyword" || k == "search_type" { continue; }
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push((k.clone(), val));
        }
    }

    // Default search_type
    if !has_search_type {
        query.push(("search_type".to_string(), "Top".to_string()));
    }

    // If no keyword provided, we still return any extra params; TikHub may error accordingly.
    query
}

// For TikHub TikTok (web), ensure `keyword` is used (map `q` -> `keyword`) and default `count=20`, `offset=0`.
fn tikhub_tiktok_query(params: &Option<Value>) -> Vec<(String, String)> {
    let mut query: Vec<(String, String)> = Vec::new();
    let mut has_count = false;
    let mut has_offset = false;

    if let Some(Value::Object(map)) = params {
        // Prefer `keyword`, fallback to `q`
        if let Some(v) = map.get("keyword") {
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push(("keyword".to_string(), val));
        } else if let Some(v) = map.get("q") {
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push(("keyword".to_string(), val));
        }

        // Carry `count` if provided
        if let Some(v) = map.get("count") {
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push(("count".to_string(), val));
            has_count = true;
        }

        // Carry `offset` if provided
        if let Some(v) = map.get("offset") {
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push(("offset".to_string(), val));
            has_offset = true;
        }

        // Include remaining params except these mapped ones
        for (k, v) in map.iter() {
            if k == "q" || k == "keyword" || k == "count" || k == "offset" { continue; }
            let val = match v { Value::String(s) => s.clone(), _ => v.to_string() };
            query.push((k.clone(), val));
        }
    }

    // Defaults per curl example
    if !has_count { query.push(("count".to_string(), "20".to_string())); }
    if !has_offset { query.push(("offset".to_string(), "0".to_string())); }

    // If no keyword provided, we still return any extra params; TikHub may error accordingly.
    query
}

async fn execute_request(client: &Client, req: reqwest::RequestBuilder) -> Response {
    match req.send().await {
        Ok(resp) => {
            let status_u16 = resp.status().as_u16();
            let ct = resp
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|x| x.to_str().ok())
                .unwrap_or("")
                .to_string();

            if ct.contains("application/json") {
                match resp.json::<Value>().await {
                    Ok(data) => (StatusCode::OK, Json(json!({"status": status_u16, "data": data}))).into_response(),
                    Err(_) => (StatusCode::OK, Json(json!({"status": status_u16, "data": Value::Null}))).into_response(),
                }
            } else {
                match resp.text().await {
                    Ok(text) => (StatusCode::OK, Json(json!({"status": status_u16, "data_text": text}))).into_response(),
                    Err(_) => (StatusCode::OK, Json(json!({"status": status_u16, "data_text": ""}))).into_response(),
                }
            }
        }
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Request failed: {}", err)}))).into_response(),
    }
}

async fn execute_request_capture(client: &Client, req: reqwest::RequestBuilder) -> (Response, Option<String>) {
    match req.send().await {
        Ok(resp) => {
            let status_u16 = resp.status().as_u16();
            let ct = resp
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|x| x.to_str().ok())
                .unwrap_or("")
                .to_string();

            if ct.contains("application/json") {
                match resp.json::<Value>().await {
                    Ok(data) => {
                        let payload_str = serde_json::to_string(&data).ok();
                        ((StatusCode::OK, Json(json!({"status": status_u16, "data": data}))).into_response(), payload_str)
                    }
                    Err(_) => ((StatusCode::OK, Json(json!({"status": status_u16, "data": Value::Null}))).into_response(), None),
                }
            } else {
                match resp.text().await {
                    Ok(text) => ((StatusCode::OK, Json(json!({"status": status_u16, "data_text": text}))).into_response(), Some(text)),
                    Err(_) => ((StatusCode::OK, Json(json!({"status": status_u16, "data_text": ""}))).into_response(), None),
                }
            }
        }
        Err(err) => ((StatusCode::BAD_REQUEST, Json(json!({"error": format!("Request failed: {}", err)}))).into_response(), None),
    }
}

#[derive(Deserialize)]
pub struct TikHubGenericRequest {
    pub service: String, // e.g., "twitter/web" or "tiktok/app/v3"
    pub path: String,
    #[serde(default)]
    pub params: Option<Value>,
    #[serde(default)]
    pub method: Option<String>,
}

pub async fn proxy_tikhub_generic(State(state): State<CassandraState>, Json(body): Json<TikHubGenericRequest>) -> impl IntoResponse {
    let token = match std::env::var("TIKHUB_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing TIKHUB_TOKEN in environment"})),
            )
                .into_response();
        }
    };

    let service = body.service.trim_matches('/');
    let url = format!("https://api.tikhub.io/api/v1/{}/{}", service, body.path.trim_start_matches('/'));
    let client = Client::new();

    let method = body.method.clone().unwrap_or_else(|| "GET".to_string());
    let rb = if method.eq_ignore_ascii_case("GET") {
        client
            .get(&url)
            .query(&params_to_query(&body.params))
            .header("accept", "application/json")
            .header("Authorization", format!("Bearer {}", token))
    } else {
        let params_body = body.params.clone().unwrap_or(Value::Null);
        client
            .post(&url)
            .header("accept", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&params_body)
    };

    let (resp, payload_opt) = execute_request_capture(&client, rb).await;

    if let Some(payload_json) = payload_opt {
        let session = state.session.clone();
        let keyspace = state.keyspace.clone();
        let source = "tikhub_generic".to_string();
        let request_path = body.path.clone();
        let params_json = body.params.as_ref().and_then(|v| serde_json::to_string(v).ok());
        tokio::spawn(async move {
            match insert_social_result(session, keyspace, source, request_path, params_json, payload_json).await {
                Ok(()) => eprintln!("[Cassandra] Inserted social_result: tikhub_generic"),
                Err(e) => eprintln!("[Cassandra] Insert failed: {}", e),
            }
        });
    }

    resp
}

pub async fn proxy_tikhub_twitter(State(state): State<CassandraState>, Json(body): Json<ProxyRequest>) -> impl IntoResponse {
    let token = match std::env::var("TIKHUB_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing TIKHUB_TOKEN in environment"})),
            )
                .into_response();
        }
    };

    let url = format!("{}{}", TIKHUB_TWITTER_BASE, body.path.trim_start_matches('/'));
    let client = Client::new();

    let method = body.method.clone().unwrap_or_else(|| "GET".to_string());
    let rb = if method.eq_ignore_ascii_case("GET") {
        client
            .get(&url)
            .query(&tikhub_twitter_query(&body.params))
            .header("accept", "application/json")
            .header("Authorization", format!("Bearer {}", token))
    } else {
        let params_body = body.params.clone().unwrap_or(Value::Null);
        client
            .post(&url)
            .header("accept", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&params_body)
    };

    let (resp, payload_opt) = execute_request_capture(&client, rb).await;

    if let Some(payload_json) = payload_opt {
        let session = state.session.clone();
        let keyspace = state.keyspace.clone();
        let source = "tikhub_twitter".to_string();
        let request_path = body.path.clone();
        let params_json = body.params.as_ref().and_then(|v| serde_json::to_string(v).ok());
        tokio::spawn(async move {
            match insert_social_result(session, keyspace, source, request_path, params_json, payload_json).await {
                Ok(()) => eprintln!("[Cassandra] Inserted social_result: tikhub_twitter"),
                Err(e) => eprintln!("[Cassandra] Insert failed: {}", e),
            }
        });
    }

    resp
}

pub async fn proxy_tikhub_tiktok(State(state): State<CassandraState>, Json(body): Json<ProxyRequest>) -> impl IntoResponse {
    let token = match std::env::var("TIKHUB_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing TIKHUB_TOKEN in environment"})),
            )
                .into_response();
        }
    };

    let url = format!("{}{}", TIKHUB_TIKTOK_BASE, body.path.trim_start_matches('/'));
    let client = Client::new();

    let method = body.method.clone().unwrap_or_else(|| "GET".to_string());
    let rb = if method.eq_ignore_ascii_case("GET") {
        client
            .get(&url)
            .query(&tikhub_tiktok_query(&body.params))
            .header("accept", "application/json")
            .header("Authorization", format!("Bearer {}", token))
    } else {
        let params_body = body.params.clone().unwrap_or(Value::Null);
        client
            .post(&url)
            .header("accept", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&params_body)
    };

    let (resp, payload_opt) = execute_request_capture(&client, rb).await;

    if let Some(payload_json) = payload_opt {
        let session = state.session.clone();
        let keyspace = state.keyspace.clone();
        let source = "tikhub_tiktok".to_string();
        let request_path = body.path.clone();
        let params_json = body.params.as_ref().and_then(|v| serde_json::to_string(v).ok());
        tokio::spawn(async move {
            match insert_social_result(session, keyspace, source, request_path, params_json, payload_json).await {
                Ok(()) => eprintln!("[Cassandra] Inserted social_result: tikhub_tiktok"),
                Err(e) => eprintln!("[Cassandra] Insert failed: {}", e),
            }
        });
    }

    resp
}

pub async fn proxy_rapidapi_instagram(State(state): State<CassandraState>, Json(body): Json<ProxyRequest>) -> impl IntoResponse {
    let key = match std::env::var("RAPIDAPI_KEY") {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing RAPIDAPI_KEY in environment"})),
            )
                .into_response();
        }
    };

    let url = format!("https://{}/{}", RAPIDAPI_INSTAGRAM_HOST, body.path.trim_start_matches('/'));
    let client = Client::new();
    let method = body.method.clone().unwrap_or_else(|| "GET".to_string());
    let rb = if method.eq_ignore_ascii_case("GET") {
        client
            .get(&url)
            .query(&params_to_query(&body.params))
            .header("accept", "application/json")
            .header("x-rapidapi-key", key.clone())
            .header("x-rapidapi-host", RAPIDAPI_INSTAGRAM_HOST)
    } else {
        let params_body = body.params.clone().unwrap_or(Value::Null);
        client
            .post(&url)
            .header("accept", "application/json")
            .header("x-rapidapi-key", key.clone())
            .header("x-rapidapi-host", RAPIDAPI_INSTAGRAM_HOST)
            .json(&params_body)
    };

    let (resp, payload_opt) = execute_request_capture(&client, rb).await;

    if let Some(payload_json) = payload_opt {
        let session = state.session.clone();
        let keyspace = state.keyspace.clone();
        let source = "rapidapi_instagram".to_string();
        let request_path = body.path.clone();
        let params_json = body.params.as_ref().and_then(|v| serde_json::to_string(v).ok());
        tokio::spawn(async move {
            match insert_social_result(session, keyspace, source, request_path, params_json, payload_json).await {
                Ok(()) => eprintln!("[Cassandra] Inserted social_result: rapidapi_instagram"),
                Err(e) => eprintln!("[Cassandra] Insert failed: {}", e),
            }
        });
    }

    resp
}

pub async fn proxy_rapidapi_twitter_v24(State(state): State<CassandraState>, Json(body): Json<ProxyRequest>) -> impl IntoResponse {
    let key = match std::env::var("RAPIDAPI_KEY") {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing RAPIDAPI_KEY in environment"})),
            )
                .into_response();
        }
    };

    let url = format!("https://{}/{}", RAPIDAPI_TWITTER_V24_HOST, body.path.trim_start_matches('/'));
    let client = Client::new();
    let method = body.method.clone().unwrap_or_else(|| "GET".to_string());
    let rb = if method.eq_ignore_ascii_case("GET") {
        client
            .get(&url)
            .query(&params_to_query(&body.params))
            .header("accept", "application/json")
            .header("x-rapidapi-key", key.clone())
            .header("x-rapidapi-host", RAPIDAPI_TWITTER_V24_HOST)
    } else {
        let params_body = body.params.clone().unwrap_or(Value::Null);
        client
            .post(&url)
            .header("accept", "application/json")
            .header("x-rapidapi-key", key.clone())
            .header("x-rapidapi-host", RAPIDAPI_TWITTER_V24_HOST)
            .json(&params_body)
    };

    let (resp, payload_opt) = execute_request_capture(&client, rb).await;

    if let Some(payload_json) = payload_opt {
        let session = state.session.clone();
        let keyspace = state.keyspace.clone();
        let source = "rapidapi_twitter_v24".to_string();
        let request_path = body.path.clone();
        let params_json = body.params.as_ref().and_then(|v| serde_json::to_string(v).ok());
        tokio::spawn(async move {
            match insert_social_result(session, keyspace, source, request_path, params_json, payload_json).await {
                Ok(()) => eprintln!("[Cassandra] Inserted social_result: rapidapi_twitter_v24"),
                Err(e) => eprintln!("[Cassandra] Insert failed: {}", e),
            }
        });
    }

    resp
}

#[derive(Deserialize)]
pub struct RapidApiGenericRequest {
    pub host: String,
    pub path: String,
    #[serde(default)]
    pub params: Option<Value>,
    #[serde(default)]
    pub method: Option<String>,
}

pub async fn proxy_rapidapi_generic(State(state): State<CassandraState>, Json(body): Json<RapidApiGenericRequest>) -> impl IntoResponse {
    let key = match std::env::var("RAPIDAPI_KEY") {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing RAPIDAPI_KEY in environment"})),
            )
                .into_response();
        }
    };

    let url = format!("https://{}/{}", body.host, body.path.trim_start_matches('/'));
    let client = Client::new();
    let method = body.method.clone().unwrap_or_else(|| "GET".to_string());
    let rb = if method.eq_ignore_ascii_case("GET") {
        client
            .get(&url)
            .query(&params_to_query(&body.params))
            .header("accept", "application/json")
            .header("x-rapidapi-key", key.clone())
            .header("x-rapidapi-host", body.host.clone())
    } else {
        let params_body = body.params.clone().unwrap_or(Value::Null);
        client
            .post(&url)
            .header("accept", "application/json")
            .header("x-rapidapi-key", key.clone())
            .header("x-rapidapi-host", body.host.clone())
            .json(&params_body)
    };

    let (resp, payload_opt) = execute_request_capture(&client, rb).await;

    if let Some(payload_json) = payload_opt {
        let session = state.session.clone();
        let keyspace = state.keyspace.clone();
        let source = format!("rapidapi_{}", body.host);
        let request_path = body.path.clone();
        let params_json = body.params.as_ref().and_then(|v| serde_json::to_string(v).ok());
        tokio::spawn(async move {
            let source_for_log = source.clone();
            match insert_social_result(session, keyspace, source, request_path, params_json, payload_json).await {
                Ok(()) => eprintln!("[Cassandra] Inserted social_result: rapidapi_generic {}", source_for_log),
                Err(e) => eprintln!("[Cassandra] Insert failed: {}", e),
            }
        });
    }

    resp
}