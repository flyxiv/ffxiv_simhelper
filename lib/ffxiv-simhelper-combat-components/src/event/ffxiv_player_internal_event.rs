use crate::live_objects::turn_type::FfxivTurnType;
use crate::types::{ComboType, ResourceIdType, ResourceType};
use crate::types::{SkillIdType, TimeType};

/// Events that happen to a player's internal status "instantly" after casting a skill
/// These kinds of events only change the player's internal state, so the party members aren't notified of these events and they are handled the instant they are generated.
/// Ex) Stack is raised, cooldown is started, combo is updated
#[derive(Clone)]
pub enum FfxivPlayerInternalEvent {
    /// stack id, increase amount
    IncreaseResource(ResourceIdType, ResourceType),
    UseResource(ResourceIdType, ResourceType),
    /// skill ID
    StartCooldown(SkillIdType),
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
    RemoveBuff(SkillIdType),
    /// debuff_id
    RemoveDebuff(SkillIdType),
}
