use crate::types::{BuffIncreasePercentType, IncreaseType};

#[derive(Clone, Debug)]
pub enum StatusInfo {
    DamagePercent(BuffIncreasePercentType),
    CritHitRatePercent(BuffIncreasePercentType),
    DirectHitRatePercent(BuffIncreasePercentType),
    SpeedPercent(BuffIncreasePercentType),
    /// increase % by stack
    SpeedByStack(Vec<BuffIncreasePercentType>),
    SpeedOnlyAutoAttack(BuffIncreasePercentType),

    /// max increase value, increase %
    IncreaseMainStat(IncreaseType, BuffIncreasePercentType),
    None,
}

impl PartialEq<Self> for StatusInfo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StatusInfo::DamagePercent(a), StatusInfo::DamagePercent(b)) => a == b,
            (StatusInfo::CritHitRatePercent(a), StatusInfo::CritHitRatePercent(b)) => a == b,
            (StatusInfo::DirectHitRatePercent(a), StatusInfo::DirectHitRatePercent(b)) => a == b,
            (StatusInfo::SpeedPercent(a), StatusInfo::SpeedPercent(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for StatusInfo {}
