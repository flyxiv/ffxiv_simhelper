/// Saves functions needed to implement food data
/// for FFXIV Simbot.
use crate::stat::{SubStatTrait, SubStats};
use crate::{JsonFileReader, Result};
use itertools::Itertools;

struct EtroFood {
    id: usize,
    name: String,
    #[serde(rename = "Piety")]
    piety: usize,
    #[serde(rename = "Tenacity")]
    tenacity: usize,
    #[serde(rename = "Direct Hit Rate")]
    direct_hit: usize,
    #[serde(rename = "Critical Hit")]
    critical_hit: usize,
    #[serde(rename = "Determination")]
    determination: usize,
    #[serde(rename = "Skill Speed")]
    skill_speed: usize,
    #[serde(rename = "Spell Speed")]
    spell_speed: usize,
}

/// Buff Food Data for FFXIV Simbot.
/// Only treat Combat Foods as of now.
pub struct Food {
    id: usize,
    name: String,
    sub_stats: SubStats,
}

impl SubStatTrait for Food {
    fn get_critical_strike(&self) -> usize {
        self.sub_stats.get_critical_strike()
    }

    fn get_direct_hit(&self) -> usize {
        self.sub_stats.get_direct_hit()
    }

    fn get_determination(&self) -> usize {
        self.sub_stats.get_determination()
    }

    fn get_skill_speed(&self) -> usize {
        self.sub_stats.get_skill_speed()
    }

    fn get_spell_speed(&self) -> usize {
        self.sub_stats.get_spell_speed()
    }

    fn get_tenacity(&self) -> usize {
        self.sub_stats.get_tenacity()
    }

    fn get_piety(&self) -> usize {
        self.sub_stats.get_piety()
    }
}

struct FoodFactory {}
impl JsonFileReader for FoodFactory {}

impl FoodFactory {
    /// parse jobs_data.json file into Job usable in the engine.
    pub fn parse_food_json_file(&self, file_path: &str) -> Result<Vec<Food>> {
        let data = self.read_json_file(file_path)?;
        let etro_clans: Vec<Food> = serde_json::from_str(data.as_str())?;

        let jobs = etro_clans
            .into_iter()
            .map(|etro_clan| self.convert_to_job(etro_clan))
            .collect_vec();

        Ok(jobs)
    }

    fn convert_to_food(&self, etro_food: EtroFood) -> Food {
        Food {
            id: etro_food.id,
            name: etro_food.name.clone(),
            sub_stats: etro_food.into(),
        }
    }
}
