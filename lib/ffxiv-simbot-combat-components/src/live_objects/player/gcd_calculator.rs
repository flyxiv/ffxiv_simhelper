use crate::status::buff_status::BuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::TimeType;
use ffxiv_simbot_db::MultiplierType;

pub(crate) trait GcdCalculator {
    fn calculate_gcd_millisecond(
        &self,
        gcd_delay: TimeType,
        speed_multiplier: MultiplierType,
        speed_buff_increase: MultiplierType,
    ) -> TimeType {
        let gcd_millisecond = gcd_delay as MultiplierType / speed_multiplier / speed_buff_increase;
        (gcd_millisecond / 10.0f64) as TimeType * 10
    }
}
