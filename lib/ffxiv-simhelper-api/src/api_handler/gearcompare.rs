use crate::api_handler::quicksim::quicksim;
use crate::errors::Result;
use crate::request::gear_compare_api_request::GearCompareApiRequest;
use crate::response::stat_compare_api_response::GearCompareApiResponse;
use axum::Json;

pub(crate) async fn gear_compare_api_handler(
    Json(request): Json<GearCompareApiRequest>,
) -> Result<Json<GearCompareApiResponse>> {
    let simulation_response1 = quicksim(request.gear1_request)?;
    let simulation_response2 = quicksim(request.gear2_request)?;

    Ok(Json(GearCompareApiResponse::from((
        simulation_response1,
        simulation_response2,
    ))))
}
