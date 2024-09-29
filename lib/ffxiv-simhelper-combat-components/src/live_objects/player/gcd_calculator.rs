use crate::damage_calculator::{INCREASE_BASE, MULTIPLIER_BASE};
use crate::types::MultiplierType;
use crate::types::TimeType;
use log::info;

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
        debug_assert!(speed_multiplier >= 1.0, "{}", speed_multiplier);
        let gcd_multiplier = f64::floor((2.0 - speed_multiplier) * 1000.0) / 1000.0;
        let gcd_millisecond =
            f64::floor(gcd_delay as MultiplierType * gcd_multiplier) * speed_buff_reducer;
        (gcd_millisecond / 10.0f64) as TimeType * 10
    }
}
