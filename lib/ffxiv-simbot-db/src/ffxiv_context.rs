use crate::clan::ClanTable;
use crate::equipment::EquipmentTable;
use crate::food::FoodTable;
use crate::job::JobTable;

/// The Monolith FFXIV Database that stores all the data needed for DPS simulation.
/// Data is organized into a hashmap for faster searching via Id.
/// Equipments' key is the jobs and slots the equipments belong to, since those are the data
/// equipments will be most searched by
pub struct FfxivContext {
    pub jobs: JobTable,
    pub equipments: EquipmentTable,
    pub clans: ClanTable,
    pub foods: FoodTable,
}
