/// Implements functions needed to save Job data
/// in FFXIV Simbot.
/// Only save combat jobs as of now.
use crate::stat::{HpType, MainStatTrait, MainStats, SpecialStatTrait, StatType, SubStatTrait};
use crate::{item_vec_to_id_table, IdTable, JsonFileReader, Result, SearchKeyEntity};
use itertools::Itertools;
use serde::Deserialize;
use std::path::PathBuf;

/// Type for Stat modifiers.
pub type StatModifierType = f64;
pub type JobAbbrevType = String;
pub(crate) type JobId = usize;
/// Equipment Searches Jobs by its Abbrev, so key has to be Abbrev for jobs.
pub type JobTable = IdTable<JobAbbrevType, Job>;

/// Crude Job Data fetched from Etro
/// Gets converted to JobCLass, the engine's preferred type for the corresponding entity.
#[derive(Deserialize, Clone)]
pub struct EtroJob {
    id: JobId,
    abbrev: JobAbbrevType,
    name: String,
    #[serde(rename = "Strength")]
    strength: StatType,
    #[serde(rename = "Dexterity")]
    dexterity: StatType,
    #[serde(rename = "Vitality")]
    vitality: StatType,
    #[serde(rename = "Intelligence")]
    intelligence: StatType,
    #[serde(rename = "Mind")]
    mind: StatType,
    #[serde(rename = "Hp")]
    hp: HpType,
}

/// Saves Base Constants Needed for getting Job Attributes for Stats
/// https://www.akhmorning.com/allagan-studies/modifiers/levelmods/
#[derive(PartialEq, Copy, Clone)]
pub struct StatModifier {
    pub max_level_main_stat_modifier: StatModifierType,
    pub max_level_base_piety: StatType,
    pub max_level_base_direct_hit: StatType,
    pub max_level_base_critical_hit: StatType,
    pub max_level_base_determination: StatType,
    pub max_level_base_skill_speed: StatType,
    pub max_level_base_spell_speed: StatType,
    pub max_level_base_tenacity: StatType,
    pub max_level_hp_modifier: StatModifierType,
    pub max_level_div: StatModifierType,
    pub hp_per_vitality_non_tank: StatModifierType,
    pub hp_per_vitality_tank: StatModifierType,
}

/// Job for FFXIV Simbot.
/// Only treat Combat Jobs as of now.
#[derive(Clone)]
pub struct Job {
    id: JobId,
    abbrev: String,
    name: String,
    base_main_stats: MainStats,
    base_hp: usize,
    // https://www.akhmorning.com/allagan-studies/modifiers/levelmods/
    // base stats are determined by base * level stat modifier
    stat_modifier: StatModifier,
}

