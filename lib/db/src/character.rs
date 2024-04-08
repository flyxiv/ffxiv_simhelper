use crate::clan::Clan;
use crate::constants::*;
/// Implements features needed to represent a Character in FFXIV Simbot.
use crate::equipment::{Equipment, SlotType, WeaponTrait};
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
    pub(crate) clan: Clan,
    pub(crate) job: Job,
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
    pub(crate) equipments: Vec<Option<Equipment>>,
    pub(crate) food: Option<Food>,
    pub(crate) medicine: Option<Medicine>,
}

/// Defines actions of a character to equip/unequip items.
pub trait ItemManager {
    /// Equip an item to the character.
    /// If there is already another equipment in the slot, replace it.
    fn equip_or_replace(&mut self, item: Equipment, slot: SlotType) -> Result<()>;

    /// Unequip an item from the character.
    fn unequip(&mut self, slot: SlotType) -> Result<()>;
}

impl WeaponTrait for Character {
    fn get_damage_mag(&self) -> usize {
        let equipped_weapon = &self.equipments[*WEAPON_SLOT];

        if equipped_weapon.is_none() {
            0
        } else {
            let equipped_weapon = equipped_weapon.as_ref().unwrap();
            equipped_weapon.get_damage_mag()
        }
    }

    fn get_damage_phys(&self) -> usize {
        let equipped_weapon = &self.equipments[*WEAPON_SLOT];

        if equipped_weapon.is_none() {
            0
        } else {
            let equipped_weapon = equipped_weapon.as_ref().unwrap();
            equipped_weapon.get_damage_phys()
        }
    }
}

impl ItemManager for Character {
    fn equip_or_replace(&mut self, item: Equipment, slot: SlotType) -> Result<()> {
        if slot > *EQUIPMENT_SLOTS {
            return Err(DataError::EquipError(slot));
        }

        if slot == *RING_SLOT {
            self.equip_ring_if_not_duplicate(item, *RING_SLOT, *RING_SLOT2)
        } else if slot == *RING_SLOT2 {
            self.equip_ring_if_not_duplicate(item, *RING_SLOT2, *RING_SLOT)
        } else {
            self.equipments[slot] = Some(item);
            Ok(())
        }
    }

    fn unequip(&mut self, slot: SlotType) -> Result<()> {
        if slot > self.equipments.len() {
            return Err(DataError::UnEquipError(slot));
        }

        self.equipments[slot] = None;
        Ok(())
    }
}

impl Character {
    fn equip_ring_if_not_duplicate(
        &mut self,
        item: Equipment,
        current_ring_slot: usize,
        other_ring_slot: SlotType,
    ) -> Result<()> {
        if let Some(other_ring) = &self.equipments[other_ring_slot] {
            if item.id == other_ring.id {
                return Err(DataError::EquipError(current_ring_slot));
            }
        }

        self.equipments[current_ring_slot] = Some(item);
        Ok(())
    }

    pub fn eat_food(&mut self, food: Food) {
        self.food = Some(food);
    }

    pub fn eat_medicine(&mut self, medicine: Medicine) {
        self.medicine = Some(medicine);
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
    use crate::character::{
        get_character_main_stats, get_character_sub_stats, Character, ItemManager, BODY_SLOT,
        EARS_SLOT, FEET_SLOT, HANDS_SLOT, HEAD_SLOT, LEGS_SLOT, NECK_SLOT, RING_SLOT, RING_SLOT2,
        WEAPON_SLOT, WRISTS_SLOT,
    };
    use crate::clan::Clan;
    use crate::equipment::{ArmorDefense, Equipment, WeaponDamage};
    use crate::food::Food;
    use crate::job::Job;
    use crate::medicine::Medicine;
    use crate::stat::{MainStats, SubStatTrait, SubStats};

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
        assert!(character
            .equip_or_replace(ring1.clone(), *RING_SLOT2)
            .is_err());
        assert!(character.equip_or_replace(ring1, 15).is_err());

        let food = Food {
            id: 1,
            name: "Baked Eggplant".to_string(),
            sub_stats: SubStats {
                critical_strike: 62,
                determination: 103,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                direct_hit: 0,
                piety: 0,
            },
            vitality: 143,
        };

        let medicine = Medicine {
            main_stats: MainStats {
                strength: 0,
                dexterity: 262,
                vitality: 0,
                intelligence: 0,
                mind: 0,
            },
            duration: 0,
        };

        character.eat_food(food);
        assert_eq!(get_character_main_stats(&character).dexterity, 3360);

        let sub_stats = get_character_sub_stats(&character);
        assert_eq!(sub_stats.get_critical_strike(), 2010);
        assert_eq!(sub_stats.get_determination(), 1589);
        assert_eq!(sub_stats.get_spell_speed(), 400);
        assert_eq!(sub_stats.get_spell_speed(), 400);
        assert_eq!(sub_stats.get_tenacity(), 400);
        assert_eq!(sub_stats.get_direct_hit(), 1442);
        assert_eq!(sub_stats.get_piety(), 390);

        character.eat_medicine(medicine);
        assert_eq!(get_character_main_stats(&character).dexterity, 3622);
    }
}
