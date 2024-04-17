use crate::character::{get_character_main_stats, get_character_sub_stats, Character};
use crate::constants::*;
use crate::equipment::WeaponTrait;
use crate::stat::{MainStatTrait, MainStats, StatType, SubStatTrait};
use crate::{DamageMultiplierType, StatModifierType};
use crate::{DataError, Result, StatModifier};

pub struct CharacterPower {
    pub critical_strike_rate: DamageMultiplierType,
    pub critical_strike_damage: DamageMultiplierType,
    pub direct_hit_rate: DamageMultiplierType,
    pub determination_damage_multiplier: DamageMultiplierType,
    pub tenacity_damage_multiplier: DamageMultiplierType,
    pub speed_multiplier: DamageMultiplierType,
    pub weapon_damage_multiplier: DamageMultiplierType,
    pub main_stat_multiplier: DamageMultiplierType,
}

impl Default for CharacterPower {
    #[inline]
    fn default() -> Self {
        CharacterPower {
            critical_strike_rate: 0.0,
            critical_strike_damage: 0.0,
            direct_hit_rate: 0.0,
            determination_damage_multiplier: 0.0,
            speed_multiplier: 0.0,
            weapon_damage_multiplier: 0.0,
            main_stat_multiplier: 0.0,
            tenacity_damage_multiplier: 0.0,
        }
    }
}

#[inline]
fn is_physical_job(job_abbrev: &String) -> bool {
    MELEE_JOB_ABBREVS.contains(job_abbrev)
}

/// Get the main stat of the class depending on the job.
fn get_job_main_stat(main_stat: &MainStats, job_abbrev: &String) -> StatType {
    if FFXIV_STRENGTH_JOB_ABBREVS.contains(job_abbrev) {
        main_stat.get_strength()
    } else if FFXIV_DEXTERITY_JOB_ABBREVS.contains(job_abbrev) {
        main_stat.get_dexterity()
    } else if FFXIV_INTELLIGENCE_JOB_ABBREVS.contains(job_abbrev) {
        main_stat.get_intelligence()
    } else {
        main_stat.get_mind()
    }
}

pub fn convert_character_to_power(
    character: &Character,
    stat_modifier: StatModifier,
) -> Result<CharacterPower> {
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
        *CRIT_MULTIPLIER,
    );

    let critical_strike_rate = *BASE_CRITICAL_RATE + critical_strike_increase;
    let critical_strike_damage = *BASE_CRITICAL_DAMAGE + critical_strike_increase;

    let direct_hit_increase = get_increase(
        direct_hit_stat as StatModifierType,
        stat_modifier.max_level_base_direct_hit as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        *DIRECT_HIT_MULTIPLIER,
    );

    let direct_hit_rate = direct_hit_increase;

    let determination_increase = get_increase(
        determination_stat as StatModifierType,
        stat_modifier.max_level_base_determination as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        *DETERMINATION_MULTIPLIER,
    );

    let determination_damage_multiplier = 1.0 + determination_increase;

    let tenacity_increase = get_increase(
        tenacity_stat as StatModifierType,
        stat_modifier.max_level_base_tenacity as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        *TENACITY_MULTIPLIER,
    );
    let tenacity_damage_multiplier = 1.0 + tenacity_increase;

    let job_abbrev = &character.job.abbrev;
    let speed_stat = if is_physical_job(job_abbrev) {
        skill_speed_stat as StatModifierType
    } else {
        spell_speed_stat as StatModifierType
    };

    let speed_increase = get_increase(
        speed_stat as StatModifierType,
        stat_modifier.max_level_base_skill_speed as StatModifierType,
        stat_modifier.max_level_sub_stat_modifier,
        *SPEED_MULTIPLIER,
    );

    let speed_multiplier = 1.0 + speed_increase;

    let base_weapon_damage = WEAPON_ATTACK_BASE_PER_JOB.get(job_abbrev);
    if base_weapon_damage.is_none() {
        return Err(DataError::JobClassParseError(
            "Job class not found".to_string(),
        ));
    }
    let base_weapon_damage = base_weapon_damage.unwrap();

    let weapon_damage = if is_physical_job(job_abbrev) {
        character.get_damage_phys()
    } else {
        character.get_damage_mag()
    };

    let final_weapon_damage = weapon_damage + base_weapon_damage;

    let weapon_damage_multiplier = final_weapon_damage as f64 / 100f64;

    let main_stat_slope = if character.job.is_tank {
        *BASE_TANK_MAIN_STAT_MULTIPLIER
    } else {
        *BASE_NON_TANK_MAIN_STAT_MULTIPLIER
    };

    let main_stat = get_job_main_stat(&main_stats, job_abbrev)
        - FFXIV_STAT_MODIFIER.max_level_main_stat_modifier as StatType;
    let main_stat_multiplier = 1.0f64 + ((main_stat as f64) * main_stat_slope / 100f64);

    Ok(CharacterPower {
        critical_strike_rate,
        critical_strike_damage,
        direct_hit_rate,
        determination_damage_multiplier,
        speed_multiplier,
        weapon_damage_multiplier,
        main_stat_multiplier,
        tenacity_damage_multiplier,
    })
}

