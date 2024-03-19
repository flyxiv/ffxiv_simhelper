use ffxiv_simbot_lib_db::data::EngineData;
use ffxiv_simbot_lib_db::equipment::*;
use ffxiv_simbot_lib_db::jobclass::*;
use ffxiv_simbot_lib_db::race::*;
use std::collections::HashMap;

pub type EquipmentSearchTable = HashMap<EquipmentKey, Vec<Equipment>>;
pub type RaceSearchTable = HashMap<RaceId, Race>;
pub type JobClassTable = HashMap<JobId, JobClass>;

/// The Monolith FFXIV Database that stores all the data needed for DPS simulation.
/// Data is organized into a hashmap for faster searching via Id.
/// Equipments' key is the jobs and slots the equipments belong to, those are the data
/// equipments will be most searched by
pub(crate) struct FfxivContext {
    equipments: EquipmentSearchTable,
    races: RaceSearchTable,
    jobclass: JobClassTable,
    // pub statladder: Map<String, Vec<StatLadder>>
}

impl FfxivContext {
    pub(crate) fn new(engine_datas: Vec<EngineData>) -> Self {
        let mut equipments = EquipmentSearchTable::new();
        let mut races = RaceSearchTable::new();
        let mut jobclass = JobClassTable::new();

        for engine_data in engine_datas {
            match engine_data {
                EngineData::EquipmentEngine(equipment_table) => {
                    equipments = equipment_table;
                }
                EngineData::RaceEngine(race_table) => races = race_table,
                EngineData::JobClassEngine(jobclass_table) => jobclass = jobclass_table,
            }
        }

        FfxivContext {
            equipments,
            races,
            jobclass,
        }
    }
}
