use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HttpError {
    #[error("Not found for {0}")]
    NotFound(String),
    #[error("Internal Error: {0}")]
    InternalError(String),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            HttpError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            HttpError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(json!({
            "error": format!("invalid path {}", error_message),
        }));

        (status, body).into_response()
    }
}
