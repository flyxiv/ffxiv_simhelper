use crate::live_objects::turn_type::FfxivTurnType;
use crate::{ComboType, IdType, ResourceType, TimeType};

/// Events that happen to a player's internal status "instantly" after casting a skill
/// Ex) Stack is raised, cooldown is started, combo is updated
pub enum FfxivPlayerInternalEvent {
    /// stack id, increase amount
    IncreaseResource(IdType, ResourceType),
    UseResource(IdType, ResourceType),
    /// skill ID
    StartCooldown(IdType),
    /// combo ID
    UpdateCombo(ComboType),
    /// turn type, combat time, charge time, cast time, gcd cooldown, delay of current turn
    UpdateTurn(
        FfxivTurnType,
        TimeType,
        TimeType,
        TimeType,
        TimeType,
        TimeType,
    ),

    /// buff_id
    RemoveBuff(IdType),
    /// debuff_id
    RemoveDebuff(IdType),
}
