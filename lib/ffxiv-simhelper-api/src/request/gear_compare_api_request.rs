use crate::request::simulation_api_request::SimulationApiRequest;
use serde::Deserialize;

/// Request type for GearCompare API
/// Received by POST body
/// Receives two SimulationApiRequest objects, each containing info of the gearset to be compared
/// To control the variables of the simulation experiment, some fields have restrictions:
///
/// ## Restriction 1. main_player's job_abbrev is the same for the two requests.
/// ## Restriction 2. party composition is the same for the two requests.
///
/// # request example)
/// ```json
/// {
///    gear1_request: {
///         main_player_id: 0,
///
///         // this field is ignored for GearCompare API
///         combat_time_millisecond: 300000,
///
///         party: [{
///             player_id: 0,
///             partner1_id: null,
///             partner2_id: null,
///             job_abbrev: "DRG",
///             power: {
///                 auto_attack_delays: 2.96,
///                 critical_strike_rate: 0.374,
///                 critical_strike_damage: 1.724,
///                 direct_hit_rate: 0.421,
///                 determination_multiplier: 1.206,
///                 tenacity_multiplier: 1.000,
///                 speed_multiplier: 1.005,
///                 weapon_damage_multiplier: 1.96,
///                 main_stat_multiplier: 23.96,
///                 auto_direct_hit_increase: 0.071,
///                 weapon_damage: 144,
///                 main_stat: 5884,
///                 critical_strike: 3122,
///                 direct_hit: 2596,
///                 determination: 2106,
///                 skill_speed: 420,
///                 spell_speed: 420,
///                 tenacity: 420,
///             }
///         },
///         {
///             player_id: 1,
///             partner1_id: null,
///             partner2_id: 0,
///             job_abbrev: "AST",
///             power: {
///                 auto_attack_delays: 2.96,
///                 critical_strike_rate: 0.374,
///                 critical_strike_damage: 1.724,
///                 direct_hit_rate: 0.421,
///                 determination_multiplier: 1.211,
///                 tenacity_multiplier: 1.000,
///                 speed_multiplier: 1.007,
///                 weapon_damage_multiplier: 1.96,
///                 main_stat_multiplier: 23.96,
///                 auto_direct_hit_increase: 0.071,
///                 weapon_damage: 144,
///                 main_stat: 5884,
///                 critical_strike: 3122,
///                 direct_hit: 2596,
///                 determination: 2106,
///                 skill_speed: 420,
///                 spell_speed: 524,
///                 tenacity: 420,
///             }
///         },
///         // ... more party members
///    ],
///
///     gear2_request: {
///         main_player_id: 0,
///
///         // this field is ignored for GearCompare API
///         combat_time_millisecond: 300000,
///
///         party: [{
///             player_id: 0,
///             partner1_id: null,
///             partner2_id: null,
///             job_abbrev: "DRG",
///             power: {
///                 auto_attack_delays: 2.96,
///                 critical_strike_rate: 0.354,
///                 critical_strike_damage: 1.704,
///                 direct_hit_rate: 0.426,
///                 determination_multiplier: 1.226,
///                 tenacity_multiplier: 1.000,
///                 speed_multiplier: 1.005,
///                 weapon_damage_multiplier: 1.96,
///                 main_stat_multiplier: 23.96,
///                 auto_direct_hit_increase: 0.071,
///                 weapon_damage: 144,
///                 main_stat: 5884,
///                 critical_strike: 3122,
///                 direct_hit: 2596,
///                 determination: 2106,
///                 skill_speed: 420,
///                 spell_speed: 420,
///                 tenacity: 420,
///             }
///         },
///         {
///             player_id: 1,
///             partner1_id: null,
///             partner2_id: 0,
///             job_abbrev: "AST",
///             power: {
///                 auto_attack_delays: 2.96,
///                 critical_strike_rate: 0.374,
///                 critical_strike_damage: 1.724,
///                 direct_hit_rate: 0.421,
///                 determination_multiplier: 1.211,
///                 tenacity_multiplier: 1.000,
///                 speed_multiplier: 1.007,
///                 weapon_damage_multiplier: 1.96,
///                 main_stat_multiplier: 23.96,
///                 auto_direct_hit_increase: 0.071,
///                 weapon_damage: 144,
///                 main_stat: 5884,
///                 critical_strike: 3122,
///                 direct_hit: 2596,
///                 determination: 2106,
///                 skill_speed: 420,
///                 spell_speed: 524,
///                 tenacity: 420,
///             }
///         },
///         // ... more party members
///    ],
///
///    party_ilvl_adjustment: 0.85,
///    use_pot: true
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GearCompareApiRequest {
    /// Contains info of the first gearset
    pub gear1_request: SimulationApiRequest,

    /// Contains info of the second gearset
    pub gear2_request: SimulationApiRequest,
}
