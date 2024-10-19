use crate::api_handler::bestpartner::best_partner_api_handler;
use crate::api_handler::beststats::best_stats_api_handler;
use crate::api_handler::dpsanalysis::dps_analysis_api_handler;
use crate::api_handler::gearcompare::gear_compare_api_handler;
use crate::config::AppState;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Method, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

/// Server router for FFXIV SimHelper service.
/// # Current Supported APIs
/// ---
/// | API route           | Request Type           | Description                                                                                          |
/// |---------------------|------------------------|------------------------------------------------------------------------------------------------------|
/// | /api/v1/simulate    | POST(application/json) | In-depth analysis of player's detailed damage profile. (1000 iterations)                             |
/// | /api/v1/gearcompare | POST(application/json) | Detailed simulation of expected DPS for two different gearsets. (4000 iterations each)               |
/// | /api/v1/bestpartner | POST(application/json) | Finds out which jobs contributes the most to the main player's raidbuffs (1000 iteration/partner)    |
/// | /api/v1/statweights | POST(application/json) | Finds out the expected DPS increase per stat point for each character stats (1000 iteration/stat)    |
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
