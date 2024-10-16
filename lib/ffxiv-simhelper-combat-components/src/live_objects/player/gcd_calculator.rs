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

    impl GcdCalculator for GcdCalculatorStruct {}

    #[test]
    fn gcd_calculation_test() {
        let gcd_calculator = GcdCalculatorStruct {};

        // test 1. MNK at 955 SKS(102.5%) is the start of 1.94 GCD
        let skill_speed_multiplier = 1.025;
        let speed_buff_reducer = 0.80;
        let gcd_delay = 2500;

        let monk_gcd_millisecond = gcd_calculator.calculate_speed_buffed_cooldown_millisecond(
            gcd_delay,
            skill_speed_multiplier,
            speed_buff_reducer,
        );

        assert_eq!(
            monk_gcd_millisecond, 1940,
            "GCD calculation has to be 1940, but it is {}",
            monk_gcd_millisecond
        );

        // test 2. BLM at 1283 SPS(104.0%), skill of GCD 2.5 is 2.40, 2.5 with leyline is 2.04, 2.8 is 2.68, and 2.8 with leyline is 2.28
        let blm_speed_multiplier = 1.040;
        let speed_buff_reducer_with_leyline = 0.85;

        let blm_normal_gcd_speed = gcd_calculator.calculate_speed_buffed_cooldown_millisecond(
            2500,
            blm_speed_multiplier,
            1.0,
        );

        let blm_fire4_gcd_speed = gcd_calculator.calculate_speed_buffed_cooldown_millisecond(
            2800,
            blm_speed_multiplier,
            1.0,
        );

        let blm_normal_gcd_with_leyline = gcd_calculator
            .calculate_speed_buffed_cooldown_millisecond(
                2500,
                blm_speed_multiplier,
                speed_buff_reducer_with_leyline,
            );

        let blm_fire4_gcd_with_leyline = gcd_calculator
            .calculate_speed_buffed_cooldown_millisecond(
                2800,
                blm_speed_multiplier,
                speed_buff_reducer_with_leyline,
            );

        assert_eq!(
            blm_normal_gcd_speed, 2400,
            "GCD calculation has to be 2400, but it is {}",
            blm_normal_gcd_speed
        );
        assert_eq!(
            blm_fire4_gcd_speed, 2680,
            "GCD calculation has to be 2680, but it is {}",
            blm_fire4_gcd_speed
        );
        assert_eq!(
            blm_normal_gcd_with_leyline, 2040,
            "GCD calculation has to be 2040, but it is {}",
            blm_normal_gcd_with_leyline
        );
        assert_eq!(
            blm_fire4_gcd_with_leyline, 2280,
            "GCD calculation has to be 2280, but it is {}",
            blm_fire4_gcd_with_leyline
        );

        // test 3. PCT at 420(default) SPS, skill of GCD 3.30 is 2.47 with Hyperphantasia
        let pct_speed_multiplier = 1.0;
        let pct_speed_buff_reducer = 0.75;

        let pct_gcd_speed = gcd_calculator.calculate_speed_buffed_cooldown_millisecond(
            3300,
            pct_speed_multiplier,
            pct_speed_buff_reducer,
        );

        assert_eq!(
            pct_gcd_speed, 2470,
            "GCD calculation has to be 2470, but it is {}",
            pct_gcd_speed
        );
    }
}
