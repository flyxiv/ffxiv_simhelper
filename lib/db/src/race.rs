use crate::stat::*;
use std::collections::HashMap;

pub type RaceId = usize;

#[derive(PartialEq, Eq)]
pub struct RaceDbData {
    race_id: usize,
    race_name: String,
    main_stat: MainStat,
}

/// Saves information needed about each race.
#[derive(PartialEq, Eq)]
pub struct Race {
    race_name: String,
    main_stat: MainStat,
}

impl RaceDbData {
    pub(crate) fn new(
        race_id: usize,
        race_name: String,
        strength: usize,
        intelligence: usize,
        dexterity: usize,
        mind: usize,
    ) -> RaceDbData {
        RaceDbData {
            race_id,
            race_name,
            main_stat: make_race_main_stat(strength, intelligence, dexterity, mind),
        }
    }
}

pub(crate) fn make_race_data_table(race_db_data: Vec<RaceDbData>) -> HashMap<RaceId, Race> {
    let mut race_table = HashMap::<RaceId, Race>::new();

    for race_data in race_db_data {
        race_table.insert(race_data.race_id, Race::from(race_data));
    }

    race_table
}

impl From<RaceDbData> for Race {
    fn from(data: RaceDbData) -> Self {
        Race {
            race_name: data.race_name,
            main_stat: data.main_stat,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::make_data_table;
    use crate::database_manager::{make_database_mock, DatabaseManager, MockDatabaseManager};
    use crate::race::{make_race_data_table, Race, RaceDbData};
    #[test]
    fn test_race_db_data() {
        let race: RaceDbData = RaceDbData::new(5, "Hyur".to_string(), 16, 13, 12, 11);

        assert_eq!(race.race_id, 5);
        assert_eq!(race.race_name, String::from("Hyur"));

        let strength = race.main_stat.get(&0);
        let dexterity = race.main_stat.get(&1);
        let intelligence = race.main_stat.get(&2);
        let mind = race.main_stat.get(&3);

        assert!(strength.is_some());
        assert!(dexterity.is_some());
        assert!(intelligence.is_some());
        assert!(mind.is_some());

        assert_eq!(16, *strength.unwrap());
        assert_eq!(13, *dexterity.unwrap());
        assert_eq!(12, *intelligence.unwrap());
        assert_eq!(11, *mind.unwrap());
    }

    #[test]
    fn test_race() {
        let race = Race::from(RaceDbData::new(0, "Hyur".to_string(), 100, 99, 98, 97));
        let strength = race.main_stat.get(&0);
        let dexterity = race.main_stat.get(&1);
        let intelligence = race.main_stat.get(&2);
        let mind = race.main_stat.get(&3);

        assert_eq!(String::from("Hyur"), race.race_name);

        assert!(strength.is_some());
        assert!(dexterity.is_some());
        assert!(intelligence.is_some());
        assert!(mind.is_some());

        assert_eq!(100, *strength.unwrap());
        assert_eq!(99, *dexterity.unwrap());
        assert_eq!(98, *intelligence.unwrap());
        assert_eq!(97, *mind.unwrap());
    }

    #[test]
    fn test_make_race_data_table() {
        let mut database_manager = make_database_mock();
        let race_data = database_manager.get_races().unwrap();
        let race_data_table = make_race_data_table(race_data);

        let hyur = race_data_table.get(&0).unwrap();
        let elezen = race_data_table.get(&1).unwrap();
        let lalafell = race_data_table.get(&2).unwrap();
        let miqote = race_data_table.get(&3).unwrap();

        assert_eq!(hyur.race_name, "Hyur");
        assert_eq!(elezen.race_name, "Elezen");
        assert_eq!(lalafell.race_name, "Lalafell");
        assert_eq!(miqote.race_name, "Miqo'te");
    }
}
