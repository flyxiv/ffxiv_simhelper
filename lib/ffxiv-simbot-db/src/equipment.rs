use crate::errors::DataError;
use crate::errors::Result;
use crate::job::JobAbbrevType;
/// Implements functions needed to save Equipment data
/// in FFXIV Simbot.
use crate::materia::Materia;
use crate::stat::{MainStatTrait, MainStats, StatType, SubStatTrait, SubStats};
use crate::{item_vec_to_id_vec_table, IdTable, JsonFileReader, SearchKeyEntity};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::PathBuf;

pub type EquipmentId = usize;
pub type SlotType = usize;
static PENTAMELD_MATERIA_SLOT: usize = 5;

/// Equipment is usually searched by
/// 1. Equipments that can be equipped by the selected Job
/// 2. Equipments that can be equipped in the given slot
/// So Equipment's Key must be <JobId, SlotId>
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct EquipmentKey {
    pub job_id: JobAbbrevType,
    pub slot_id: SlotType,
}

pub type EquipmentTable = IdTable<EquipmentKey, Vec<Equipment>>;

/// Trait for Weapons
/// give magic/weapon damage info
pub trait WeaponTrait {
    fn get_damage_mag(&self) -> usize;
    fn get_damage_phys(&self) -> usize;
}

pub trait ArmorTrait {
    fn get_defense_mag(&self) -> usize;
    fn get_defense_phys(&self) -> usize;
}

pub trait MateriaTrait {
    fn get_materias(&self) -> &[Option<Materia>];
    fn equip_materia(&mut self, slot: SlotType, materia: Materia) -> Result<()>;
    fn unequip_materia(&mut self, slot: SlotType) -> Result<()>;
}

/// Equipment Data Type for FFXIV Simbot
/// Equipments of different kinds(weapons, armor, accessories) are all
/// represented by this one data, since it makes it more flexible for changes.
#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Equipment {
    pub(crate) id: usize,
    pub(crate) slot_name: String,
    pub slot_category: SlotType,

    pub(crate) name: String,
    pub equipable_jobs: Vec<JobAbbrevType>,

    pub(crate) main_stats: MainStats,
    pub(crate) sub_stats: SubStats,

    pub(crate) weapon_damage: WeaponDamage,
    pub(crate) armor_defense: ArmorDefense,

    pub(crate) materia_slot_count: SlotType,
    pub(crate) materia_slot: Vec<Option<Materia>>,
    pub(crate) pentameldable: bool,
}

impl Equipment {
    pub(crate) fn get_substat_cap(&self) -> StatType {
        let substats_list = vec![
            self.get_raw_direct_hit(),
            self.get_raw_critical_strike(),
            self.get_raw_determination(),
            self.get_raw_skill_speed(),
            self.get_raw_spell_speed(),
            self.get_raw_tenacity(),
            self.get_raw_piety(),
        ];

        let max = substats_list.iter().max();

        if max.is_none() {
            return 0;
        }
        let max = max.unwrap();
        *max
    }

    fn get_raw_direct_hit(&self) -> StatType {
        self.sub_stats.direct_hit
    }
    fn get_raw_critical_strike(&self) -> StatType {
        self.sub_stats.critical_strike
    }

    fn get_raw_determination(&self) -> StatType {
        self.sub_stats.determination
    }

    fn get_raw_skill_speed(&self) -> StatType {
        self.sub_stats.skill_speed
    }

    fn get_raw_spell_speed(&self) -> StatType {
        self.sub_stats.spell_speed
    }

    fn get_raw_tenacity(&self) -> StatType {
        self.sub_stats.tenacity
    }

    fn get_raw_piety(&self) -> StatType {
        self.sub_stats.piety
    }

