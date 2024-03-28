/// Saves functions needed to implement food data
/// for FFXIV Simbot.
use crate::stat::{StatType, SubStatTrait, SubStats};
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
}

/// Buff Food Data for FFXIV Simbot.
/// Only treat Combat Foods as of now.
#[derive(Clone, Eq, PartialEq)]
pub struct Food {
    id: FoodId,
    name: String,
    sub_stats: SubStats,
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
        }
    }
}
