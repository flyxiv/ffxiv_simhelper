use crate::live_objects::player::role::{job_abbrev_to_role, Role};
use crate::types::{BuffIncreasePercentType, IncreaseType, MultiplierType, StatType};
use serde::{Deserialize, Serialize};
use std::cmp::min;

const NON_TANK_MULTIPLIER: MultiplierType = 237.0 / 440.0;
const TANK_MULTIPLIER: MultiplierType = 190.0 / 440.0;
const MAIN_STAT_BASE_NON_TANK: StatType = 440;
const MAIN_STAT_BASE_TANK: StatType = 440;

/// The power specification of the player. Contains the player's stats and stat ladder multiplier values of each player.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPower {
    /// The auto attack delay of the weapon the player is using. This value is usually a constant value for each job. The table of job -> auto attack delay is kept in the application frontend. Server
    /// just receives this value and uses it.
    /// Auto attack damage is adjusted by this value by (auto_attack_delays) / 3.
    ///
    /// ex) if auto_attack_delays is 2.82 and base auto attack damage was 300, then its actual auto attack damage is 300 * (2.82 / 3) = 282
    pub auto_attack_delays: MultiplierType,

    /// The critical strike rate of the player.
    /// value must be >=0.0 and <=1.0
    /// ex) if critical_strike_rate is 0.354, then the player has 35.4% chance to critical strike.
    pub critical_strike_rate: MultiplierType,

    /// The critical strike damage multiplier of the player.
    /// value must be >= 1.0
    /// ex) if critical_strike_damage is 1.700, and skill's normal damage was 100, then that skill's damage will be 100 * 1.700 = 170 when it crits.
    pub critical_strike_damage: MultiplierType,

    /// The direct hit rate of the player.
    /// value must be >=0.0 and <=1.0
    /// ex) if direct_hit_rate is 0.354, then the player has 35.4% chance to direct hit.
    pub direct_hit_rate: MultiplierType,

    /// The determination multiplier of the player.
    /// value must be >= 1.0
    /// ex) if determination_multiplier is 1.201, and a skill's damage was 100 when its determination_multiplier was 1.000, then that skill's damage will be 100 * 1.201 = 120.1 when the player has 1.201 determination_multiplier.
    pub determination_multiplier: MultiplierType,

    /// The tenacity multiplier of the player.
    /// ex) if tenacity_multiplier is 1.201, and a skill's damage was 100 when its tenacity_multiplier was 1.000, then that skill's damage will be 100 * 1.201 = 120.1 when the player has 1.201 tenacity_multiplier.
    pub tenacity_multiplier: MultiplierType,

    /// The speed multiplier of the player.
    /// Used for GCD calculation and dot damage calculation.
    pub speed_multiplier: MultiplierType,

    /// The weapon damage multiplier of the player.
    /// ex) if weapon_damage_multiplier is 1.96, and a skill's damage was 100 when its weapon_damage_multiplier was 1.00, then that skill's damage will be 100 * 1.96 = 196 when the player has 1.96 weapon_damage_multiplier.
    pub weapon_damage_multiplier: MultiplierType,

    /// The main stat multiplier of the player.
    /// must be >= 1.0
    /// ex) if main_stat_multiplier is 23.96, and a skill's damage was 100 when its main_stat_multiplier was 1.00, then that skill's damage will be 100 * 23.96 = 2396 when the player has 23.96 main_stat_multiplier.
    pub main_stat_multiplier: MultiplierType,

    /// in 6.2, auto_direct_hits were added, which boosts the damage of guaranteed direct hit skills.  
    /// must be >= 0.0 and <= 1.0
    /// ex) if auto_direct_hit_increase is 0.071, and a direct hit guaranteed skill's damage was 100 when its direct hit rate was 0.000, then that skill's damage will be 100 * (1 + 0.071) = 107.1 when the player has 0.071 auto_direct_hit_increase.
    pub auto_direct_hit_increase: MultiplierType,

    /// The weapon damage of the player. Not used directly in the backend, and just passes this value to the response so that the app can use it in the result page.
    pub weapon_damage: StatType,

    /// The main stat of the player (one of STR, DEX, INT, MND depending on the player's job). Not used directly in the backend, and just passes this value to the response so that the app can use it in the result page.
    pub main_stat: StatType,

    /// The critical strike substat value of the player. Not used directly in the backend, and just passes this value to the response so that the app can use it in the result page.
    pub critical_strike: StatType,

    /// The direct hit substat value of the player. Not used directly in the backend, and just passes this value to the response so that the app can use it in the result page.
    pub direct_hit: StatType,

    /// The determination substat value of the player. Not used directly in the backend, and just passes this value to the response so that the app can use it in the result page.
    pub determination: StatType,

    /// The skill speed substat value of the player. Not used directly in the backend, and just passes this value to the response so that the app can use it in the result page.
    pub skill_speed: StatType,

    /// The spell speed substat value of the player. Not used directly in the backend, and just passes this value to the response so that the app can use it in the result page.
    pub spell_speed: StatType,

    /// The tenacity substat value of the player. Not used directly in the backend, and just passes this value to the response so that the app can use it in the result page.
    pub tenacity: StatType,
}

pub fn add_main_stat(
    power: &PlayerPower,
    job_abbrev: &String,
    maximum_increase: IncreaseType,
    increase_percent: BuffIncreasePercentType,
) -> PlayerPower {
    let mut power = power.clone();
    let role = job_abbrev_to_role(job_abbrev);

    let increase_percent_amount = (power.main_stat as MultiplierType
        * (increase_percent as MultiplierType / 100.0))
        as IncreaseType;
    let increase_amount = min(maximum_increase, increase_percent_amount);
    power.main_stat += increase_amount as StatType;
    power.main_stat_multiplier = convert_main_stat_to_multiplier(power.main_stat, role);
    power
}

