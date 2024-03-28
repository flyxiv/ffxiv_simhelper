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

/*
#[cfg(test)]
mod tests {
    #[test]
    fn test_get_character_default_main_stats() {
}
*/
