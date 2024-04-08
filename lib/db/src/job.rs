use crate::constants::FFXIV_STAT_MODIFIER;
/// Implements functions needed to save Job data
/// in FFXIV Simbot.
/// Only save combat jobs as of now.
use crate::stat::{HpType, MainStatTrait, MainStats, SpecialStatTrait, StatType, SubStatTrait};
use crate::{item_vec_to_id_table, IdTable, JsonFileReader, Result, SearchKeyEntity, StatModifier};
use itertools::Itertools;
use serde::Deserialize;
use std::path::PathBuf;
use std::string::ToString;

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
    #[serde(rename = "IsTank")]
    is_tank: bool,
}

/// Job for FFXIV Simbot.
/// Only treat Combat Jobs as of now.
#[derive(Clone)]
pub struct Job {
    pub(crate) id: JobId,
    pub(crate) abbrev: String,
    pub(crate) name: String,
    pub(crate) base_main_stats: MainStats,
    pub(crate) base_hp: usize,
    // https://www.akhmorning.com/allagan-studies/modifiers/levelmods/
    // base stats are determined by base * level stat modifier
    pub(crate) is_tank: bool,
}

pub fn is_tank(abbrev: String) -> bool {
    match abbrev.as_str() {
        "PLD" | "WAR" | "DRK" | "GNB" => true,
        _ => false,
    }
}

impl PartialEq for Job {
    // Job is only equal if job id is equal.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Job {}

fn calculate_modified_stat(base_stat: i32) -> i32 {
    (base_stat as f64 * FFXIV_STAT_MODIFIER.max_level_main_stat_modifier / 100f64).floor() as i32
}

impl MainStatTrait for Job {
    //https://finalfantasy.fandom.com/wiki/Final_Fantasy_XIV_attributes
    fn get_strength(&self) -> i32 {
        calculate_modified_stat(self.base_main_stats.strength)
    }

    fn get_dexterity(&self) -> i32 {
        calculate_modified_stat(self.base_main_stats.dexterity)
    }

    fn get_vitality(&self) -> i32 {
        calculate_modified_stat(self.base_main_stats.vitality)
    }

    fn get_intelligence(&self) -> i32 {
        calculate_modified_stat(self.base_main_stats.intelligence)
    }

    fn get_mind(&self) -> i32 {
        calculate_modified_stat(self.base_main_stats.mind)
    }
}

impl SubStatTrait for Job {
    fn get_critical_strike(&self) -> i32 {
        FFXIV_STAT_MODIFIER.max_level_base_critical_hit
    }

    fn get_direct_hit(&self) -> i32 {
        FFXIV_STAT_MODIFIER.max_level_base_direct_hit
    }

    fn get_determination(&self) -> i32 {
        FFXIV_STAT_MODIFIER.max_level_base_determination
    }

    fn get_piety(&self) -> i32 {
        FFXIV_STAT_MODIFIER.max_level_base_piety
    }

    fn get_skill_speed(&self) -> i32 {
        FFXIV_STAT_MODIFIER.max_level_base_skill_speed
    }

    fn get_spell_speed(&self) -> i32 {
        FFXIV_STAT_MODIFIER.max_level_base_spell_speed
    }

    fn get_tenacity(&self) -> i32 {
        FFXIV_STAT_MODIFIER.max_level_base_tenacity
    }
}

// TODO: Fix HP equation.
impl SpecialStatTrait for Job {
    fn get_hp(&self) -> usize {
        let hp_stat_base = self.base_hp * FFXIV_STAT_MODIFIER.max_level_hp_modifier as usize;
        let vitality_stat = self.get_vitality() - FFXIV_STAT_MODIFIER.max_level_base_vitality;
        let vitality_stat_hp = if self.is_tank {
            FFXIV_STAT_MODIFIER.hp_per_vitality_tank * vitality_stat as f64
        } else {
            FFXIV_STAT_MODIFIER.hp_per_vitality_non_tank * vitality_stat as f64
        };

        hp_stat_base + vitality_stat_hp as usize
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
        data_directory: &PathBuf,
        file_path: &str,
    ) -> Result<JobTable> {
        let data = self.read_json_file(&data_directory.join(file_path))?;
        let etro_jobs: Vec<EtroJob> = serde_json::from_str(data.as_str())?;

        let jobs = etro_jobs
            .into_iter()
            .map(|etro_job| self.convert_to_job(etro_job))
            .collect_vec();

        Ok(item_vec_to_id_table(jobs))
    }