fn convert_main_stat_to_multiplier(main_stat: StatType, role: Role) -> MultiplierType {
    let main_stat_multiplier_increase_percent = if matches!(role, Role::Tank) {
        f64::floor((main_stat - MAIN_STAT_BASE_TANK) as MultiplierType * TANK_MULTIPLIER)
    } else {
        f64::floor((main_stat - MAIN_STAT_BASE_NON_TANK) as MultiplierType * NON_TANK_MULTIPLIER)
    };

    1.0 + main_stat_multiplier_increase_percent / 100.0
}

#[cfg(test)]
mod tests {
    use super::convert_main_stat_to_multiplier;

    #[test]
    fn add_main_stat_test() {
        let power1 = super::PlayerPower {
            auto_attack_delays: 2.96,
            critical_strike_rate: 0.374,
            critical_strike_damage: 1.724,
            direct_hit_rate: 0.421,
            determination_multiplier: 1.211,
            tenacity_multiplier: 1.000,
            speed_multiplier: 1.000,
            weapon_damage_multiplier: 1.96,
            main_stat_multiplier: 23.96,
            auto_direct_hit_increase: 0.071,
            weapon_damage: 144,
            main_stat: 5104,
            critical_strike: 3122,
            direct_hit: 2596,
            determination: 2106,
            skill_speed: 420,
            spell_speed: 420,
            tenacity: 420,
        };

        // Test 1. Non-tank job
        let job_abbrev = "DRG".to_string();
        let maximum_increase = 391;
        let increase_percent = 10;

        let power_after_pot =
            super::add_main_stat(&power1, &job_abbrev, maximum_increase, increase_percent);

        assert_eq!(
            power_after_pot.main_stat,
            power1.main_stat + maximum_increase
        );
        assert_eq!(power_after_pot.main_stat_multiplier, 28.22);

        // Test 2. Tank job
        let job_abbrev2 = "PLD".to_string();

        let power_after_pot2 =
            super::add_main_stat(&power1, &job_abbrev2, maximum_increase, increase_percent);

        assert_eq!(
            power_after_pot2.main_stat,
            power1.main_stat + maximum_increase
        );
        assert_eq!(power_after_pot2.main_stat_multiplier, 22.82);

        // Test 3. Increase amount is less than maximum increase
        let power2 = super::PlayerPower {
            auto_attack_delays: 2.96,
            critical_strike_rate: 0.374,
            critical_strike_damage: 1.724,
            direct_hit_rate: 0.421,
            determination_multiplier: 1.211,
            tenacity_multiplier: 1.000,
            speed_multiplier: 1.000,
            weapon_damage_multiplier: 1.96,
            main_stat_multiplier: 23.96,
            auto_direct_hit_increase: 0.071,
            weapon_damage: 144,
            main_stat: 2052,
            critical_strike: 3122,
            direct_hit: 2596,
            determination: 2106,
            skill_speed: 420,
            spell_speed: 420,
            tenacity: 420,
        };

        let power_after_pot3 =
            super::add_main_stat(&power2, &job_abbrev, maximum_increase, increase_percent);

        assert_eq!(power_after_pot3.main_stat, power2.main_stat + 205);
        assert_eq!(power_after_pot3.main_stat_multiplier, 10.78);

        let power_after_pot4 =
            super::add_main_stat(&power2, &job_abbrev2, maximum_increase, increase_percent);

        assert_eq!(power_after_pot4.main_stat, power2.main_stat + 205);
        assert_eq!(power_after_pot4.main_stat_multiplier, 8.84);
    }

    #[test]
    fn convert_main_stat_to_multiplier_test() {
        let main_stat1 = 5042;
        let main_stat2 = 4994;
        let main_stat3 = 4849;

        let tank_main_stat1 = 4842;
        let tank_main_stat2 = 4987;
        let tank_main_stat3 = 5035;

        let non_tank_job_abbrev = "AST".to_string();
        let tank_job_abbrev = "PLD".to_string();

        let non_tank_main_stat_multiplier1 = convert_main_stat_to_multiplier(
            main_stat1,
            super::job_abbrev_to_role(&non_tank_job_abbrev),
        );
        let non_tank_main_stat_multiplier2 = convert_main_stat_to_multiplier(
            main_stat2,
            super::job_abbrev_to_role(&non_tank_job_abbrev),
        );
        let non_tank_main_stat_multiplier3 = convert_main_stat_to_multiplier(
            main_stat3,
            super::job_abbrev_to_role(&non_tank_job_abbrev),
        );

        let tank_main_stat_multiplier1 = convert_main_stat_to_multiplier(
            tank_main_stat1,
            super::job_abbrev_to_role(&tank_job_abbrev),
        );
        let tank_main_stat_multiplier2 = convert_main_stat_to_multiplier(
            tank_main_stat2,
            super::job_abbrev_to_role(&tank_job_abbrev),
        );
        let tank_main_stat_multiplier3 = convert_main_stat_to_multiplier(
            tank_main_stat3,
            super::job_abbrev_to_role(&tank_job_abbrev),
        );

        assert_eq!(non_tank_main_stat_multiplier1, 25.78);
        assert_eq!(non_tank_main_stat_multiplier2, 25.52);
        assert_eq!(non_tank_main_stat_multiplier3, 24.74);

        assert_eq!(tank_main_stat_multiplier1, 20.00);
        assert_eq!(tank_main_stat_multiplier2, 20.63);
        assert_eq!(tank_main_stat_multiplier3, 20.84);
    }
}
