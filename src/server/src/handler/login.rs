use axum::extract::State;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;

use common::config::get_app_conf;
use common::http_error::HttpError;
use common::http_response::ApiResponse;

use crate::auth::Claims;
use crate::http::state::HttpServerState;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_handler(
    State(_state): State<HttpServerState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<String>>, HttpError> {
    let app_conf = get_app_conf();
    if payload.username != app_conf.server.username || payload.password != app_conf.server.password
    {
        return Err(HttpError::InvalidInput("Invalid credentials".to_string()));
    }

    let claims = Claims::new(payload.username);

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_conf.server.jwt_secret.as_bytes()),
    ) {
        Ok(token) => Ok(Json(ApiResponse::success(token))),
        Err(e) => Err(HttpError::Internal(format!(
            "failed to issue jwt token: {e}"
        ))),
    }
}
