use crate::http::state::HttpServerState;
use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use common::config::get_app_conf;
use common::http_error::HttpError;
use common::tools::get_epoch;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static AUTH_HEADER_KEY: &str = "Authorization";
static AUTH_HEADER_PREFIX: &str = "Bearer ";

static JWT_VALIDATION: OnceLock<Validation> = OnceLock::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    // user input
    pub sub: String, // Optional. Subject (whom token refers to)
    pub aud: String, // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)

    // code fill
    pub iat: usize,  // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub nbf: usize,  // Optional. Not Before (as UTC timestamp)
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let now_timestamp = get_epoch();
        let app_conf = get_app_conf();
        Self {
            sub,
            aud: app_conf.server.jwt_audience.to_owned(),
            exp: now_timestamp + app_conf.server.jwt_expire,
            iat: now_timestamp,
            iss: app_conf.server.jwt_issuer.to_owned(),
            nbf: now_timestamp,
        }
    }
}

pub async fn auth_middleware(
    State(_state): State<HttpServerState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, HttpError> {
    // get token from headers
    let auth_header = req
        .headers()
        .get(AUTH_HEADER_KEY)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix(AUTH_HEADER_PREFIX))
        .ok_or_else(|| HttpError::Internal("Missing authorization header".to_string()))?;

    // check JWT token
    let app_conf = get_app_conf();
    let key = DecodingKey::from_secret(app_conf.server.jwt_secret.as_bytes());
    let validation = JWT_VALIDATION.get_or_init(|| {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&app_conf.server.jwt_audience]);
        validation.set_issuer(&[&app_conf.server.jwt_issuer]);
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.validate_aud = true;
        validation.leeway = 5;
        validation
    });

    let token_data = decode::<Claims>(auth_header, &key, validation)
        .map_err(|_| HttpError::Internal("Invalid token".to_string()))?;

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
