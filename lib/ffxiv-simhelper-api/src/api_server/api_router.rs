use crate::api_handler::bestpartner::best_partner_api_handler;
use crate::api_handler::gearcompare::gear_compare_api_handler;
use crate::api_handler::quicksim::quicksim_api_handler;
use crate::api_handler::statweights::stat_weights_api_handler;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Method, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

pub fn create_ffxiv_simhelper_service_router() -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods(Method::POST)
        .allow_origin(Any)
        .allow_headers(vec![CONTENT_TYPE]);

    Router::new()
        .route("/api/v1/simulate", post(quicksim_api_handler))
        .route("/api/v1/gearcompare", post(gear_compare_api_handler))
        .route("/api/v1/statweights", post(stat_weights_api_handler))
        .route("/api/v1/bestpartner", post(best_partner_api_handler))
        .route("/api/v1/healthcheck", get(|| async { StatusCode::OK }))
        .layer(cors_layer)
}
