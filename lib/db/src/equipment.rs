use crate::job::JobAbbrevType;
/// Implements functions needed to save Equipment data
/// in FFXIV Simbot.
use crate::materia::Materia;
use crate::stat::{MainStatTrait, MainStats, StatType, SubStatTrait, SubStats};
use crate::{item_vec_to_id_table, DataError, IdTable, JsonFileReader, SearchKeyEntity};
use itertools::Itertools;
use serde::Deserialize;
use std::path::PathBuf;

pub type EquipmentId = usize;
pub type SlotType = usize;

/// Equipment is usually searched by
/// 1. Equipments that can be equipped by the selected Job
/// 2. Equipments that can be equipped in the given slot
/// So Equipment's Key must be <JobId, SlotId>
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct EquipmentKey {
    job_id: JobAbbrevType,
    slot_id: SlotType,
}

pub type EquipmentTable = IdTable<EquipmentKey, Equipment>;
type Result<T> = std::result::Result<T, DataError>;

/// Trait for Weapons
/// give magic/weapon damage info
trait WeaponTrait {
    fn get_damage_mag(&self) -> usize;
    fn get_damage_phys(&self) -> usize;
}

trait ArmorTrait {
    fn get_defense_mag(&self) -> usize;
    fn get_defense_phys(&self) -> usize;
}

pub trait MateriaTrait {
    fn get_materias(&self) -> &[Option<Materia>];
    fn equip_materia(&mut self, slot: SlotType, materia: Materia) -> bool;
    fn unequip_materia(&mut self, slot: SlotType) -> bool;
}

/// Equipment Data Type for FFXIV Simbot
/// Equipments of different kinds(weapons, armor, accessories) are all
/// represented by this one data, since it makes it more flexible for changes.
#[derive(PartialEq, Clone)]
pub struct Equipment {
    id: usize,
    slot_name: String,
    pub slot_category: SlotType,
    name: String,
    pub equipable_jobs: Vec<JobAbbrevType>,
    main_stats: MainStats,
    sub_stats: SubStats,
    weapon_damage: WeaponDamage,
    armor_defense: ArmorDefense,
    materia_slot_count: SlotType,
    materia_slot: Vec<Option<Materia>>,
}

#[derive(Eq, PartialEq, Clone)]
struct WeaponDamage {
    damage_mag: usize,
    damage_phys: usize,
}

#[derive(Eq, PartialEq, Clone)]
struct ArmorDefense {
    defense_mag: usize,
    defense_phys: usize,
}

impl WeaponTrait for WeaponDamage {
    fn get_damage_mag(&self) -> usize {
        self.damage_mag
    }

    fn get_damage_phys(&self) -> usize {
        self.damage_phys
    }
}

impl ArmorTrait for ArmorDefense {
    fn get_defense_mag(&self) -> usize {
        self.defense_mag
    }

    fn get_defense_phys(&self) -> usize {
        self.defense_phys
    }
}

impl MainStatTrait for Equipment {
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

/// Equipment Sub Stat = Equipment Stat + sum(Melded Materia Stat)
impl SubStatTrait for Equipment {
    fn get_critical_strike(&self) -> StatType {
        let equipment_critical_strike = self.sub_stats.get_critical_strike();
        let materia_critical_strike = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_critical_strike())
            .sum::<StatType>();

        equipment_critical_strike + materia_critical_strike
    }

    fn get_direct_hit(&self) -> StatType {
        let equipment_direct_hit = self.sub_stats.get_direct_hit();
        let materia_direct_hit = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_direct_hit())
            .sum::<StatType>();

        equipment_direct_hit + materia_direct_hit
    }

    fn get_determination(&self) -> StatType {
        let equipment_determination = self.sub_stats.get_determination();
        let materia_determination = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_determination())
            .sum::<StatType>();

        equipment_determination + materia_determination
    }

    fn get_skill_speed(&self) -> StatType {
        let equipment_skill_speed = self.sub_stats.get_skill_speed();
        let materia_skill_speed = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_skill_speed())
            .sum::<StatType>();

        equipment_skill_speed + materia_skill_speed
    }

    fn get_spell_speed(&self) -> StatType {
        let equipment_spell_speed = self.sub_stats.get_spell_speed();
        let materia_spell_speed = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_spell_speed())
            .sum::<StatType>();

        equipment_spell_speed + materia_spell_speed
    }

    fn get_tenacity(&self) -> StatType {
        let equipment_tenacity = self.sub_stats.get_tenacity();
        let materia_tenacity = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_tenacity())
            .sum::<StatType>();

        equipment_tenacity + materia_tenacity
    }

    fn get_piety(&self) -> StatType {
        let equipment_piety = self.sub_stats.get_piety();
        let materia_piety = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_piety())
            .sum::<StatType>();

        equipment_piety + materia_piety
    }
}

impl MateriaTrait for Equipment {
    fn get_materias(&self) -> &[Option<Materia>] {
        self.materia_slot.as_slice()
    }

    fn equip_materia(&mut self, slot: usize, materia: Materia) -> bool {
        if self.materia_slot.len() < self.materia_slot_count {
            if slot > 2 && !materia.penta_meldable {
                return false;
            }

            self.materia_slot[slot] = Some(materia);
            true
        } else {
            false
        }
    }

    fn unequip_materia(&mut self, slot: usize) -> bool {
        if self.materia_slot.len() > slot {
            self.materia_slot[slot] = None;
            true
        } else {
            false
        }
    }
}

