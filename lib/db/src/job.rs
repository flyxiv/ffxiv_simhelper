/// Implements functions needed to save Job data
/// in FFXIV Simbot.
/// Only save combat jobs as of now.
use crate::stat::{MainStatTrait, MainStats, SpecialStatTrait};
use crate::{JsonFileReader, Result};
use itertools::Itertools;
use serde::Deserialize;

/// Crude Job Data fetched from Etro
/// Gets converted to JobCLass, the engine's preferred type for the corresponding entity.
#[derive(Deserialize, Clone)]
pub struct EtroJob {
    id: usize,
    abbrev: String,
    name: String,
    #[serde(rename = "Strength")]
    strength: usize,
    #[serde(rename = "Dexterity")]
    dexterity: usize,
    #[serde(rename = "Vitality")]
    vitality: usize,
    #[serde(rename = "Intelligence")]
    intelligence: usize,
    #[serde(rename = "Mind")]
    mind: usize,
    #[serde(rename = "Hp")]
    hp: usize,
}

#[derive(PartialEq, Copy, Clone)]
struct StatModifier {
    max_level_main_stat_modifier: f64,
    max_level_sub_stat_modifier: f64,
    max_level_hp_modifier: f64,
    max_level_div: f64,
    hp_per_vitality_non_tank: f64,
    hp_per_vitality_tank: f64,
}

#[derive(PartialEq, Clone)]
pub(crate) struct Job {
    id: usize,
    abbrev: String,
    name: String,
    base_main_stats: MainStats,
    base_hp: usize,
    // https://www.akhmorning.com/allagan-studies/modifiers/levelmods/
    // base stats are determined by base * level stat modifier
    stat_modifier: StatModifier,
}

// TODO: get from LUA config file
fn make_stat_modifier() -> StatModifier {
    StatModifier {
        max_level_main_stat_modifier: 390f64,
        max_level_sub_stat_modifier: 400f64,
        max_level_hp_modifier: 1900f64,
        max_level_div: 3000f64,
        hp_per_vitality_non_tank: 22.1f64,
        hp_per_vitality_tank: 31.5f64,
    }
}

fn calculate_modified_stat(base_stat: usize, modifier: f64) -> usize {
    (base_stat as f64 * modifier / 100f64).floor() as usize
}

impl MainStatTrait for Job {
    //https://finalfantasy.fandom.com/wiki/Final_Fantasy_XIV_attributes
    fn get_strength(&self) -> usize {
        calculate_modified_stat(
            self.base_main_stats.strength,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }

    fn get_dexterity(&self) -> usize {
        calculate_modified_stat(
            self.base_main_stats.dexterity,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }

    fn get_intelligence(&self) -> usize {
        calculate_modified_stat(
            self.base_main_stats.intelligence,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }

    fn get_mind(&self) -> usize {
        calculate_modified_stat(
            self.base_main_stats.mind,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }

    fn get_vitality(&self) -> usize {
        calculate_modified_stat(
            self.base_main_stats.vitality,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }
}

impl SpecialStatTrait for Job {
    fn get_hp(&self) -> usize {
        calculate_modified_stat(self.base_hp, self.stat_modifier.max_level_hp_modifier)
    }
}

impl From<EtroJob> for MainStats {
    fn from(etro_job: EtroJob) -> Self {
        MainStats {
            strength: etro_job.strength,
            dexterity: etro_job.dexterity,
            vitality: etro_job.vitality,
            intelligence: etro_job.intelligence,
            mind: etro_job.mind,
        }
    }
}

struct JobFactory {}
impl JsonFileReader for JobFactory {}

impl JobFactory {
    /// parse jobs_data.json file into Job usable in the engine.
    pub fn parse_jobs_json_file(&self, file_path: &str) -> Result<Vec<Job>> {
        let data = self.read_json_file(file_path)?;
        let etro_jobs: Vec<EtroJob> = serde_json::from_str(data.as_str())?;

        let jobs = etro_jobs
            .into_iter()
            .map(|etro_job| self.convert_to_job(etro_job))
            .collect_vec();

        Ok(jobs)
    }

    fn convert_to_job(&self, etro_job: EtroJob) -> Job {
        Job {
            id: etro_job.id,
            abbrev: etro_job.abbrev.clone(),
            name: etro_job.name.clone(),
            base_hp: etro_job.hp,
            base_main_stats: etro_job.into(),
            stat_modifier: make_stat_modifier(),
        }
    }
}
