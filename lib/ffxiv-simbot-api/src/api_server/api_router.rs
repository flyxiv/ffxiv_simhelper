use crate::api_handler::simulate::simulate_api_handler;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Method, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

pub fn create_ffxiv_simbot_service_router() -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods(Method::POST)
        .allow_origin(Any)
        .allow_headers(vec![CONTENT_TYPE]);

    Router::new()
        .route("/api/v1/simulate", post(simulate_api_handler))
        //        .route("/api/v1/statcompare", post(stat_compare_api_handler))
        .route("/api/v1/healthcheck", get(|| async { StatusCode::OK }))
        .layer(cors_layer)
}