fn get_increase(
    stat: StatModifierType,
    base_stat: StatModifierType,
    modifier: StatModifierType,
    multiplier: f64,
) -> f64 {
    let multiplied = multiplier * (stat - base_stat);
    let modified = f64::floor(multiplied / modifier);
    modified / *UNIT_BASE
}

fn get_speed_multiplier(
    speed_stat: StatModifierType,
    base_speed: StatModifierType,
    max_level_sub_stat_modifier: StatModifierType,
    multiplier: f64,
) -> f64 {
    let speed_diff = base_speed - speed_stat;
    let speed_multiplier = f64::ceil(multiplier * speed_diff / max_level_sub_stat_modifier);
    (speed_multiplier + *UNIT_BASE) / *UNIT_BASE
}

impl CharacterPower {}

#[cfg(test)]
mod test {
    use crate::character::{get_character_main_stats, Character, ItemManager};
    use crate::clan::Clan;
    use crate::constants::*;
    use crate::equipment::{ArmorDefense, Equipment, WeaponDamage};
    use crate::job::Job;
    use crate::stat::{MainStats, SubStats};
    use crate::stat_calculator::convert_character_to_power;

    #[test]
    fn ninja_bis_test() {
        let weapon = Equipment {
            id: 40170,
            name: "Knives of Ascension".to_string(),
            slot_name: "weapon".to_string(),
            slot_category: 13,
            main_stats: MainStats {
                strength: 0,
                dexterity: 416,
                vitality: 458,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 306,
                determination: 214,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 0,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 66,
                damage_phys: 132,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let helmet = Equipment {
            id: 40130,
            name: "Augmented Credendum Blinder of Scouting".to_string(),
            slot_name: "head".to_string(),
            slot_category: 3,
            main_stats: MainStats {
                strength: 0,
                dexterity: 248,
                vitality: 277,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 129,
                determination: 184,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 0,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let body = Equipment {
            id: 40131,
            name: "Augmented Credendum Courselet of Scouting".to_string(),
            slot_name: "body".to_string(),
            slot_category: 4,
            main_stats: MainStats {
                strength: 0,
                dexterity: 394,
                vitality: 440,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 0,
                determination: 204,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 292,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let hand = Equipment {
            id: 40132,
            name: "Augmented Credendum Armguards of Scouting".to_string(),
            slot_name: "hand".to_string(),
            slot_category: 5,
            main_stats: MainStats {
                strength: 0,
                dexterity: 248,
                vitality: 277,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 184,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 129,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let leg = Equipment {
            id: 40133,
            name: "Augmented Credendum Trousers of Scouting".to_string(),
            slot_name: "legs".to_string(),
            slot_category: 7,
            main_stats: MainStats {
                strength: 0,
                dexterity: 394,
                vitality: 440,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 204,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 292,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let foot = Equipment {
            id: 40209,
            name: "Ascension Sabatons of Scounting".to_string(),
            slot_name: "foot".to_string(),
            slot_category: 8,
            main_stats: MainStats {
                strength: 0,
                dexterity: 248,
                vitality: 277,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 129,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 184,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let earring = Equipment {
            id: 40147,
            name: "Augmented Credendum Earrings of Aiming".to_string(),
            slot_name: "ears".to_string(),
            slot_category: 9,
            main_stats: MainStats {
                strength: 0,
                dexterity: 196,
                vitality: 218,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 102,
                determination: 145,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 0,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let necklace = Equipment {
            id: 40148,
            name: "Augmented Credendum Necklace of Aiming".to_string(),
            slot_name: "neck".to_string(),
            slot_category: 10,
            main_stats: MainStats {
                strength: 0,
                dexterity: 196,
                vitality: 218,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 0,
                determination: 0,
                skill_speed: 145,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 102,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let wrist = Equipment {
            id: 40149,
            name: "Augmented Credendum Bracelet of Aiming".to_string(),
            slot_name: "wrists".to_string(),
            slot_category: 11,
            main_stats: MainStats {
                strength: 0,
                dexterity: 196,
                vitality: 218,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 102,
                determination: 145,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 0,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let ring1 = Equipment {
            id: 40150,
            name: "Augmented Credendum Ring of Aiming".to_string(),
            slot_name: "ring".to_string(),
            slot_category: 12,
            main_stats: MainStats {
                strength: 0,
                dexterity: 196,
                vitality: 218,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 145,
                determination: 102,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 0,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let ring2 = Equipment {
            id: 40151,
            name: "Ascension Ring of Aiming".to_string(),
            slot_name: "ring".to_string(),
            slot_category: 12,
            main_stats: MainStats {
                strength: 0,
                dexterity: 196,
                vitality: 218,
                intelligence: 0,
                mind: 0,
            },
            sub_stats: SubStats {
                critical_strike: 102,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 145,
                piety: 0,
            },
            materia_slot: vec![None; 2],
            materia_slot_count: 2,
            weapon_damage: WeaponDamage {
                damage_mag: 0,
                damage_phys: 0,
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            equipable_jobs: vec!["NIN".to_string()],
            pentameldable: false,
        };

        let mut character = Character {
            clan: Clan {
                id: 7,
                name: "Seeker of the Sun".to_string(),
                main_stats: MainStats {
                    strength: 2,
                    dexterity: 3,
                    vitality: 0,
                    intelligence: -1,
                    mind: -1,
                },
            },
            job: Job {
                id: 25,
                name: "Ninja".to_string(),
                abbrev: "NIN".to_string(),
                base_main_stats: MainStats {
                    strength: 85,
                    dexterity: 110,
                    vitality: 100,
                    intelligence: 65,
                    mind: 75,
                },
                base_hp: 108,
                is_tank: false,
            },
            equipments: vec![None; 15],
            food: None,
            medicine: None,
        };

        assert_eq!(get_character_main_stats(&character).dexterity, 432);

        assert!(character.equip_or_replace(weapon, *WEAPON_SLOT).is_ok());
        assert!(character.equip_or_replace(helmet, *HEAD_SLOT).is_ok());
        assert!(character.equip_or_replace(body, *BODY_SLOT).is_ok());
        assert!(character.equip_or_replace(hand, *HANDS_SLOT).is_ok());
        assert!(character.equip_or_replace(leg, *LEGS_SLOT).is_ok());
        assert!(character.equip_or_replace(foot, *FEET_SLOT).is_ok());
        assert!(character.equip_or_replace(earring, *EARS_SLOT).is_ok());
        assert!(character.equip_or_replace(necklace, *NECK_SLOT).is_ok());
        assert!(character.equip_or_replace(wrist, *WRISTS_SLOT).is_ok());
        assert!(character
            .equip_or_replace(ring1.clone(), *RING_SLOT)
            .is_ok());
        assert!(character.equip_or_replace(ring2, *RING_SLOT2).is_ok());

        let power = convert_character_to_power(&character, *FFXIV_STAT_MODIFIER).unwrap();

        assert_eq!(power.critical_strike_rate, 0.197f64);
        assert_eq!(power.critical_strike_damage, 1.547f64);
        assert_eq!(power.direct_hit_rate, 0.331f64);
        assert_eq!(power.determination_damage_multiplier, 1.073f64);
        assert_eq!(power.speed_multiplier, 1.009f64);
        assert_eq!(power.main_stat_multiplier, 15.85f64);
        assert_eq!(power.weapon_damage_multiplier, 1.74f64);
    }
}
