use crate::api_handler::bestpartner::best_partner_api_handler;
use crate::api_handler::dpsanalysis::dps_analysis_api_handler;
use crate::api_handler::gearcompare::gear_compare_api_handler;
use crate::api_handler::statweights::best_stats_api_handler;
use crate::config::AppState;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Method, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

/// Server router for FFXIV SimHelper service.
pub fn create_ffxiv_simhelper_service_router(app_state: AppState) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods(Method::POST)
        .allow_origin(Any)
        .allow_headers(vec![CONTENT_TYPE]);

    Router::new()
        .route("/api/v1/bestpartner", post(best_partner_api_handler))
        .route("/api/v1/beststats", post(best_stats_api_handler))
        .with_state(app_state)
        .route("/api/v1/simulate", post(dps_analysis_api_handler))
        .route("/api/v1/gearcompare", post(gear_compare_api_handler))
        .route("/api/v1/healthcheck", get(|| async { StatusCode::OK }))
        .layer(cors_layer)
}
