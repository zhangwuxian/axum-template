use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::error;

use crate::http_response::ApiResponse;

#[derive(Debug)]
pub enum HttpError {
    NotFound(String),
    InvalidInput(String),
    InvalidAuth(String),
    Internal(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::NotFound(msg) => write!(f, "Not Found: {msg}"),
            HttpError::InvalidInput(msg) => write!(f, "Invalid Input: {msg}"),
            HttpError::InvalidAuth(msg) => write!(f, "Invalid Authorization: {msg}"),
            HttpError::Internal(msg) => write!(f, "Internal Error: {msg}"),
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, code, message) = log_error(self);

        let body = Json(ApiResponse::<()>::error(code, message));
        (status, body).into_response()
    }
}

fn log_error(error: HttpError) -> (StatusCode, i32, String) {
    error!("error occurred: {:?}", error);
    match error {
        HttpError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg),
        HttpError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, 400, msg),
        HttpError::InvalidAuth(msg) => (StatusCode::BAD_REQUEST, 403, msg),
        HttpError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
    }
}
