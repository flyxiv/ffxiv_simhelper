use crate::request::simulation_api_request::{PlayerInfoRequest, SimulationApiRequest};
use ffxiv_simhelper_combat_components::types::{PlayerIdType, StatType, TimeType};
use serde::Deserialize;

/// The request body for the BestStats API
/// Very similar to SimulationApiRequest body but has extra data to show which stat was
/// augmented by how much
///
/// /// # request example)
/// ```json
/// {
///     main_player_id: 0,
///
///     combat_time_millisecond: 300000,
///
///     party: [{
///         player_id: 0,
///         partner1_id: null,
///         partner2_id: null,
///         job_abbrev: "DRG",
///         power: {
///             auto_attack_delays: 2.96,
///             critical_strike_rate: 0.374,
///             critical_strike_damage: 1.724,
///             direct_hit_rate: 0.421,
///             determination_multiplier: 1.211,
///             tenacity_multiplier: 1.000,
///             speed_multiplier: 1.011,
///             weapon_damage_multiplier: 1.96,
///             main_stat_multiplier: 23.96,
///             auto_direct_hit_increase: 0.071,
///             weapon_damage: 144,
///             main_stat: 5884,
///             critical_strike: 3122,
///             direct_hit: 2596,
///             determination: 2106,
///             skill_speed: 624,  // 420 + 204
///             spell_speed: 420,
///             tenacity: 420,
///         }
///     },
///     {
///         player_id: 1,
///         partner1_id: null,
///         partner2_id: 0,
///         job_abbrev: "AST",
///         power: {
///             auto_attack_delays: 2.96,
///             critical_strike_rate: 0.374,
///             critical_strike_damage: 1.724,
///             direct_hit_rate: 0.421,
///             determination_multiplier: 1.211,
///             tenacity_multiplier: 1.000,
///             speed_multiplier: 1.007,
///             weapon_damage_multiplier: 1.96,
///             main_stat_multiplier: 23.96,
///             auto_direct_hit_increase: 0.071,
///             weapon_damage: 144,
///             main_stat: 5884,
///             critical_strike: 3122,
///             direct_hit: 2596,
///             determination: 2106,
///             skill_speed: 420,
///             spell_speed: 524,
///             tenacity: 420,
///         }
///     },
///     // ... more party members
///    ],
///
///    party_ilvl_adjustment: 0.85,
///    stat_name: "SKS",
///    augment_amount: 204,
///
///    use_pot: true
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestStatsApiRequest {
    pub main_player_id: PlayerIdType,
    pub combat_time_millisecond: TimeType,

    /// Same as the party field in SimulationApiRequest
    pub party: Vec<PlayerInfoRequest>,

    /// The name of the stat that was augmented
    /// ex) "WD", "STR", "CRT", "DH", "DET", "SKS", "SPS", "TEN"
    pub stat_name: String,

    /// Shows how much the stat was augmented
    pub augment_amount: StatType,
    pub party_ilvl_adjustment: f64,
    pub use_pot: bool,
}

impl From<&BestStatsApiRequest> for SimulationApiRequest {
    fn from(request: &BestStatsApiRequest) -> Self {
        SimulationApiRequest {
            main_player_id: request.main_player_id,
            combat_time_millisecond: request.combat_time_millisecond,
            party: request.party.clone(),
            use_pot: request.use_pot,
            party_ilvl_adjustment: request.party_ilvl_adjustment,
        }
    }
}