    fn convert_to_job(&self, etro_job: EtroJob) -> Job {
        Job {
            id: etro_job.id,
            abbrev: etro_job.abbrev.clone(),
            name: etro_job.name.clone(),
            base_hp: etro_job.hp,
            is_tank: etro_job.is_tank,
            base_main_stats: etro_job.into(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::item_vec_to_id_table;
    use crate::job::Job;
    use crate::stat::{MainStatTrait, MainStats, SpecialStatTrait, SubStatTrait};

    #[test]
    fn job_basic_test() {
        let job = Job {
            id: 21,
            abbrev: "WAR".to_string(),
            name: "Warrior".to_string(),
            base_main_stats: MainStats {
                strength: 105,
                dexterity: 95,
                vitality: 110,
                intelligence: 40,
                mind: 55,
            },
            base_hp: 145,
            is_tank: true,
        };
    }

    #[test]
    fn job_warrior_base_stat_calculation_test() {
        let job = Job {
            id: 21,
            abbrev: "WAR".to_string(),
            name: "Warrior".to_string(),
            base_main_stats: MainStats {
                strength: 105,
                dexterity: 95,
                vitality: 110,
                intelligence: 40,
                mind: 55,
            },
            base_hp: 145,
            is_tank: true,
        };

        assert_eq!(409, job.get_strength());
        assert_eq!(429, job.get_vitality());
        assert_eq!(400, job.get_critical_strike());
        assert_eq!(390, job.get_determination());
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
            base_main_stats: MainStats {
                strength: 90,
                dexterity: 115,
                vitality: 100,
                intelligence: 85,
                mind: 80,
            },
            base_hp: 105,
            is_tank: false,
        };

        assert_eq!(448, job.get_dexterity());
        assert_eq!(390, job.get_vitality());
        assert_eq!(400, job.get_critical_strike());
        assert_eq!(390, job.get_determination());
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
            base_main_stats: MainStats {
                strength: 55,
                dexterity: 105,
                vitality: 100,
                intelligence: 115,
                mind: 110,
            },
            base_hp: 105,
            is_tank: false,
        };

        assert_eq!(448, job.get_intelligence());
        assert_eq!(390, job.get_vitality());
        assert_eq!(400, job.get_critical_strike());
        assert_eq!(390, job.get_determination());
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
            id: 28,
            abbrev: "SCH".to_string(),
            name: "Scholar".to_string(),
            base_main_stats: MainStats {
                strength: 90,
                dexterity: 100,
                vitality: 100,
                intelligence: 105,
                mind: 115,
            },
            base_hp: 105,
            is_tank: false,
        };

        assert_eq!(448, job.get_mind());
        assert_eq!(390, job.get_vitality());
        assert_eq!(400, job.get_critical_strike());
        assert_eq!(390, job.get_determination());
        assert_eq!(400, job.get_direct_hit());
        assert_eq!(400, job.get_skill_speed());
        assert_eq!(400, job.get_spell_speed());
        assert_eq!(400, job.get_tenacity());
        assert_eq!(390, job.get_piety());
        assert_eq!(3150, job.get_hp());
    }

    #[test]
    fn job_table_test() {
        let jobs = vec![
            Job {
                id: 28,
                abbrev: "SCH".to_string(),
                name: "Scholar".to_string(),
                base_main_stats: MainStats {
                    strength: 90,
                    dexterity: 100,
                    vitality: 100,
                    intelligence: 105,
                    mind: 115,
                },
                base_hp: 105,
                is_tank: false,
            },
            Job {
                id: 35,
                abbrev: "RDM".to_string(),
                name: "Red Mage".to_string(),
                base_main_stats: MainStats {
                    strength: 55,
                    dexterity: 105,
                    vitality: 100,
                    intelligence: 115,
                    mind: 110,
                },
                base_hp: 105,
                is_tank: false,
            },
            Job {
                id: 23,
                abbrev: "BRD".to_string(),
                name: "BARD".to_string(),
                base_main_stats: MainStats {
                    strength: 90,
                    dexterity: 115,
                    vitality: 100,
                    intelligence: 85,
                    mind: 80,
                },
                base_hp: 105,
                is_tank: false,
            },
            Job {
                id: 21,
                abbrev: "WAR".to_string(),
                name: "Warrior".to_string(),
                base_main_stats: MainStats {
                    strength: 105,
                    dexterity: 95,
                    vitality: 110,
                    intelligence: 40,
                    mind: 55,
                },
                base_hp: 145,
                is_tank: true,
            },
        ];

        let job_table = item_vec_to_id_table(jobs);

        assert_eq!(job_table.get("SCH").unwrap().id, 28);
        assert_eq!(job_table.get("RDM").unwrap().id, 35);
        assert_eq!(job_table.get("BRD").unwrap().id, 23);
        assert_eq!(job_table.get("WAR").unwrap().id, 21);
    }
}
