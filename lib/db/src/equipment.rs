/// Implements functions needed to save Equipment data
/// in FFXIV Simbot.
use crate::materia::Materia;
use crate::stat::{MainStatTrait, MainStats, SubStatTrait, SubStats};
use crate::{DataError, JsonFileReader};
use itertools::Itertools;
use serde::Deserialize;

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

trait MateriaTrait {
    fn get_materia_slot(&self) -> usize;
    fn equip_materia(&mut self, slot: usize, materia: Materia) -> bool;
    fn unequip_materia(&mut self, slot: usize) -> bool;
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct Equipment {
    id: usize,
    slot_name: String,
    name: String,
    job_name: String,
    main_stats: MainStats,
    sub_stats: SubStats,
    weapon_damage: WeaponDamage,
    armor_defense: ArmorDefense,
    materia_slot_count: usize,
    materia_slot: Vec<Option<Materia>>,
}

#[derive(Eq, PartialEq)]
struct WeaponDamage {
    damage_mag: usize,
    damage_phys: usize,
}

#[derive(Eq, PartialEq)]
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
    fn get_strength(&self) -> usize {
        self.main_stats.get_strength()
    }

    fn get_dexterity(&self) -> usize {
        self.main_stats.get_dexterity()
    }

    fn get_vitality(&self) -> usize {
        self.main_stats.get_vitality()
    }

    fn get_intelligence(&self) -> usize {
        self.main_stats.get_intelligence()
    }

    fn get_mind(&self) -> usize {
        self.main_stats.get_mind()
    }
}

/// Equipment Sub Stat = Equipment Stat + sum(Melded Materia Stat)
impl SubStatTrait for Equipment {
    fn get_critical_strike(&self) -> usize {
        let equipment_critical_strike = self.sub_stats.get_critical_strike();
        let materia_critical_strike = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_critical_strike())
            .sum::<usize>();

        equipment_critical_strike + materia_critical_strike
    }

    fn get_direct_hit(&self) -> usize {
        let equipment_direct_hit = self.sub_stats.get_direct_hit();
        let materia_direct_hit = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_direct_hit())
            .sum::<usize>();

        equipment_direct_hit + materia_direct_hit
    }

    fn get_determination(&self) -> usize {
        let equipment_determination = self.sub_stats.get_determination();
        let materia_determination = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_determination())
            .sum::<usize>();

        equipment_determination + materia_determination
    }

    fn get_skill_speed(&self) -> usize {
        let equipment_skill_speed = self.sub_stats.get_skill_speed();
        let materia_skill_speed = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_skill_speed())
            .sum::<usize>();

        equipment_skill_speed + materia_skill_speed
    }

    fn get_spell_speed(&self) -> usize {
        let equipment_spell_speed = self.sub_stats.get_spell_speed();
        let materia_spell_speed = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_spell_speed())
            .sum::<usize>();

        equipment_spell_speed + materia_spell_speed
    }

    fn get_tenacity(&self) -> usize {
        let equipment_tenacity = self.sub_stats.get_tenacity();
        let materia_tenacity = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_tenacity())
            .sum::<usize>();

        equipment_tenacity + materia_tenacity
    }

    fn get_piety(&self) -> usize {
        let equipment_piety = self.sub_stats.get_piety();
        let materia_piety = self
            .materia_slot
            .iter()
            .filter_map(|materia| materia.as_ref())
            .map(|materia| materia.get_piety())
            .sum::<usize>();

        equipment_piety + materia_piety
    }
}

impl MateriaTrait for Equipment {
    fn get_materia_slot(&self) -> usize {
        self.materia_slot_count
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
    id: usize,
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
    strength: usize,
    #[serde(rename = "Dexterity")]
    dexterity: usize,
    #[serde(rename = "Vitality")]
    vitality: usize,
    #[serde(rename = "Intelligence")]
    intelligence: usize,
    #[serde(rename = "Mind")]
    mind: usize,

    #[serde(rename = "Piety")]
    piety: usize,
    #[serde(rename = "Tenacity")]
    tenacity: usize,
    #[serde(rename = "Direct Hit Rate")]
    direct_hit: usize,
    #[serde(rename = "Critical Hit")]
    critical_hit: usize,
    #[serde(rename = "Determination")]
    determination: usize,
    #[serde(rename = "Skill Speed")]
    skill_speed: usize,
    #[serde(rename = "Spell Speed")]
    spell_speed: usize,
}

impl From<EtroEquipment> for MainStats {
    fn from(equipment: EtroEquipment) -> Self {
        MainStats {
            strength: equipment.strength,
            dexterity: equipment.dexterity,
            vitality: equipment.vitality,
            intelligence: equipment.intelligence,
            mind: equipment.mind,
        }
    }
}

impl From<EtroEquipment> for SubStats {
    fn from(equipment: EtroEquipment) -> Self {
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

impl From<EtroEquipment> for WeaponDamage {
    fn from(equipment: EtroEquipment) -> Self {
        WeaponDamage {
            damage_mag: equipment.damage_mag,
            damage_phys: equipment.damage_phys,
        }
    }
}

impl From<EtroEquipment> for ArmorDefense {
    fn from(equipment: EtroEquipment) -> Self {
        ArmorDefense {
            defense_mag: equipment.defense_mag,
            defense_phys: equipment.defense_phys,
        }
    }
}

struct EquipmentFactory {}
impl JsonFileReader for EquipmentFactory {}

impl EquipmentFactory {
    /// parse equipment_data.json file into Equipment usable in the engine.
    pub fn parse_equipment_json_file<T>(&self, file_path: &str) -> Result<Vec<Equipment>> {
        let data = self.read_json_file(file_path)?;
        let etro_equipments: Vec<EtroEquipment> = serde_json::from_str(data.as_str())?;

        let equipments = etro_equipments
            .into_iter()
            .map(|etro_equipment| self.convert_to_equipment(etro_equipment))
            .collect_vec();

        Ok(equipments)
    }

    fn convert_to_equipment(&self, etro_equipment: EtroEquipment) -> Equipment {
        let (main_stats, sub_stats, weapon_damage, armor_defense) =
            self.make_needed_sub_data(&etro_equipment);

        Equipment {
            id: etro_equipment.id,
            slot_name: etro_equipment.slot_name,
            name: etro_equipment.name,
            job_name: etro_equipment.job_name,
            main_stats,
            sub_stats,
            weapon_damage,
            armor_defense,
            materia_slot_count: etro_equipment.materia_slot_count,
            materia_slot: vec![None; etro_equipment.materia_slot_count],
        }
    }

    fn make_needed_sub_data(
        equipment: &EtroEquipment,
    ) -> (MainStats, SubStats, WeaponDamage, ArmorDefense) {
        let main_stat = MainStats::from(equipment);
        let sub_stat = SubStats::from(equipment);
        let weapon_damage = WeaponDamage::from(equipment);
        let armor_defense = ArmorDefense::from(equipment);

        (main_stat, sub_stat, weapon_damage, armor_defense)
    }
}
