use axum::extract::State;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;

use common::http_response::{error_response, success_response};
use common::tools::get_epoch;

use crate::http::auth::Claims;
use crate::http::state::HttpServerState;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_handler(
    State(state): State<HttpServerState>,
    Json(payload): Json<LoginRequest>,
) -> String {
    if payload.username != "admin" || payload.password != "password" {
        return error_response("Invalid credentials".to_string());
    }

    let now = get_epoch();
    let claims = Claims {
        sub: payload.username,
        exp: now + 24 * 3600, // 24小时后过期
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .unwrap_or_else(|e| panic!("failed to issue jwt token: {}", e));

    success_response(token)
}

pub async fn get_machine_list_handler(State(_state): State<HttpServerState>) -> String {
    success_response("list1, list2")
}
