use crate::clan::Clan;
/// Implements features needed to represent a Character in FFXIV Simbot.
use crate::equipment::Equipment;
use crate::food::Food;
use crate::job::Job;
use crate::medicine::Medicine;
use crate::stat::{add_main_stats, add_sub_stats, MainStats, StatFrom, SubStats};
use crate::{DataError, Result};

/// Data for a single Character in FFXIV Simbot.
/// Combat Data for Characters in FFXIV include:
/// 1. Current Clan(Race)
/// 2. Current Job
/// 3. Equipments
/// 4. Food
/// 5. Medicine
pub struct Character {
    clan: Clan,
    job: Job,
    /// 0: Weapon
    /// 1: Head
    /// 2: Body
    /// 3: Hands
    /// 4: Legs
    /// 5: Feet
    /// 6: Ears
    /// 7: Neck
    /// 8: Wrists
    /// 9: Ring
    /// 10: Ring
    equipments: Vec<Option<Equipment>>,
    food: Option<Food>,
    medicine: Option<Medicine>,
}

/// Defines actions of a character to equip/unequip items.
pub trait ItemManager {
    /// Equip an item to the character.
    /// If there is already another equipment in the slot, replace it.
    fn equip_or_replace(&mut self, item: Equipment) -> Result<()>;

    /// Unequip an item from the character.
    fn unequip(&mut self, slot: usize) -> Result<()>;
}

impl ItemManager for Character {
    fn equip_or_replace(&mut self, item: Equipment) -> Result<()> {
        let slot = item.slot_category;

        if slot > self.equipments.len() {
            return Err(DataError::EquipError(slot));
        }

        self.equipments[slot] = Some(item);
        Ok(())
    }

    fn unequip(&mut self, slot: usize) -> Result<()> {
        if slot > self.equipments.len() {
            return Err(DataError::UnEquipError(slot));
        }

        self.equipments[slot] = None;
        Ok(())
    }
}

/// Get the Final Main Stats of the Character
/// Main Stat(Character) = Main Stat(Job) + Main Stat(Equipment) + Main Stat(Medicine)
///                        + Main Stat(Clan)
pub fn get_character_main_stats(character: &Character) -> MainStats {
    let mut main_stats = MainStats::stat_from(&character.job);

    for equipment in &character.equipments {
        if let Some(equipment) = equipment {
            main_stats = add_main_stats(&main_stats, equipment);
        }
    }

    main_stats = add_main_stats(&main_stats, &character.clan);

    if let Some(medicine) = &character.medicine {
        main_stats = add_main_stats(&main_stats, medicine);
    }

    main_stats
}

/// Get the Final Sub Stats of the Character
/// Sub Stat(Character) = Sub Stat(Job) + Sub Stat(Equipment) + Sub Stat(Food)
pub fn get_character_sub_stats(character: &Character) -> SubStats {
    let mut sub_stats = SubStats::stat_from(&character.job);

    for equipment in &character.equipments {
        if let Some(equipment) = equipment {
            sub_stats = add_sub_stats(&sub_stats, equipment);
        }
    }

    if let Some(food) = &character.food {
        sub_stats = add_sub_stats(&sub_stats, food);
    }

    sub_stats
}

#[cfg(test)]
mod tests {
    use crate::character::Character;
    use crate::clan::Clan;
    use crate::equipment::{ArmorDefense, Equipment, WeaponDamage};
    use crate::job::tests::get_test_stat_modifier;
    use crate::job::Job;
    use crate::stat::{MainStats, SubStats};

    #[test]
    fn test_get_character_stats() {
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

        // TODO:
        // 1) equip armor
        // 2) equip food
        // 3) equip medicine
        // 4) equip viable materia to equipment
        // 5) equip viable materia to equipment where the stat must be capped

        let character = Character {
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
                name: "Black Mage".to_string(),
                abbrev: "BLM".to_string(),
                base_main_stats: MainStats {
                    strength: 45,
                    dexterity: 100,
                    vitality: 100,
                    intelligence: 115,
                    mind: 75,
                },
                base_hp: 105,
                stat_modifier: get_test_stat_modifier(),
                is_tank: false,
            },
            equipments: vec![],
            food: None,
            medicine: None,
        };
    }
}
