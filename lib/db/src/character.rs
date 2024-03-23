use crate::character;
use crate::clan::Clan;
/// Implements features needed to represent a Character in FFXIV Simbot.
use crate::equipment::Equipment;
use crate::food::Food;
use crate::job::Job;
use crate::medicine::Medicine;
use crate::stat::{add_main_stats, add_sub_stats, MainStats, SubStats};

struct Character {
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

/// Get the Final Main Stats of the Character
/// Main Stat(Character) = Main Stat(Job) + Main Stat(Equipment) + Main Stat(Medicine)
///                        + Main Stat(Clan)
pub fn get_character_main_stats(character: &Character) -> MainStats {
    let mut main_stats = MainStats::from(&character.job);

    for equipment in &character.equipments {
        if let Some(equipment) = equipment {
            main_stats = add_main_stats(&main_stats, &equipment);
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
    let mut sub_stats = SubStats::from(&character.job);

    for equipment in &character.equipments {
        if let Some(equipment) = equipment {
            sub_stats = add_sub_stats(&sub_stats, &equipment);
        }
    }

    if let Some(food) = &character.food {
        sub_stats = add_sub_stats(&sub_stats, food);
    }

    sub_stats
}
