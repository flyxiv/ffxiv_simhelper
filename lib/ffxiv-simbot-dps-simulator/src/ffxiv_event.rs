use ffxiv_simbot_combat_components::live_objects::turn_type::PlayerTurn;
use ffxiv_simbot_combat_components::{IdType, TimeType};

/// All possible damage related events in a FFXIV combat.
pub(crate) enum FfxivEvent {
    /// owner_player_id, turn, time
    PlayerTurn(IdType, PlayerTurn, TimeType),

    /// skill ID
    Damage(IdType, TimeType),

    /// ticker ID
    Tick(IdType, TimeType),
}
