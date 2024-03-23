use ffxiv_simbot_lib_db::clan::*;
use ffxiv_simbot_lib_db::equipment::*;
use ffxiv_simbot_lib_db::food::*;
use ffxiv_simbot_lib_db::job::*;
use std::collections::HashMap;

pub(crate) struct EquipmentKey {
    job_id: usize,
    slot_id: usize,
}

type EquipmentSearchTable = HashMap<EquipmentKey, Vec<Equipment>>;
type ClanId = usize;
type ClanTable = HashMap<ClanId, Vec<Clan>>;
type JobId = usize;
type JobTable = HashMap<JobId, Vec<Job>>;
type FoodId = usize;
type FoodTable = HashMap<FoodId, Vec<Food>>;

/// The Monolith FFXIV Database that stores all the data needed for DPS simulation.
/// Data is organized into a hashmap for faster searching via Id.
/// Equipments' key is the jobs and slots the equipments belong to, since those are the data
/// equipments will be most searched by
pub(crate) struct FfxivContext {
    equipments: EquipmentSearchTable,
    clans: ClanTable,
    jobs: JobTable,
    foods: FoodTable,
}
