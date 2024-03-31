/// Saves functions needed to implement food data
/// for FFXIV Simbot.
use crate::stat::{MainStatTrait, StatType, SubStatTrait, SubStats};
use crate::{item_vec_to_id_table, IdTable, JsonFileReader, Result, SearchKeyEntity};
use itertools::Itertools;
use serde::Deserialize;
use std::path::PathBuf;

type FoodId = usize;
pub type FoodTable = IdTable<FoodId, Food>;

#[derive(Eq, PartialEq, Deserialize)]
struct EtroFood {
    id: FoodId,
    name: String,
    #[serde(rename = "Piety")]
    piety: StatType,
    #[serde(rename = "Tenacity")]
    tenacity: StatType,
    #[serde(rename = "Direct Hit Rate")]
    direct_hit: StatType,
    #[serde(rename = "Critical Hit")]
    critical_hit: StatType,
    #[serde(rename = "Determination")]
    determination: StatType,
    #[serde(rename = "Skill Speed")]
    skill_speed: StatType,
    #[serde(rename = "Spell Speed")]
    spell_speed: StatType,
    #[serde(rename = "Vitality")]
    vitality: StatType,
}

/// Buff Food Data for FFXIV Simbot.
/// Only treat Combat Foods as of now.
#[derive(Clone, Eq, PartialEq)]
pub struct Food {
    id: FoodId,
    name: String,
    sub_stats: SubStats,
    vitality: StatType,
}

impl MainStatTrait for Food {
    fn get_strength(&self) -> StatType {
        0
    }

    fn get_dexterity(&self) -> StatType {
        0
    }

    fn get_vitality(&self) -> StatType {
        self.vitality
    }

    fn get_intelligence(&self) -> StatType {
        0
    }

    fn get_mind(&self) -> StatType {
        0
    }
}

impl SubStatTrait for Food {
    fn get_critical_strike(&self) -> StatType {
        self.sub_stats.get_critical_strike()
    }

    fn get_direct_hit(&self) -> StatType {
        self.sub_stats.get_direct_hit()
    }

    fn get_determination(&self) -> StatType {
        self.sub_stats.get_determination()
    }

    fn get_skill_speed(&self) -> StatType {
        self.sub_stats.get_skill_speed()
    }

    fn get_spell_speed(&self) -> StatType {
        self.sub_stats.get_spell_speed()
    }

    fn get_tenacity(&self) -> StatType {
        self.sub_stats.get_tenacity()
    }

    fn get_piety(&self) -> StatType {
        self.sub_stats.get_piety()
    }
}

impl SearchKeyEntity<FoodId> for Food {
    fn get_search_key(&self) -> Vec<FoodId> {
        vec![self.id]
    }
}

pub struct FoodFactory {}
impl JsonFileReader for FoodFactory {}

impl FoodFactory {
    pub fn new() -> Self {
        FoodFactory {}
    }

    /// parse jobs_data.json file into Job usable in the engine.
    pub fn parse_food_json_file(
        &self,
        data_directory: &PathBuf,
        file_path: &str,
    ) -> Result<FoodTable> {
        let data = self.read_json_file(&data_directory.join(file_path))?;
        let etro_food: Vec<EtroFood> = serde_json::from_str(data.as_str())?;

        let food = etro_food
            .into_iter()
            .map(|etro_food| self.convert_to_food(etro_food))
            .collect_vec();

        Ok(item_vec_to_id_table(food))
    }

    fn convert_to_food(&self, etro_food: EtroFood) -> Food {
        Food {
            id: etro_food.id,
            name: etro_food.name.clone(),
            sub_stats: SubStats {
                critical_strike: etro_food.critical_hit,
                direct_hit: etro_food.direct_hit,
                determination: etro_food.determination,
                skill_speed: etro_food.skill_speed,
                spell_speed: etro_food.spell_speed,
                tenacity: etro_food.tenacity,
                piety: etro_food.piety,
            },
            vitality: etro_food.vitality,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::food::*;

    #[test]
    fn food_basic_test() {
        let food = Food {
            id: 1,
            name: "Baked Onion Soup".to_string(),
            sub_stats: SubStats {
                direct_hit: 1,
                critical_strike: 2,
                determination: 3,
                skill_speed: 4,
                spell_speed: 5,
                tenacity: 6,
                piety: 7,
            },
            vitality: 15,
        };

        assert_eq!(food.get_direct_hit(), 1);
        assert_eq!(food.get_critical_strike(), 2);
        assert_eq!(food.get_determination(), 3);
        assert_eq!(food.get_skill_speed(), 4);
        assert_eq!(food.get_spell_speed(), 5);
        assert_eq!(food.get_tenacity(), 6);
        assert_eq!(food.get_piety(), 7);
        assert_eq!(food.get_vitality(), 15);
    }

    #[test]
    fn food_search_table_test() {
        let foods = vec![
            Food {
                id: 421,
                name: "Masala Chai".into(),
                sub_stats: SubStats {
                    critical_strike: 33,
                    direct_hit: 0,
                    determination: 0,
                    skill_speed: 0,
                    spell_speed: 0,
                    tenacity: 0,
                    piety: 54,
                },
                vitality: 58,
            },
            Food {
                id: 121,
                name: "La Noscean Toast".into(),
                sub_stats: SubStats {
                    critical_strike: 12,
                    direct_hit: 0,
                    determination: 0,
                    skill_speed: 0,
                    spell_speed: 23,
                    tenacity: 0,
                    piety: 0,
                },
                vitality: 14,
            },
            Food {
                id: 140,
                name: "Mugwort Carp".into(),
                sub_stats: SubStats {
                    critical_strike: 8,
                    direct_hit: 12,
                    determination: 0,
                    skill_speed: 0,
                    spell_speed: 0,
                    tenacity: 0,
                    piety: 0,
                },
                vitality: 10,
            },
            Food {
                id: 525,
                name: "Spicy Shakshouka".into(),
                sub_stats: SubStats {
                    critical_strike: 0,
                    direct_hit: 0,
                    determination: 45,
                    skill_speed: 0,
                    spell_speed: 75,
                    tenacity: 0,
                    piety: 0,
                },
                vitality: 81,
            },
        ];

        let food_table = item_vec_to_id_table(foods);

        assert_eq!(food_table.get(&421).unwrap().name, "Masala Chai");
        assert_eq!(food_table.get(&121).unwrap().name, "La Noscean Toast");
        assert_eq!(food_table.get(&140).unwrap().name, "Mugwort Carp");
        assert_eq!(food_table.get(&525).unwrap().name, "Spicy Shakshouka");
    }
}
