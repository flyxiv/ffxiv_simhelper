use ffxiv_simbot_combat_components::TimeType;
use ffxiv_simbot_db::DamageMultiplierType;

pub(crate) fn calculate_gcd(
    gcd_delay: TimeType,
    speed_multiplier: DamageMultiplierType,
) -> TimeType {
    let gcd_millisecond = gcd_delay as DamageMultiplierType / speed_multiplier;

    (gcd_millisecond / 10.0f64) as TimeType * 10
}
