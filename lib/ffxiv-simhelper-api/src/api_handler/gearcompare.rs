use crate::api_handler::dpsanalysis::dps_analysis;
use crate::errors::Result;
use crate::request::gear_compare_api_request::GearCompareApiRequest;
use crate::response::stat_compare_api_response::GearCompareApiResponse;
use axum::Json;

const NUMBER_OF_ITERATIONS_PER_REQUEST_GEAR_COMPARE: usize = 8;

/// GearCompare API Request Handler
///
/// # Description
/// 1) Compares the DPS of two different gearsets in PDPS/RDPS/EDPS.
/// 2) Most accurate simulation of FFXIV SimHelper: does 4000 iterations for each gearset.
pub async fn gear_compare_api_handler(
    Json(request): Json<GearCompareApiRequest>,
) -> Result<Json<GearCompareApiResponse>> {
    let simulation_response1 = dps_analysis(
        request.gear1_request,
        NUMBER_OF_ITERATIONS_PER_REQUEST_GEAR_COMPARE,
    )?;
    let simulation_response2 = dps_analysis(
        request.gear2_request,
        NUMBER_OF_ITERATIONS_PER_REQUEST_GEAR_COMPARE,
    )?;

    Ok(Json(GearCompareApiResponse::from((
        simulation_response1,
        simulation_response2,
    ))))
}