    fn clip_to_max_stat(&self, stat: StatType) -> StatType {
        let max_stat_threshold = self.get_substat_cap();

        if stat >= max_stat_threshold {
            max_stat_threshold
        } else {
            stat
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize)]
pub(crate) struct WeaponDamage {
    pub(crate) damage_mag: usize,
    pub(crate) damage_phys: usize,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize)]
pub(crate) struct ArmorDefense {
    pub(crate) defense_mag: usize,
    pub(crate) defense_phys: usize,
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

impl WeaponTrait for Equipment {
    fn get_damage_mag(&self) -> usize {
        self.weapon_damage.get_damage_mag()
    }

    fn get_damage_phys(&self) -> usize {
        self.weapon_damage.get_damage_phys()
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
    fn get_direct_hit(&self) -> StatType {
        let equipment_direct_hit = self.sub_stats.get_direct_hit();
        let materia_direct_hit = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_direct_hit())
            .sum::<StatType>();

        self.clip_to_max_stat(equipment_direct_hit + materia_direct_hit)
    }

    fn get_critical_strike(&self) -> StatType {
        let equipment_critical_strike = self.sub_stats.get_critical_strike();
        let materia_critical_strike = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_critical_strike())
            .sum::<StatType>();

        self.clip_to_max_stat(equipment_critical_strike + materia_critical_strike)
    }

    fn get_determination(&self) -> StatType {
        let equipment_determination = self.sub_stats.get_determination();
        let materia_determination = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_determination())
            .sum::<StatType>();

        self.clip_to_max_stat(equipment_determination + materia_determination)
    }

    fn get_skill_speed(&self) -> StatType {
        let equipment_skill_speed = self.sub_stats.get_skill_speed();
        let materia_skill_speed = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_skill_speed())
            .sum::<StatType>();

        self.clip_to_max_stat(equipment_skill_speed + materia_skill_speed)
    }

    fn get_spell_speed(&self) -> StatType {
        let equipment_spell_speed = self.sub_stats.get_spell_speed();
        let materia_spell_speed = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_spell_speed())
            .sum::<StatType>();

        self.clip_to_max_stat(equipment_spell_speed + materia_spell_speed)
    }

    fn get_tenacity(&self) -> StatType {
        let equipment_tenacity = self.sub_stats.get_tenacity();
        let materia_tenacity = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_tenacity())
            .sum::<StatType>();

        self.clip_to_max_stat(equipment_tenacity + materia_tenacity)
    }

    fn get_piety(&self) -> StatType {
        let equipment_piety = self.sub_stats.get_piety();
        let materia_piety = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_piety())
            .sum::<StatType>();

        self.clip_to_max_stat(equipment_piety + materia_piety)
    }
}

impl MateriaTrait for Equipment {
    fn get_materias(&self) -> &[Option<Materia>] {
        self.materia_slot.as_slice()
    }

    fn equip_materia(&mut self, slot: usize, materia: Materia) -> Result<()> {
        if slot >= PENTAMELD_MATERIA_SLOT {
            return Err(DataError::MateriaEquipError(slot));
        }

        if self.materia_slot_count <= slot {
            if !self.pentameldable || slot >= PENTAMELD_MATERIA_SLOT {
                return Err(DataError::MateriaEquipError(slot));
            }
        }

        let materia_in_slot = &self.materia_slot[slot];

        if materia_in_slot.is_some() {
            return Err(DataError::MateriaEquipError(slot));
        }

        self.materia_slot[slot] = Some(materia);
        Ok(())
    }

    fn unequip_materia(&mut self, slot: usize) -> Result<()> {
        if self.materia_slot.len() > slot {
            self.materia_slot[slot] = None;
            Ok(())
        } else {
            Err(DataError::MateriaUnequipError(slot))
        }
    }
}

#[derive(Deserialize, Clone)]
struct EtroEquipment {
    id: EquipmentId,
    name: String,
    level: usize,

