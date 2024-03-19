use crate::jobclass::JobId;
use crate::stat::{make_equipment_main_stat, MainStat, SubStats};
use itertools::Itertools;
use std::collections::HashMap;

pub type EquipmentSlot = usize;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct EquipmentKey {
    pub job_id: JobId,
    slot: EquipmentSlot,
}

#[derive(PartialEq, Eq)]
pub struct EquipmentDbData {
    equipment_id: usize,
    slot: usize,
    name: String,
    class_id: usize,
    item_level: usize,
    weapon_attack: Option<usize>,
    main_stats: MainStat,
    sub_stats: SubStats,
}

#[derive(PartialEq, Eq)]
pub struct Equipment {
    equipment_id: usize,
    name: String,
    item_level: usize,
    weapon_attack: Option<usize>,
    main_stats: MainStat,
    sub_stats: SubStats,
}

impl EquipmentDbData {
    pub(crate) fn new(
        eq_id: usize,
        slot: usize,
        name: String,
        class_id: usize,
        weapon_attack: Option<usize>,
        ilvl: usize,
        main_stat: usize,
        crit: Option<usize>,
        dh: Option<usize>,
        det: Option<usize>,
        speed: Option<usize>,
        tenacity: Option<usize>,
        piety: Option<usize>,
        main_stat_id: usize,
    ) -> EquipmentDbData {
        EquipmentDbData {
            equipment_id: eq_id,
            slot,
            name: name,
            class_id,
            item_level: ilvl,
            weapon_attack,
            main_stats: make_equipment_main_stat(main_stat_id, main_stat),
            sub_stats: SubStats::new(crit, dh, det, speed, tenacity, piety),
        }
    }
}

impl From<EquipmentDbData> for Equipment {
    fn from(data: EquipmentDbData) -> Self {
        Equipment {
            equipment_id: data.equipment_id,
            name: data.name,
            item_level: data.item_level,
            weapon_attack: data.weapon_attack,
            main_stats: data.main_stats,
            sub_stats: data.sub_stats,
        }
    }
}

/// Make a Hashtable of Equipments based on the jobs they where, and the equipment slot(head, chest, etc) they are in.
pub(crate) fn make_equipment_data_table<T: Into<EquipmentDbData>>(
    equipment_db_data: Vec<T>,
) -> HashMap<EquipmentKey, Vec<Equipment>> {
    let equipment_db_data: Vec<EquipmentDbData> = equipment_db_data
        .into_iter()
        .map(|data| data.into())
        .collect_vec();

    let mut equipment_table = HashMap::<EquipmentKey, Vec<Equipment>>::new();

    for equipment_data in equipment_db_data {
        let key = EquipmentKey {
            job_id: equipment_data.class_id,
            slot: equipment_data.slot,
        };

        if let Some(equipments) = equipment_table.get_mut(&key) {
            equipments.push(Equipment::from(equipment_data));
        } else {
            equipment_table.insert(key, vec![Equipment::from(equipment_data)]);
        }
    }

    equipment_table
}

#[cfg(test)]
mod tests {
    use crate::database_manager::{make_database_mock, DatabaseManager};
    use crate::equipment::{make_equipment_data_table, Equipment, EquipmentDbData, EquipmentKey};

    #[test]
    fn test_equipment_db_data() {
        let equipment_db_data: EquipmentDbData = EquipmentDbData::new(
            1,
            2,
            "Iron Sword".to_string(),
            3,
            Some(4),
            5,
            6,
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(13),
            3,
        );

        assert_eq!(equipment_db_data.equipment_id, 1);
        assert_eq!(equipment_db_data.slot, 2);
        assert_eq!(equipment_db_data.name, "Iron Sword".to_string());
        assert_eq!(equipment_db_data.class_id, 3);
        assert_eq!(equipment_db_data.weapon_attack, Some(4));
        assert_eq!(equipment_db_data.item_level, 5);
        assert_eq!(equipment_db_data.main_stats.get(&3), Some(&6));
        assert_eq!(equipment_db_data.sub_stats.critical_strike, 7);
        assert_eq!(equipment_db_data.sub_stats.direct_hit, 8);
        assert_eq!(equipment_db_data.sub_stats.determination, 9);
        assert_eq!(equipment_db_data.sub_stats.speed, 10);
        assert_eq!(equipment_db_data.sub_stats.tenacity, 11);
        assert_eq!(equipment_db_data.sub_stats.piety, 13);
    }

    #[test]
    fn test_equipment() {
        let equipment = Equipment::from(EquipmentDbData::new(
            1,
            2,
            "Iron Sword".to_string(),
            3,
            Some(4),
            5,
            6,
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(13),
            1,
        ));
    }

    #[test]
    fn test_make_equipment_data_table() {
        let mut database_manager = make_database_mock();
        let equipment_data = database_manager.get_equipment().unwrap();
        let equipment_table = make_equipment_data_table(equipment_data);

        let iron_sword_key = EquipmentKey { job_id: 2, slot: 1 };

        let iron_shield_key = EquipmentKey { job_id: 3, slot: 2 };

        let iron_sword_entry = equipment_table.get(&iron_sword_key).unwrap();
        let iron_shield_entry = equipment_table.get(&iron_shield_key).unwrap();

        let iron_sword = iron_sword_entry.get(0).unwrap().clone();
        let iron_shield = iron_shield_entry.get(0).unwrap().clone();

        assert_eq!(iron_sword.name, "Iron Sword");
        assert_eq!(iron_shield.name, "Iron Shield");
    }
}
