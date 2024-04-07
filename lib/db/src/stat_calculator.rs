use crate::character::{get_character_main_stats, get_character_sub_stats, Character};
use crate::job::StatModifierType;
use crate::stat::SubStatTrait;
use crate::StatModifier;

static BASE_PHYSICAL_MAIN_STAT_MULTIPLIER = 705;
static BASE_CRITICAL_RATE: f64 = 0.05;
static BASE_CRITICAL_DAMAGE: f64 = 1.4;
static CRIT_MULTIPLIER: f64 = 200.0;
static DIRECT_HIT_MULTIPLIER: f64 = 550.0;
static DETERMINATION_MULTIPLIER: f64 = 140.0;
static TENACITY_MULTIPLIER: f64 = 100.0;
static SPEED_MULTIPLIER: f64 = 130.0;
static UNIT_BASE: f64 = 1000.0;
static SPEED_BASE: f64 = 10000.0;

static WEAPON_ATTACK_BASE: usize = 22;

pub struct CharacterPower {
    critical_strike_rate: f64,
    direct_hit_rate: f64,
    determination_damage_multiplier: f64,
    gcd_multiplier: f64,
    auto_attack_hot_multiplier: f64,
    weapon_dmg_multiplier: f64,
    main_stat_multiplier: f64,
}

pub fn convert_character_to_power(
    character: &Character,
    stat_modifier: StatModifier,
) -> CharacterPower {
    let sub_stats = get_character_sub_stats(&character);
    let critical_stat = sub_stats.get_critical_strike();
    let direct_hit_stat = sub_stats.get_direct_hit();
    let determination_stat = sub_stats.get_determination();
    let skill_speed_stat = sub_stats.get_skill_speed();
    let spell_speed_stat = sub_stats.get_spell_speed();
    let tenacity_stat = sub_stats.get_tenacity();
    let main_stats = get_character_main_stats(&character);

    let critical_strike_increase = get_increase(
        critical_stat as StatModifierType,
        stat_modifier.max_level_base_critical_hit as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        CRIT_MULTIPLIER,
    );

    let critical_rate = BASE_CRITICAL_RATE + critical_strike_increase;
    let critical_damage = BASE_CRITICAL_DAMAGE + critical_strike_increase;

    let direct_hit_increase = get_increase(
        direct_hit_stat as StatModifierType,
        stat_modifier.max_level_base_direct_hit as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        DIRECT_HIT_MULTIPLIER,
    );

    let direct_hit_rate = direct_hit_increase;

    let determination_increase = get_increase(
        determination_stat as StatModifierType,
        stat_modifier.max_level_base_determination as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        DETERMINATION_MULTIPLIER,
    );

    let determination_damage_multiplier = 1.0 + determination_increase;

    let tenacity_increase = get_increase(
        tenacity_stat as StatModifierType,
        stat_modifier.max_level_base_tenacity as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        TENACITY_MULTIPLIER,
    );
    let tenacity_damage_multiplier = 1.0 + tenacity_increase;

    let speed_increase = get_increase(
        skill_speed_stat as StatModifierType,
        stat_modifier.max_level_base_skill_speed as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        SPEED_MULTIPLIER,
    );

    let auto_attack_hot_multiplier = 1.0 + speed_increase;
    let skill_gcd_multiplier = get_speed_multiplier(
        skill_speed_stat as StatModifierType,
        stat_modifier.max_level_base_skill_speed as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        SPEED_MULTIPLIER,
    );
    let spell_gcd_multiplier = get_speed_multiplier(
        spell_speed_stat as StatModifierType,
        stat_modifier.max_level_base_skill_speed as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        SPEED_MULTIPLIER,
    );

    //TODO: Mainstat/weapon
    let weapon_damage = character.get_weapon_damage() + WEAPON_ATTACK_BASE;
    let main_stat_multiplier = 1.0 + get_increase(
        main_stats as StatModifierType,
        stat_modifier.max_level_main_stat_modifier,
        stat_modifier.max_level_sub_stat_modifier,
        BASE_PHYSICAL_MAIN_STAT_MULTIPLIER as f64,
    );
}

227, 107%
638, 261%
1032, 408%
1280, 500%
1528, 592%
248, 92
1921, 738
146/393
b * 411 / 390 =154 * 390 /411 = 146.
92 / 248
23/62
145.826 / 390 = 0.374
147 / 394 = 0.372
// calculate 154 / (638 - 227), answer is b * (x - a) / 390


fn get_increase(
    stat: StatModifierType,
    base_stat: StatModifierType,
    modifier: StatModifierType,
    multiplier: f64,
) -> f64 {
    let multiplied = multiplier * (stat - base_stat);
    let modified = f64::floor(multiplied / modifier);
    modified / UNIT_BASE
}

fn get_speed_multiplier(
    speed_stat: StatModifierType,
    base_speed: StatModifierType,
    max_level_sub_stat_modifier: StatModifierType,
    multiplier: f64,
) -> f64 {
    let speed_diff = base_speed - speed_stat;
    let speed_multiplier = f64::ceil(multiplier * speed_diff / max_level_sub_stat_modifier);
    speed_multiplier + UNIT_BASE / SPEED_BASE
}
