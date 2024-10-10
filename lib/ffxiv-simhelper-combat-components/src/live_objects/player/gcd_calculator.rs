use crate::damage_calculator::{INCREASE_BASE, MULTIPLIER_BASE};
use crate::types::MultiplierType;
use crate::types::TimeType;

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

#[cfg(test)]
mod tests {
    use crate::live_objects::player::gcd_calculator::GcdCalculator;

    struct GcdCalculatorStruct {}

    impl GcdCalculator for GcdCalculatorStruct
    #[test]
    fn gcd_calculation_test() {
        let gcd_calculator = GcdCalculatorStruct{};

        // test 1. MNK at 955 SKS, which is


        // test 2. VPR at 530 SKS, skill of GCD 3s with 15% speed buff

        // test 3. BLM at 1930 SPS, skill of GCD 2.8s with ley line speed buff


        // test 4. PCT at 420(default) SPS, skill of GCD 2.8 with Hyperphantasia


        // test 3. BLM at 1930 SPS, sill of GCD 2.5 with ley line speed buff
    }
}