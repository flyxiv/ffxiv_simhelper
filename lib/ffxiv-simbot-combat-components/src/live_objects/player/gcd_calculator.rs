use crate::damage_calculator::{INCREASE_BASE, MULTIPLIER_BASE};
use crate::types::MultiplierType;
use crate::types::TimeType;
use log::debug;

pub(crate) trait GcdCalculator {
    fn calculate_speed_buffed_cooldown_millisecond(
        &self,
        gcd_delay: TimeType,
        speed_multiplier: MultiplierType,
        speed_buff_reducer: MultiplierType,
    ) -> TimeType {
        debug_assert!(
            speed_buff_reducer > INCREASE_BASE && speed_buff_reducer <= MULTIPLIER_BASE,
            "{}",
            speed_buff_reducer
        );
        let gcd_millisecond = gcd_delay as MultiplierType / speed_multiplier * speed_buff_reducer;
        (gcd_millisecond / 10.0f64) as TimeType * 10
    }
}
