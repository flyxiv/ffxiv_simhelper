use crate::api_handler::quicksim::quicksim;
use crate::errors::Result;
use crate::request::gear_compare_api_request::GearCompareApiRequest;
use crate::response::stat_compare_api_response::GearCompareApiResponse;
use axum::Json;

const NUMBER_OF_ITERATIONS_PER_REQUEST_GEAR_COMPARE: usize = 8;

pub(crate) async fn gear_compare_api_handler(
    Json(request): Json<GearCompareApiRequest>,
) -> Result<Json<GearCompareApiResponse>> {
    let simulation_response1 = quicksim(
        request.gear1_request,
        NUMBER_OF_ITERATIONS_PER_REQUEST_GEAR_COMPARE,
    )?;
    let simulation_response2 = quicksim(
        request.gear2_request,
        NUMBER_OF_ITERATIONS_PER_REQUEST_GEAR_COMPARE,
    )?;

    Ok(Json(GearCompareApiResponse::from((
        simulation_response1,
        simulation_response2,
    ))))
}
