use crate::http::state::HttpServerState;
use axum::extract::State;
use axum::Json;
use common::http_error::HttpError;
use common::http_response::ApiResponse;

pub async fn hello_handler(
    State(_state): State<HttpServerState>,
) -> Result<Json<ApiResponse<String>>, HttpError> {
    Ok(Json(ApiResponse::success("hello world".to_string())))
}