    #[serde(rename = "jobName")]
    job_name: Vec<String>,

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
    #[serde(rename = "pentameldable")]
    pentameldable: bool,

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

pub struct EquipmentFactory {}
impl JsonFileReader for EquipmentFactory {}

impl EquipmentFactory {
    pub fn new() -> Self {
        EquipmentFactory {}
    }

    /// parse equipment_data.json file into Equipment usable in the ffxiv-simbot-engine.
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

        Ok(item_vec_to_id_vec_table(equipments))
    }

    fn convert_to_equipment(&self, etro_equipment: EtroEquipment) -> Equipment {
        let (main_stats, sub_stats, weapon_damage, armor_defense) =
            self.make_needed_sub_data(&etro_equipment);

        let max_materia_slot = if etro_equipment.pentameldable {
            5
        } else {
            etro_equipment.materia_slot_count
        };

        Equipment {
            id: etro_equipment.id,
            slot_name: etro_equipment.slot_name,
            slot_category: etro_equipment.slot_category,
            name: etro_equipment.name,
            equipable_jobs: etro_equipment.job_name,
            main_stats,
            sub_stats,
            weapon_damage,
            armor_defense,
            materia_slot_count: etro_equipment.materia_slot_count,
            materia_slot: vec![None; max_materia_slot],
            pentameldable: etro_equipment.pentameldable,
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

#[cfg(test)]
mod tests {
    use crate::equipment::{ArmorDefense, Equipment, EquipmentKey, MateriaTrait, WeaponDamage};
    use crate::item_vec_to_id_table;
    use crate::materia::Materia;
    use crate::stat::{MainStatTrait, MainStats, SubStatTrait, SubStats};

    #[test]
    fn test_weapon_equipment() {
        let weapon = Equipment {
            id: 35175,
            slot_name: "weapon".to_string(),
            slot_category: 13,
            name: "Augmented Radiant's Sword Breakers".to_string(),
            equipable_jobs: vec!["NIN".to_string()],
            main_stats: MainStats {
                strength: 0,
                dexterity: 296,
                intelligence: 0,
                mind: 0,
                vitality: 310,
            },
            sub_stats: SubStats {
                direct_hit: 266,
                critical_strike: 186,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            weapon_damage: {
                WeaponDamage {
                    damage_mag: 0,
                    damage_phys: 119,
                }
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            materia_slot_count: 2,
            materia_slot: vec![None; 2],
            pentameldable: false,
        };

        assert_eq!(weapon.id, 35175);
        assert_eq!(weapon.slot_name, "weapon".to_string());
        assert_eq!(weapon.slot_category, 13);
        assert_eq!(
            weapon.name,
            "Augmented Radiant's Sword Breakers".to_string()
        );
        assert_eq!(weapon.equipable_jobs, vec!["NIN".to_string()]);
        assert_eq!(weapon.get_dexterity(), 296);
        assert_eq!(weapon.get_direct_hit(), 266);
        assert_eq!(weapon.get_critical_strike(), 186);
        assert_eq!(weapon.get_determination(), 0);
        assert_eq!(weapon.get_skill_speed(), 0);
    }

    #[test]
    fn test_armor_equipment() {
        let armor = Equipment {
            id: 37812,
            slot_name: "finger".to_string(),
            slot_category: 12,
            name: "Rinascita Ring of Fending".to_string(),
            equipable_jobs: vec!["PLD", "WAR", "DRK", "GNB"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            main_stats: MainStats {
                strength: 149,
                dexterity: 0,
                intelligence: 0,
                mind: 0,
                vitality: 158,
            },
            sub_stats: SubStats {
                direct_hit: 0,
                critical_strike: 130,
                determination: 91,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            weapon_damage: {
                WeaponDamage {
                    damage_mag: 0,
                    damage_phys: 0,
                }
            },
            armor_defense: ArmorDefense {
                defense_mag: 1,
                defense_phys: 1,
            },
            materia_slot_count: 1,
            materia_slot: vec![None; 1],
            pentameldable: false,
        };

        assert_eq!(armor.id, 37812);
        assert_eq!(armor.slot_name, "finger".to_string());
        assert_eq!(armor.slot_category, 12);
        assert_eq!(armor.name, "Rinascita Ring of Fending".to_string());
        assert_eq!(
            armor.equipable_jobs,
            vec!["PLD", "WAR", "DRK", "GNB"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn equipment_table_test() {
        let weapon = Equipment {
            id: 35175,
            slot_name: "weapon".to_string(),
            slot_category: 13,
            name: "Augmented Radiant's Sword Breakers".to_string(),
            equipable_jobs: vec!["NIN".into()],
            main_stats: MainStats {
                strength: 0,
                dexterity: 296,
                intelligence: 0,
                mind: 0,
                vitality: 310,
            },
            sub_stats: SubStats {
                direct_hit: 266,
                critical_strike: 186,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            weapon_damage: {
                WeaponDamage {
                    damage_mag: 0,
                    damage_phys: 119,
                }
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            materia_slot_count: 2,
            materia_slot: vec![None; 2],
            pentameldable: false,
        };

        let armor = Equipment {
            id: 37812,
            slot_name: "finger".to_string(),
            slot_category: 12,
            name: "Rinascita Ring of Fending".to_string(),
            equipable_jobs: vec!["PLD", "WAR", "DRK", "GNB"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            main_stats: MainStats {
                strength: 149,
                dexterity: 0,
                intelligence: 0,
                mind: 0,
                vitality: 158,
            },
            sub_stats: SubStats {
                direct_hit: 0,
                critical_strike: 130,
                determination: 91,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            weapon_damage: {
                WeaponDamage {
                    damage_mag: 0,
                    damage_phys: 0,
                }
            },
            armor_defense: ArmorDefense {
                defense_mag: 1,
                defense_phys: 1,
            },
            materia_slot_count: 1,
            materia_slot: vec![None; 1],
            pentameldable: false,
        };

        let equipment_table = item_vec_to_id_table(vec![weapon.clone(), armor.clone()]);

        assert_eq!(
            equipment_table
                .get(&EquipmentKey {
                    job_id: "NIN".to_string(),
                    slot_id: 13
                })
                .unwrap()
                .id,
            weapon.id
        );

        assert_eq!(
            equipment_table
                .get(&EquipmentKey {
                    job_id: "PLD".to_string(),
                    slot_id: 12
                })
                .unwrap()
                .id,
            armor.id
        );

        assert_eq!(
            equipment_table
                .get(&EquipmentKey {
                    job_id: "WAR".to_string(),
                    slot_id: 12
                })
                .unwrap()
                .id,
            armor.id
        );

        assert_eq!(
            equipment_table
                .get(&EquipmentKey {
                    job_id: "GNB".to_string(),
                    slot_id: 12
                })
                .unwrap()
                .id,
            armor.id
        );

        assert_eq!(
            equipment_table
                .get(&EquipmentKey {
                    job_id: "DRK".to_string(),
                    slot_id: 12
                })
                .unwrap()
                .id,
            armor.id
        );
    }

    #[test]
    fn test_materia_cap_equip() {
        let mut weapon = Equipment {
            id: 35175,
            slot_name: "weapon".to_string(),
            slot_category: 13,
            name: "Augmented Radiant's Sword Breakers".to_string(),
            equipable_jobs: vec!["NIN".to_string()],
            main_stats: MainStats {
                strength: 0,
                dexterity: 296,
                intelligence: 0,
                mind: 0,
                vitality: 310,
            },
            sub_stats: SubStats {
                direct_hit: 220,
                critical_strike: 186,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            weapon_damage: {
                WeaponDamage {
                    damage_mag: 0,
                    damage_phys: 119,
                }
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            materia_slot_count: 2,
            materia_slot: vec![None; 2],
            pentameldable: false,
        };

        let crit_materia = Materia {
            sub_stats: SubStats {
                critical_strike: 32,
                direct_hit: 0,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            penta_meldable: false,
        };

        weapon.equip_materia(0, crit_materia.clone());

        assert_eq!(weapon.get_critical_strike(), 186 + 32);

        // if two crit materia is wielded, critical strike becomes 186 + 64 = 250,
        // exceeding the substat cap of 220.
        // Make sure equipment clips their stat to the cap 220

        weapon.equip_materia(1, crit_materia.clone());
        assert_eq!(weapon.get_critical_strike(), weapon.get_substat_cap());

        // unequip a crit materia, and and determination materia
        let determination_materia = Materia {
            sub_stats: SubStats {
                critical_strike: 0,
                direct_hit: 0,
                determination: 32,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            penta_meldable: false,
        };

        weapon.unequip_materia(0);
        weapon.equip_materia(0, determination_materia.clone());

        assert_eq!(weapon.get_determination(), 32);
        assert_eq!(weapon.get_critical_strike(), 186 + 32);

        // Weapon is unpentameldable, so make sure it doesn't equip materia at a penta-meldable slot
        let determination_materia_pentameldable = Materia {
            sub_stats: SubStats {
                critical_strike: 0,
                direct_hit: 0,
                determination: 24,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            penta_meldable: false,
        };

        assert!(weapon.equip_materia(2, determination_materia).is_err());
        assert!(weapon
            .equip_materia(2, determination_materia_pentameldable)
            .is_err());
    }

    #[test]
    fn equipment_pentameld_test() {
        let mut weapon = Equipment {
            id: 35175,
            slot_name: "weapon".to_string(),
            slot_category: 13,
            name: "Augmented Radiant's Sword Breakers".to_string(),
            equipable_jobs: vec!["NIN".to_string()],
            main_stats: MainStats {
                strength: 0,
                dexterity: 296,
                intelligence: 0,
                mind: 0,
                vitality: 310,
            },
            sub_stats: SubStats {
                direct_hit: 220,
                critical_strike: 186,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            weapon_damage: {
                WeaponDamage {
                    damage_mag: 0,
                    damage_phys: 119,
                }
            },
            armor_defense: ArmorDefense {
                defense_mag: 0,
                defense_phys: 0,
            },
            materia_slot_count: 2,
            materia_slot: vec![None; 5],
            pentameldable: true,
        };

        let crit_materia = Materia {
            sub_stats: SubStats {
                critical_strike: 32,
                direct_hit: 0,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            penta_meldable: false,
        };

        let det_materia = Materia {
            sub_stats: SubStats {
                critical_strike: 0,
                direct_hit: 0,
                determination: 32,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            penta_meldable: false,
        };

        let det_penta_materia = Materia {
            sub_stats: SubStats {
                critical_strike: 0,
                direct_hit: 0,
                determination: 24,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            penta_meldable: true,
        };

        let dh_penta_materia = Materia {
            sub_stats: SubStats {
                critical_strike: 0,
                direct_hit: 24,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            penta_meldable: true,
        };

        assert!(weapon.equip_materia(0, crit_materia.clone()).is_ok());
        assert!(weapon.equip_materia(1, det_materia.clone()).is_ok());
        assert!(weapon.equip_materia(2, dh_penta_materia.clone()).is_ok());
        assert!(weapon.equip_materia(3, det_penta_materia.clone()).is_ok());
        assert!(weapon.equip_materia(4, det_penta_materia.clone()).is_ok());

        assert!(weapon.equip_materia(5, crit_materia.clone()).is_err());
        assert_eq!(
            weapon.get_critical_strike(),
            weapon.get_raw_critical_strike() + crit_materia.get_critical_strike()
        );
        assert_eq!(
            weapon.get_determination(),
            weapon.get_raw_determination()
                + det_materia.get_determination()
                + det_penta_materia.get_determination() * 2
        );
    }
}
