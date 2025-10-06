use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::models::{ApiError, ApiResponse, CreateUserRequest, UpdateUserRequest, User};

mod crawler;
pub use crawler::crawl_website;
mod social;
pub use social::{
    proxy_tikhub_twitter,
    proxy_tikhub_tiktok,
    proxy_rapidapi_instagram,
    proxy_rapidapi_twitter_v24,
    proxy_rapidapi_generic,
    proxy_tikhub_generic,
};

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "API is running")
}

pub async fn get_users() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<Vec<User>>::error("Users endpoint disabled during Cassandra migration")),
    )
}

pub async fn get_user_by_id(Path(_id): Path<i32>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<User>::error("Users endpoint disabled during Cassandra migration")),
    )
}

pub async fn create_user(Json(_payload): Json<CreateUserRequest>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<User>::error("Users endpoint disabled during Cassandra migration")),
    )
}

pub async fn update_user(Path(_id): Path<i32>, Json(_payload): Json<UpdateUserRequest>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<User>::error("Users endpoint disabled during Cassandra migration")),
    )
}

pub async fn delete_user(Path(_id): Path<i32>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<User>::error("Users endpoint disabled during Cassandra migration")),
    )
}

// Error handling function
fn handle_error<T>(err: ApiError) -> (StatusCode, Json<ApiResponse<T>>) {
    let status = match &err {
        ApiError::NotFound(_) => StatusCode::NOT_FOUND,
        ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
        ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (status, Json(ApiResponse::error(&err.to_string())))
}