use ffxiv_simbot_db::clan::*;
use ffxiv_simbot_db::equipment::*;
use ffxiv_simbot_db::food::*;
use ffxiv_simbot_db::job::*;

/// The Monolith FFXIV Database that stores all the data needed for DPS simulation.
/// Data is organized into a hashmap for faster searching via Id.
/// Equipments' key is the jobs and slots the equipments belong to, since those are the data
/// equipments will be most searched by
pub(crate) struct FfxivContext {
    pub(crate) jobs: JobTable,
    pub(crate) equipments: EquipmentTable,
    pub(crate) clans: ClanTable,
    pub(crate) foods: FoodTable,
}
