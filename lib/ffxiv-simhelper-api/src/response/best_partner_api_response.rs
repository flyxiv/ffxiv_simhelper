use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestPartnerApiResponse {
    pub partner_job_abbrev: String,
    pub contributed_dps: Vec<i32>,
}
