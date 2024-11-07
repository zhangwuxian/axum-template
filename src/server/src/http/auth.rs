use crate::http::state::HttpServerState;
use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use common::http_error::HttpError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize,
}

pub async fn auth_middleware(
    State(state): State<HttpServerState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, HttpError> {
    // 从请求头获取 token
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| HttpError::InternalError("Missing authorization header".to_string()))?;

    // 验证 JWT token
    let key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
    let token_data = decode::<Claims>(auth_header, &key, &Validation::default())
        .map_err(|_| HttpError::InternalError("Invalid token".to_string()))?;

    // 可以将用户信息添加到请求扩展中，供后续处理函数使用
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
