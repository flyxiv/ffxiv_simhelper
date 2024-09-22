use axum::response::IntoResponse;
use log::info;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, FfxivSimbotServiceError>;

#[derive(Error, Debug)]
pub enum FfxivSimbotServiceError {
    #[error("Axum Error: {0}")]
    AxumError(#[from] axum::Error),
    #[error("Invalid Request: {0}")]
    InvalidRequest(String),
    #[error("Invalid Response: {0}")]
    InvalidResponse(String),
    #[error("Invalid Job String: {0}")]
    InvalidJobString(String),
}

impl IntoResponse for FfxivSimbotServiceError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        info!("{:?}", self);
        axum::http::Response::builder()
            .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from(format!("{:?}", self)))
            .unwrap()
    }
}