impl PartialEq for Job {
    // Job is only equal if job id is equal.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Job {}

fn calculate_modified_stat(base_stat: i32, modifier: f64) -> i32 {
    (base_stat as f64 * modifier / 100f64).floor() as i32
}

impl MainStatTrait for Job {
    //https://finalfantasy.fandom.com/wiki/Final_Fantasy_XIV_attributes
    fn get_strength(&self) -> i32 {
        calculate_modified_stat(
            self.base_main_stats.strength,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }

    fn get_dexterity(&self) -> i32 {
        calculate_modified_stat(
            self.base_main_stats.dexterity,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }

    fn get_vitality(&self) -> i32 {
        calculate_modified_stat(
            self.base_main_stats.vitality,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }

    fn get_intelligence(&self) -> i32 {
        calculate_modified_stat(
            self.base_main_stats.intelligence,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }

    fn get_mind(&self) -> i32 {
        calculate_modified_stat(
            self.base_main_stats.mind,
            self.stat_modifier.max_level_main_stat_modifier,
        )
    }
}

impl SubStatTrait for Job {
    fn get_critical_strike(&self) -> i32 {
        self.stat_modifier.max_level_base_critical_hit
    }

    fn get_direct_hit(&self) -> i32 {
        self.stat_modifier.max_level_base_direct_hit
    }

    fn get_determination(&self) -> i32 {
        self.stat_modifier.max_level_base_determination
    }

    fn get_piety(&self) -> i32 {
        self.stat_modifier.max_level_base_piety
    }

    fn get_skill_speed(&self) -> i32 {
        self.stat_modifier.max_level_base_skill_speed
    }

    fn get_spell_speed(&self) -> i32 {
        self.stat_modifier.max_level_base_spell_speed
    }

    fn get_tenacity(&self) -> i32 {
        self.stat_modifier.max_level_base_tenacity
    }
}

impl SpecialStatTrait for Job {
    fn get_hp(&self) -> usize {
        calculate_modified_stat(
            self.base_hp as StatType,
            self.stat_modifier.max_level_hp_modifier,
        ) as usize
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

impl SearchKeyEntity<JobAbbrevType> for Job {
    fn get_search_key(&self) -> Vec<JobAbbrevType> {
        vec![self.abbrev.clone()]
    }
}

pub struct JobFactory {}
impl JsonFileReader for JobFactory {}

impl JobFactory {
    pub fn new() -> Self {
        JobFactory {}
    }

    /// parse jobs_data.json file into Job usable in the engine.
    pub fn parse_jobs_json_file(
        &self,
        stat_modifier: StatModifier,
        data_directory: &PathBuf,
        file_path: &str,
    ) -> Result<JobTable> {
        let data = self.read_json_file(&data_directory.join(file_path))?;
        let etro_jobs: Vec<EtroJob> = serde_json::from_str(data.as_str())?;

        let jobs = etro_jobs
            .into_iter()
            .map(|etro_job| self.convert_to_job(etro_job, stat_modifier))
            .collect_vec();

        Ok(item_vec_to_id_table(jobs))
    }

    fn convert_to_job(&self, etro_job: EtroJob, stat_modifier: StatModifier) -> Job {
        Job {
            id: etro_job.id,
            abbrev: etro_job.abbrev.clone(),
            name: etro_job.name.clone(),
            base_hp: etro_job.hp,
            base_main_stats: etro_job.into(),
            stat_modifier: stat_modifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::job::{Job, StatModifier};
    use crate::stat::{MainStatTrait, MainStats, SpecialStatTrait, SubStatTrait};

    #[test]
    fn job_basic_test() {
        let job = Job {
            id: 21,
            abbrev: "WAR".to_string(),
            name: "Warrior".to_string(),
            base_main_stats: MainStats::new(105, 95, 110, 40, 55),
            base_hp: 145,
            stat_modifier: StatModifier {
                max_level_main_stat_modifier: 390f64,
                max_level_base_piety: 390,
                max_level_base_direct_hit: 400,
                max_level_base_critical_hit: 400,
                max_level_base_determination: 400,
                max_level_base_skill_speed: 400,
                max_level_base_spell_speed: 400,
                max_level_base_tenacity: 400,
                max_level_hp_modifier: 1900f64,
                max_level_div: 3000f64,
                hp_per_vitality_non_tank: 22.1f64,
                hp_per_vitality_tank: 32.5f64,
            },
        };
    }

    #[test]
    fn job_warrior_base_stat_calculation_test() {
        let job = Job {
            id: 21,
            abbrev: "WAR".to_string(),
            name: "Warrior".to_string(),
            base_main_stats: MainStats::new(105, 95, 110, 40, 55),
            base_hp: 145,
            stat_modifier: StatModifier {
                max_level_main_stat_modifier: 390f64,
                max_level_base_piety: 390,
                max_level_base_direct_hit: 400,
                max_level_base_critical_hit: 400,
                max_level_base_determination: 400,
                max_level_base_skill_speed: 400,
                max_level_base_spell_speed: 400,
                max_level_base_tenacity: 400,
                max_level_hp_modifier: 1900f64,
                max_level_div: 3000f64,
                hp_per_vitality_non_tank: 22.1f64,
                hp_per_vitality_tank: 32.5f64,
            },
        };

        assert_eq!(409, job.get_strength());
        assert_eq!(429, job.get_vitality());
        assert_eq!(400, job.get_critical_strike());
        assert_eq!(400, job.get_determination());
        assert_eq!(400, job.get_direct_hit());
        assert_eq!(400, job.get_skill_speed());
        assert_eq!(400, job.get_spell_speed());
        assert_eq!(400, job.get_tenacity());
        assert_eq!(390, job.get_piety());
        assert_eq!(5699, job.get_hp());
    }
    #[test]
    fn job_bard_base_stat_calculation_test() {
        let job = Job {
            id: 23,
            abbrev: "BRD".to_string(),
            name: "BARD".to_string(),
            base_main_stats: MainStats::new(90, 115, 100, 85, 80),
            base_hp: 105,
            stat_modifier: StatModifier {
                max_level_main_stat_modifier: 390f64,
                max_level_base_piety: 390,
                max_level_base_direct_hit: 400,
                max_level_base_critical_hit: 400,
                max_level_base_determination: 400,
                max_level_base_skill_speed: 400,
                max_level_base_spell_speed: 400,
                max_level_base_tenacity: 400,
                max_level_hp_modifier: 1900f64,
                max_level_div: 3000f64,
                hp_per_vitality_non_tank: 22.1f64,
                hp_per_vitality_tank: 32.5f64,
            },
        };

        assert_eq!(448, job.get_dexterity());
        assert_eq!(390, job.get_vitality());
        assert_eq!(400, job.get_critical_strike());
        assert_eq!(400, job.get_determination());
        assert_eq!(400, job.get_direct_hit());
        assert_eq!(400, job.get_skill_speed());
        assert_eq!(400, job.get_spell_speed());
        assert_eq!(400, job.get_tenacity());
        assert_eq!(390, job.get_piety());
        assert_eq!(3150, job.get_hp());
    }

    #[test]
    fn job_red_mage_base_stat_calculation_test() {
        let job = Job {
            id: 35,
            abbrev: "RDM".to_string(),
            name: "Red Mage".to_string(),
            base_main_stats: MainStats::new(55, 105, 100, 115, 110),
            base_hp: 105,
            stat_modifier: StatModifier {
                max_level_main_stat_modifier: 390f64,
                max_level_base_piety: 390,
                max_level_base_direct_hit: 400,
                max_level_base_critical_hit: 400,
                max_level_base_determination: 400,
                max_level_base_skill_speed: 400,
                max_level_base_spell_speed: 400,
                max_level_base_tenacity: 400,
                max_level_hp_modifier: 1900f64,
                max_level_div: 3000f64,
                hp_per_vitality_non_tank: 22.1f64,
                hp_per_vitality_tank: 32.5f64,
            },
        };

        assert_eq!(448, job.get_intelligence());
        assert_eq!(390, job.get_vitality());
        assert_eq!(400, job.get_critical_strike());
        assert_eq!(400, job.get_determination());
        assert_eq!(400, job.get_direct_hit());
        assert_eq!(400, job.get_skill_speed());
        assert_eq!(400, job.get_spell_speed());
        assert_eq!(400, job.get_tenacity());
        assert_eq!(390, job.get_piety());
        assert_eq!(3150, job.get_hp());
    }

    #[test]
    fn job_sch_base_stat_calculation_test() {
        let job = Job {
            id: 35,
            abbrev: "SCH".to_string(),
            name: "Scholar".to_string(),
            base_main_stats: MainStats::new(90, 100, 100, 105, 115),
            base_hp: 105,
            stat_modifier: StatModifier {
                max_level_main_stat_modifier: 390f64,
                max_level_base_piety: 390,
                max_level_base_direct_hit: 400,
                max_level_base_critical_hit: 400,
                max_level_base_determination: 400,
                max_level_base_skill_speed: 400,
                max_level_base_spell_speed: 400,
                max_level_base_tenacity: 400,
                max_level_hp_modifier: 1900f64,
                max_level_div: 3000f64,
                hp_per_vitality_non_tank: 22.1f64,
                hp_per_vitality_tank: 32.5f64,
            },
        };

        assert_eq!(448, job.get_mind());
        assert_eq!(390, job.get_vitality());
        assert_eq!(400, job.get_critical_strike());
        assert_eq!(400, job.get_determination());
        assert_eq!(400, job.get_direct_hit());
        assert_eq!(400, job.get_skill_speed());
        assert_eq!(400, job.get_spell_speed());
        assert_eq!(400, job.get_tenacity());
        assert_eq!(390, job.get_piety());
        assert_eq!(3150, job.get_hp());
    }


    #[test]
    fn job_table_test() {
        let stat_modifier = StatModifier {
            max_level_main_stat_modifier: 390f64,
            max_level_base_piety: 390,
            max_level_base_direct_hit: 400,
            max_level_base_critical_hit: 400,
            max_level_base_determination: 400,
            max_level_base_skill_speed: 400,
            max_level_base_spell_speed: 400,
            max_level_base_tenacity: 400,
            max_level_hp_modifier: 1900f64,
            max_level_div: 3000f64,
            hp_per_vitality_non_tank: 22.1f64,
            hp_per_vitality_tank: 32.5f64,
        },
        let jobs = vec![

        ]
    }
}