#[derive(Deserialize, Clone)]
struct EtroEquipment {
    id: EquipmentId,
    name: String,
    level: usize,

    #[serde(rename = "jobName")]
    job_name: String,

    #[serde(rename = "slotName")]
    slot_name: String,
    #[serde(rename = "slotCategory")]
    slot_category: usize,

    weapon: bool,

    #[serde(rename = "damageMag")]
    damage_mag: usize,
    #[serde(rename = "damagePhys")]
    damage_phys: usize,
    #[serde(rename = "defenseMag")]
    defense_mag: usize,
    #[serde(rename = "defensePhys")]
    defense_phys: usize,

    #[serde(rename = "materiaSlotCount")]
    materia_slot_count: usize,

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

    #[serde(rename = "Piety")]
    piety: StatType,
    #[serde(rename = "Tenacity")]
    tenacity: StatType,
    #[serde(rename = "Direct Hit Rate")]
    direct_hit: StatType,
    #[serde(rename = "Critical Hit")]
    critical_hit: StatType,
    #[serde(rename = "Determination")]
    determination: StatType,
    #[serde(rename = "Skill Speed")]
    skill_speed: StatType,
    #[serde(rename = "Spell Speed")]
    spell_speed: StatType,
}

impl From<&EtroEquipment> for MainStats {
    fn from(equipment: &EtroEquipment) -> Self {
        MainStats {
            strength: equipment.strength,
            dexterity: equipment.dexterity,
            vitality: equipment.vitality,
            intelligence: equipment.intelligence,
            mind: equipment.mind,
        }
    }
}

impl From<&EtroEquipment> for SubStats {
    fn from(equipment: &EtroEquipment) -> Self {
        SubStats {
            critical_strike: equipment.critical_hit,
            direct_hit: equipment.direct_hit,
            determination: equipment.determination,
            skill_speed: equipment.skill_speed,
            spell_speed: equipment.spell_speed,
            tenacity: equipment.tenacity,
            piety: equipment.piety,
        }
    }
}

impl From<&EtroEquipment> for WeaponDamage {
    fn from(equipment: &EtroEquipment) -> Self {
        WeaponDamage {
            damage_mag: equipment.damage_mag,
            damage_phys: equipment.damage_phys,
        }
    }
}

impl From<&EtroEquipment> for ArmorDefense {
    fn from(equipment: &EtroEquipment) -> Self {
        ArmorDefense {
            defense_mag: equipment.defense_mag,
            defense_phys: equipment.defense_phys,
        }
    }
}

impl SearchKeyEntity<EquipmentKey> for Equipment {
    fn get_search_key(&self) -> Vec<EquipmentKey> {
        let mut keys = Vec::new();

        for job in &self.equipable_jobs {
            keys.push(EquipmentKey {
                job_id: job.clone(),
                slot_id: self.slot_category,
            });
        }

        keys
    }
}

/// job_name comes in job abbrevs, splitted by one space
/// ex) "PLD WAR DRK GNB"
/// Convert this to Vec!["PLD", "WAR", "DRK", "GNB"]
fn convert_to_equipable_jobs(job_name: String) -> Vec<JobAbbrevType> {
    job_name
        .split(" ")
        .map(|job_abbrev| job_abbrev.to_string())
        .collect_vec()
}

pub struct EquipmentFactory {}
impl JsonFileReader for EquipmentFactory {}

impl EquipmentFactory {
    pub fn new() -> Self {
        EquipmentFactory {}
    }

    /// parse equipment_data.json file into Equipment usable in the engine.
    pub fn parse_equipment_json_file(
        &self,
        data_directory: &PathBuf,
        file_path: &str,
    ) -> Result<EquipmentTable> {
        let data = self.read_json_file(&data_directory.join(file_path))?;
        let etro_equipments: Vec<EtroEquipment> = serde_json::from_str(data.as_str())?;

        let equipments = etro_equipments
            .into_iter()
            .map(|etro_equipment| self.convert_to_equipment(etro_equipment))
            .collect_vec();

        Ok(item_vec_to_id_table(equipments))
    }

    fn convert_to_equipment(&self, etro_equipment: EtroEquipment) -> Equipment {
        let (main_stats, sub_stats, weapon_damage, armor_defense) =
            self.make_needed_sub_data(&etro_equipment);

        Equipment {
            id: etro_equipment.id,
            slot_name: etro_equipment.slot_name,
            slot_category: etro_equipment.slot_category,
            name: etro_equipment.name,
            equipable_jobs: convert_to_equipable_jobs(etro_equipment.job_name),
            main_stats,
            sub_stats,
            weapon_damage,
            armor_defense,
            materia_slot_count: etro_equipment.materia_slot_count,
            materia_slot: vec![None; etro_equipment.materia_slot_count],
        }
    }

    fn make_needed_sub_data(
        &self,
        equipment: &EtroEquipment,
    ) -> (MainStats, SubStats, WeaponDamage, ArmorDefense) {
        let main_stat = MainStats::from(equipment);
        let sub_stat = SubStats::from(equipment);
        let weapon_damage = WeaponDamage::from(equipment);
        let armor_defense = ArmorDefense::from(equipment);

        (main_stat, sub_stat, weapon_damage, armor_defense)
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::equipment::Equipment;

    #[test]
    fn test_weapon_equipment() {
        let weapon = Equipment {
            0,
            "Weapon".to_string(),
            "Excalibur".to_string(),
            "Paladin".to_string(),

        }
    }
}
*/
