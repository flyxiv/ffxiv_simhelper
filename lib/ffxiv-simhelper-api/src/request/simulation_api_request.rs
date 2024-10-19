use ffxiv_simhelper_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simhelper_combat_components::types::{PlayerIdType, TimeType};
use serde::Deserialize;

/// The main request body for the simulation API(=DPS Analysis)
/// The DPS Analysis is the main API of FFXIV Simhelper, so this request type is also the
/// the most important request body and the building block to all other request API types.
///
/// # request example)
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
///             speed_multiplier: 1.000,
///             weapon_damage_multiplier: 1.96,
///             main_stat_multiplier: 23.96,
///             auto_direct_hit_increase: 0.071,
///             weapon_damage: 144,
///             main_stat: 5884,
///             critical_strike: 3122,
///             direct_hit: 2596,
///             determination: 2106,
///             skill_speed: 420,
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
///
///    use_pot: true
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationApiRequest {
    /// The main player id. Used to locate PartyInfoRequest that belongs to the main player. ```party[main_player_id]``` will contain the main player's data.
    pub main_player_id: PlayerIdType,

    /// Combat time to be simulated in milliseconds.
    pub combat_time_millisecond: TimeType,

    /// Contains data needed for DPS simulation for each party member. party[player_id] contains the data for the player with the given player_id.
    pub party: Vec<PlayerInfoRequest>,

    /// The ilvl adjustment factor used to simulate the party ilvl. A floating point between 0.0 and 1.0.
    /// All party member's damage will be adjusted by this factor.
    /// ex) if party_ilvl_adjustment is 0.8, if a party member's rdps contribution is 1000, it will be adjusted to 1000 * 0.8 = 800.
    pub party_ilvl_adjustment: f64,

    /// Flag that decides whether to use potions in the simulation rotation.
    pub use_pot: bool,
}

/// Data of individual players in the party that is needed to simulate their DPS.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfoRequest {
    pub player_id: PlayerIdType,

    /// If the job buffs an individual party member, it can manually set the partner player id that the player will be giving buffs to.
    ///
    /// **Current jobs that use this field: DNC and AST.**
    ///
    /// 1) for DNC, partner1_id is the **player that the DNC will give the closed position buff to.**
    /// 2) for AST, partner1_id is the **player that the AST will give the balance buff to(thus should be a melee job for maximum efficiency).**
    /// 3) if the field is ```null``` for DNC or AST, the engine will **automatically find the best partner for the player by internal algorithms.**
    ///
    /// * ex1)
    /// ```json
    /// {
    ///     job_abbrev: "DRG",
    ///     partner1_id: 0, // ignored, since DRG doesn't use partner1_id
    /// }
    /// ```
    ///
    /// * ex2)
    /// ```json
    /// {
    ///     job_abbrev: "DNC",
    ///     partner1_id: 3, // will give the closed position buff to player id 3
    /// }
    ///
    /// * ex3)
    /// ```json
    /// // party members: [0: DRG, 2: DNC, 4: PCT, 5: BRD]
    /// {
    ///    job_abbrev: "AST",
    ///   partner1_id: null, // will give the balance buff to player id 0, since the internal algorithm prioritizes DRG for melee partner out of the four party member jobs.
    /// }
    ///
    /// * ex4)
    /// ```json
    /// // party members: [0: DRG, 2: DNC, 4: PCT, 5: BRD]
    /// {
    ///   job_abbrev: "AST",
    ///   partner1_id: null, // will give the balance buff to player id 4, since the internal algorithm prioritizes PCT for Dancer's closed position partner out of the four party member jobs.
    /// }
    pub partner1_id: Option<PlayerIdType>,

    /// If the job buffs two individual party member, this field is used to manually set the second partner player id that the player will be giving buffs to.
    ///
    /// **Current jobs that use this field: AST.**
    ///
    /// 1) for AST, partner2_id is the **player that the AST will give the spear buff to(thus should be a range job for maximum efficiency).**
    /// 2) if the field is ```null``` for AST, the engine will **automatically find the best partner for the player by internal algorithms.**
    ///
    /// * ex1)
    /// ```json
    /// {
    ///     job_abbrev: "DNC",
    ///     partner2_id: 0, // ignored, since DNC doesn't use partner2_id
    /// }
    ///
    /// * ex2)
    /// ```json
    /// {
    ///     job_abbrev: "AST",
    ///     partner2_id: 3, // will give the spear buff to player id 3
    /// }
    ///
    /// * ex3)
    ///
    /// ```json
    /// // party members: [0: DRG, 2: DNC, 4: PCT, 5: BRD]
    /// {
    ///     job_abbrev: "AST",
    ///     partner2_id: null, // will give the spear buff to player id 4, since the internal algorithm prioritizes PCT for ranged partner out of the four party member jobs.
    /// }
    /// ```
    pub partner2_id: Option<PlayerIdType>,

    /// The job abbreviation of the player's combat job. ex) "DRG", "NIN", "AST", "DNC"
    pub job_abbrev: String,

    /// The power specification of the player. Contains the player's stats and stat ladder multiplier values of each player.
    pub power: PlayerPower,
}
