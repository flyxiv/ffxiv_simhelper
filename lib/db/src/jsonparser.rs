use crate::config::FfxivConfig;
use project_root;
use std::path::PathBuf;

pub trait EtroJsonParser {
    fn parse_equipment(&self) -> Vec<EquipmentEtroData>;
    fn parse_jobs(&self) -> Vec<JobEtroData>;
    fn parse_food(&self) -> Vec<FoodEtroData>;
    fn parse_clans(&self) -> Vec<ClanEtroData>;
    fn parse_materia(&self) -> Vec<MateriaEtroData>;
    fn parse_medicine(&self) -> Vec<MedicineEtroData>;
}

pub struct FfxivEtroJsonParser {
    config_directory: PathBuf,
    equipment_json_file_name: String,
    job_json_file_name: String,
    food_json_file_name: String,
    clan_json_file_name: String,
    materia_json_file_name: String,
    medicine_json_file_name: String,
}

impl FfxivEtroJsonParser {
    pub fn make_parser(ffxiv_config: FfxivConfig) -> Self {
        let root = project_root::get_project_root().unwrap();
        let config_path = root.join("config");

        FfxivEtroJsonParser {
            config_directory: config_path,
            equipment_json_file_name: ffxiv_config.equipment_json_file_name,
            job_json_file_name: ffxiv_config.jobs_json_file_name,
            food_json_file_name: ffxiv_config.food_json_file_name,
            clan_json_file_name: ffxiv_config.clan_json_file_name,
            materia_json_file_name: ffxiv_config.materia_json_file_name,
            medicine_json_file_name: ffxiv_config.medicine_json_file_name,
        }
    }
}

impl EtroJsonParser for FfxivEtroJsonParser {
    fn parse_equipment(&self) -> Vec<EquipmentEtroData> {}
}
