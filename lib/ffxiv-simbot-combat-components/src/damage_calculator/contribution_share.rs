use crate::BuffIncreasePercentType;

pub(crate) enum ContributionShare {
    GetDamageIncrease(BuffIncreasePercentType),
    GetCritRateIncrease(BuffIncreasePercentType),
    GetDirectHitRateIncrease(BuffIncreasePercentType),
}
