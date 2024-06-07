use crate::api_handler::simulate::simulate_api_handler;
use axum::routing::post;
use axum::Router;
use ffxiv_simbot_engine::engine::Engine;

pub fn create_ffxiv_simbot_service_router() -> Router {
    Router::new().route("/api/v1/simulate", post(simulate_api_handler))
}
