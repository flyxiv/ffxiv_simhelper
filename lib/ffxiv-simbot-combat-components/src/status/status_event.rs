use crate::{IdType, TimeType};

/// Buffs classified by who they are applied to
pub enum StatusEvent {
    /// owner_player_id, target_id
    ApplyBuff(IdType, IdType, Buff, TimeType),
    ApplyBuffStack(IdType, IdType, Buff, TimeType),
    ApplyRaidBuff(IdType, Buff, TimeType),

    /// owner_player_id, time
    ApplyDebuffStack(IdType, Debuff, TimeType),
    ApplyDebuff(IdType, Debuff, TimeType),
    ApplyDot(IdType, DotDebuff, TimeType),
}
