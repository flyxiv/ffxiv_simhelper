use serde::Serialize;

/// Response object for the Best Partner API
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestPartnerApiResponse {
    /// 3-letter abbreviation of the player's job
    pub partner_job_abbrev: String,

    /// The median contribution of partner to main player's buffs
    /// * The first element is the total contribution of the partner.
    /// * The rest of the elements are the contribution of the partner at each burst period (0min, 2min, 4min, 6min, ...).
    ///
    /// ex) if the fight was a 6-minute fight, and the partner **contributed 200, 400, 300, 500 raid damage at opener, 2min, 4min, 6min, respectively,**
    /// then ```contributed_damage = [1400, 200, 400, 300, 500]```
    pub contributed_damage: Vec<i32>,
}
