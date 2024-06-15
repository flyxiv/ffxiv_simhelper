use crate::StatModifierType;
use crate::{MultiplierType, StatModifier};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref EQUIPMENT_SLOTS: usize = 14;
    pub(crate) static ref WEAPON_SLOT: usize = 13;
    pub(crate) static ref OFFHAND_SLOT: usize = 2;
    pub(crate) static ref HEAD_SLOT: usize = 3;
    pub(crate) static ref BODY_SLOT: usize = 4;
    pub(crate) static ref HANDS_SLOT: usize = 5;
    pub(crate) static ref LEGS_SLOT: usize = 7;
    pub(crate) static ref FEET_SLOT: usize = 8;
    pub(crate) static ref EARS_SLOT: usize = 9;
    pub(crate) static ref NECK_SLOT: usize = 10;
    pub(crate) static ref WRISTS_SLOT: usize = 11;
    pub(crate) static ref RING_SLOT: usize = 12;
    pub(crate) static ref RING_SLOT2: usize = 14;
    pub(crate) static ref FFXIV_STRENGTH_JOB_ABBREVS: Vec<String> = vec![
        "MNK".to_string(),
        "DRG".to_string(),
        "RPR".to_string(),
        "PLD".to_string(),
        "WAR".to_string(),
        "GNB".to_string(),
        "DRK".to_string()
    ];
    pub(crate) static ref FFXIV_DEXTERITY_JOB_ABBREVS: Vec<String> = vec![
        "NIN".to_string(),
        "BRD".to_string(),
        "MCH".to_string(),
        "DNC".to_string(),
    ];
    pub(crate) static ref FFXIV_INTELLIGENCE_JOB_ABBREVS: Vec<String> =
        vec!["RDM".to_string(), "BLM".to_string(), "SMN".to_string(),];
    pub(crate) static ref FFXIV_MIND_JOB_ABBREVS: Vec<String> = vec![
        "WHM".to_string(),
        "SCH".to_string(),
        "AST".to_string(),
        "SGE".to_string(),
    ];
    pub(crate) static ref MELEE_JOB_ABBREVS: Vec<String> = vec![
        "MNK".to_string(),
        "DRG".to_string(),
        "NIN".to_string(),
        "SAM".to_string(),
        "RPR".to_string(),
        "BRD".to_string(),
        "MCH".to_string(),
        "DNC".to_string(),
        "PLD".to_string(),
        "WAR".to_string(),
        "GNB".to_string(),
        "DRK".to_string()
    ];
    pub static ref FFXIV_STAT_MODIFIER: StatModifier = StatModifier {
        max_level_main_stat_modifier: 390f64,
        max_level_base_vitality: 390,
        max_level_base_piety: 390,
        max_level_base_direct_hit: 400,
        max_level_base_critical_hit: 400,
        max_level_base_determination: 390,
        max_level_base_skill_speed: 400,
        max_level_base_spell_speed: 400,
        max_level_base_tenacity: 400,
        max_level_hp_modifier: 30f64,
        max_level_div: 3000f64,
        hp_per_vitality_non_tank: 24.3f64,
        hp_per_vitality_tank: 34.6f64,
        max_level_sub_stat_modifier: 1900f64,
    };
    pub(crate) static ref BASE_NON_TANK_MAIN_STAT_MULTIPLIER: StatModifierType = 0.5;
    pub(crate) static ref BASE_TANK_MAIN_STAT_MULTIPLIER: StatModifierType = 0.4;
    pub(crate) static ref BASE_CRITICAL_RATE: StatModifierType = 0.05;
    pub(crate) static ref BASE_CRITICAL_DAMAGE: StatModifierType = 1.4;
    pub(crate) static ref CRIT_MULTIPLIER: StatModifierType = 200.0;
    pub(crate) static ref DIRECT_HIT_MULTIPLIER: StatModifierType = 550.0;
    pub(crate) static ref DETERMINATION_MULTIPLIER: StatModifierType = 140.0;
    pub(crate) static ref TENACITY_MULTIPLIER: StatModifierType = 100.0;
    pub(crate) static ref SPEED_MULTIPLIER: StatModifierType = 130.0;
    pub(crate) static ref UNIT_BASE: StatModifierType = 1000.0;
    pub(crate) static ref SPEED_BASE: StatModifierType = 10000.0;
    pub(crate) static ref WEAPON_ATTACK_BASE_PER_JOB: HashMap<String, i32> = HashMap::from([
        ("PLD".to_string(), 39),
        ("WAR".to_string(), 40),
        ("GNB".to_string(), 39),
        ("DRK".to_string(), 40),
        ("WHM".to_string(), 44),
        ("SCH".to_string(), 44),
        ("AST".to_string(), 44),
        ("SGE".to_string(), 44),
        ("MNK".to_string(), 42),
        ("DRG".to_string(), 44),
        ("NIN".to_string(), 42),
        ("SAM".to_string(), 43),
        ("BRD".to_string(), 44),
        ("MCH".to_string(), 44),
        ("DNC".to_string(), 44),
        ("RDM".to_string(), 44),
        ("BLM".to_string(), 44),
        ("SMN".to_string(), 44),
    ]);
    pub(crate) static ref AUTO_ATTACK_DELAYS: HashMap<String, MultiplierType> = HashMap::from([
        ("PLD".to_string(), 2.24),
        ("WAR".to_string(), 3.36),
        ("DRK".to_string(), 2.96),
        ("GNB".to_string(), 2.80),
        ("DRG".to_string(), 2.80),
        ("RPR".to_string(), 3.20),
        ("MNK".to_string(), 2.56),
        ("SAM".to_string(), 2.64),
        ("NIN".to_string(), 2.56),
        ("BRD".to_string(), 3.04),
        ("MCH".to_string(), 2.64),
        ("DNC".to_string(), 3.12)
    ]);
}

#[derive(PartialEq, Eq, Hash)]
pub enum Role {
    Tank,
    Healer,
    MeleeDps,
    PhysicalRangedDps,
    MagicRangedDps,
}

pub fn job_abbrev_to_role(job_abbrev: &String) -> Role {
    match job_abbrev.as_str() {
        "PLD" | "WAR" | "DRK" | "GNB" => Role::Tank,
        "WHM" | "SCH" | "AST" | "SGE" => Role::Healer,
        "MNK" | "DRG" | "NIN" | "SAM" | "RPR" => Role::MeleeDps,
        "BRD" | "MCH" | "DNC" => Role::PhysicalRangedDps,
        _ => Role::MagicRangedDps,
    }
}
