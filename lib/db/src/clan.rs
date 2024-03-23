/// Implements functions needed to save Clan data
/// in FFXIV Simbot.
use crate::stat::{MainStatTrait, MainStats};
use crate::{JsonFileReader, Result};
use itertools::Itertools;

/// Clan Data fetched from Etro
#[derive(PartialEq, Eq)]
struct EtroClan {
    id: usize,
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
}

pub struct Clan {
    id: usize,
    name: String,
    main_stats: MainStats,
}

impl MainStatTrait for Clan {
    fn get_strength(&self) -> usize {
        self.main_stats.get_strength()
    }

    fn get_dexterity(&self) -> usize {
        self.main_stats.get_dexterity()
    }
    fn get_vitality(&self) -> usize {
        self.main_stats.get_vitality()
    }

    fn get_intelligence(&self) -> usize {
        self.main_stats.get_intelligence()
    }

    fn get_mind(&self) -> usize {
        self.main_stats.get_mind()
    }
}

impl From<EtroClan> for MainStats {
    fn from(etro_clan: EtroClan) -> Self {
        MainStats {
            strength: etro_clan.strength,
            dexterity: etro_clan.dexterity,
            vitality: etro_clan.vitality,
            intelligence: etro_clan.intelligence,
            mind: etro_clan.mind,
        }
    }
}

struct ClanFactory {}
impl JsonFileReader for ClanFactory {}

impl ClanFactory {
    /// parse jobs_data.json file into Job usable in the engine.
    pub fn parse_jobs_json_file(&self, file_path: &str) -> Result<Vec<Clan>> {
        let data = self.read_json_file(file_path)?;
        let etro_clans: Vec<EtroClan> = serde_json::from_str(data.as_str())?;

        let jobs = etro_clans
            .into_iter()
            .map(|etro_clan| self.convert_to_job(etro_clan))
            .collect_vec();

        Ok(jobs)
    }

    fn convert_to_clan(&self, etro_clan: EtroClan) -> Clan {
        Clan {
            id: etro_clan.id,
            name: etro_clan.name.clone(),
            main_stats: etro_clan.into(),
        }
    }
}
