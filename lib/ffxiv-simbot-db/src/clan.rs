use crate::errors::Result;
/// Implements functions needed to save Clan data
/// in FFXIV Simbot.
use crate::stat::{MainStatTrait, MainStats, StatType};
use crate::{item_vec_to_id_table, IdTable, JsonFileReader, SearchKeyEntity};
use itertools::Itertools;
use serde::Deserialize;
use serde_json;
use std::path::PathBuf;

type ClanId = usize;
pub type ClanTable = IdTable<ClanId, Clan>;

/// Clan Data fetched from Etro
#[derive(PartialEq, Eq, Deserialize)]
struct EtroClan {
    id: ClanId,
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
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Clan {
    pub(crate) id: ClanId,
    pub(crate) name: String,
    pub(crate) main_stats: MainStats,
}

impl MainStatTrait for Clan {
    fn get_strength(&self) -> StatType {
        self.main_stats.get_strength()
    }

    fn get_dexterity(&self) -> StatType {
        self.main_stats.get_dexterity()
    }
    fn get_vitality(&self) -> StatType {
        self.main_stats.get_vitality()
    }

    fn get_intelligence(&self) -> StatType {
        self.main_stats.get_intelligence()
    }

    fn get_mind(&self) -> StatType {
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

pub struct ClanFactory {}
impl JsonFileReader for ClanFactory {}

impl ClanFactory {
    pub fn new() -> Self {
        ClanFactory {}
    }

    /// parse clans_data.json file into Job usable in the ffxiv-simbot-engine.
    pub fn parse_clans_json_file(
        &self,
        data_directory: &PathBuf,
        file_path: &str,
    ) -> Result<ClanTable> {
        let data = self.read_json_file(&data_directory.join(file_path))?;
        let etro_clans: Vec<EtroClan> = serde_json::from_str(data.as_str())?;

        let clans = etro_clans
            .into_iter()
            .map(|etro_clan| self.convert_to_clan(etro_clan))
            .collect_vec();

        Ok(item_vec_to_id_table(clans))
    }

    fn convert_to_clan(&self, etro_clan: EtroClan) -> Clan {
        Clan {
            id: etro_clan.id,
            name: etro_clan.name.clone(),
            main_stats: etro_clan.into(),
        }
    }
}

impl SearchKeyEntity<ClanId> for Clan {
    fn get_search_key(&self) -> Vec<ClanId> {
        vec![self.id]
    }
}

#[cfg(test)]
mod tests {
    use crate::clan::*;

    #[test]
    fn clan_basic_test() {
        let clan = Clan {
            id: 7,
            name: "Seeker of the Sun".into(),
            main_stats: MainStats {
                strength: 2,
                dexterity: 3,
                vitality: 0,
                intelligence: -1,
                mind: -1,
            },
        };

        assert_eq!(clan.get_strength(), 2);
        assert_eq!(clan.get_dexterity(), 3);
        assert_eq!(clan.get_vitality(), 0);
        assert_eq!(clan.get_intelligence(), -1);
        assert_eq!(clan.get_mind(), -1);
        assert_eq!(clan.name, "Seeker of the Sun");
        assert_eq!(clan.id, 7);
    }

    #[test]
    fn clan_search_table_test() {
        let clans = vec![
            Clan {
                id: 7,
                name: "Seeker of the Sun".into(),
                main_stats: MainStats {
                    strength: 2,
                    dexterity: 3,
                    vitality: 0,
                    intelligence: -1,
                    mind: -1,
                },
            },
            Clan {
                id: 9,
                name: "Sea Wolf".into(),
                main_stats: MainStats {
                    strength: 2,
                    dexterity: -1,
                    vitality: 3,
                    intelligence: -2,
                    mind: 1,
                },
            },
            Clan {
                id: 5,
                name: "Plainsfolk".into(),
                main_stats: MainStats {
                    strength: -1,
                    dexterity: 3,
                    vitality: -1,
                    intelligence: 2,
                    mind: 0,
                },
            },
            Clan {
                id: 11,
                name: "Raen".into(),
                main_stats: MainStats {
                    strength: -1,
                    dexterity: 2,
                    vitality: -1,
                    intelligence: 0,
                    mind: 3,
                },
            },
        ];

        let clan_table = item_vec_to_id_table(clans);

        assert_eq!(clan_table.get(&7).unwrap().name, "Seeker of the Sun");
        assert_eq!(clan_table.get(&9).unwrap().name, "Sea Wolf");
        assert_eq!(clan_table.get(&5).unwrap().name, "Plainsfolk");
        assert_eq!(clan_table.get(&11).unwrap().name, "Raen");
    }
}
