use crate::equipment::{make_equipment_data_table, Equipment, EquipmentDbData, EquipmentKey};
use crate::jobclass::{make_jobclass_data_table, JobClass, JobClassDbData, JobId};
use crate::race::{make_race_data_table, Race, RaceDbData, RaceId};
use std::collections::HashMap;

pub enum DatabaseData {
    EquipmentDb(Vec<EquipmentDbData>),
    RaceDb(Vec<RaceDbData>),
    JobClassDb(Vec<JobClassDbData>),
}

pub enum EngineData {
    EquipmentEngine(HashMap<EquipmentKey, Vec<Equipment>>),
    RaceEngine(HashMap<RaceId, Race>),
    JobClassEngine(HashMap<JobId, JobClass>),
}

/// Organize Data fetched from DB to Hashmaps.
pub fn make_data_table(data: DatabaseData) -> EngineData {
    match data {
        DatabaseData::EquipmentDb(equipment_db) => {
            EngineData::EquipmentEngine(make_equipment_data_table(equipment_db))
        }
        DatabaseData::RaceDb(race_db) => EngineData::RaceEngine(make_race_data_table(race_db)),
        DatabaseData::JobClassDb(jobclass_db) => {
            EngineData::JobClassEngine(make_jobclass_data_table(jobclass_db))
        }
    }
}
