use axum::http::StatusCode;

#[tracing::instrument(name = "Health handler")]
pub async fn health() -> StatusCode {
    return StatusCode::OK;
}
