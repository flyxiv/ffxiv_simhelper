use std::collections::HashMap;

/// Saves the main stat of the equipment/character.
/// 0 -> strength, 1 -> dexterity, 2 -> intelligence, 3 -> mind
/// For equipments, MainStat Ideally should have only one key-value pair,
/// since in FFXIV no equipment has more than one main stat as of now.
pub(crate) type MainStat = HashMap<usize, usize>;

/// Saves the sub stat of the equipment/character.
#[derive(PartialEq, Eq)]
pub(crate) struct SubStats {
    pub(crate) critical_strike: usize,
    pub(crate) direct_hit: usize,
    pub(crate) determination: usize,
    pub(crate) speed: usize,
    pub(crate) tenacity: usize,
    pub(crate) piety: usize,
}

pub(crate) fn make_equipment_main_stat(main_stat_id: usize, value: usize) -> MainStat {
    let mut main_stat = HashMap::new();
    main_stat.insert(main_stat_id, value);
    main_stat
}

pub(crate) fn make_race_main_stat(
    strength: usize,
    dexterity: usize,
    intelligence: usize,
    mind: usize,
) -> MainStat {
    let mut main_stat = HashMap::new();
    main_stat.insert(0, strength);
    main_stat.insert(1, dexterity);
    main_stat.insert(2, intelligence);
    main_stat.insert(3, mind);
    main_stat
}

impl SubStats {
    pub(crate) fn new(
        crit: Option<usize>,
        dh: Option<usize>,
        det: Option<usize>,
        speed: Option<usize>,
        tenacity: Option<usize>,
        piety: Option<usize>,
    ) -> Self {
        SubStats {
            critical_strike: crit.unwrap_or_else(|| 0),
            direct_hit: dh.unwrap_or_else(|| 0),
            determination: det.unwrap_or_else(|| 0),
            speed: speed.unwrap_or_else(|| 0),
            tenacity: tenacity.unwrap_or_else(|| 0),
            piety: piety.unwrap_or_else(|| 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stat::{make_equipment_main_stat, make_race_main_stat, SubStats};
    #[test]
    fn test_substat() {
        let substats = SubStats::new(Some(100), None, Some(102), None, None, None);

        assert_eq!(substats.critical_strike, 100);
        assert_eq!(substats.direct_hit, 0);
        assert_eq!(substats.determination, 102);
        assert_eq!(substats.speed, 0);
        assert_eq!(substats.tenacity, 0);
        assert_eq!(substats.piety, 0);
    }

    #[test]
    fn test_equipment_mainstat() {
        let mainstat = make_equipment_main_stat(3, 100);

        assert_eq!(mainstat.get(&0), None);
        assert_eq!(mainstat.get(&1), None);
        assert_eq!(mainstat.get(&2), None);
        assert_eq!(mainstat.get(&3), Some(&100));
    }

    #[test]
    fn test_character_mainstat() {
        let mainstat = make_race_main_stat(23, 21, 22, 24);

        assert_eq!(mainstat.get(&0), Some(&23));
        assert_eq!(mainstat.get(&1), Some(&21));
        assert_eq!(mainstat.get(&2), Some(&22));
        assert_eq!(mainstat.get(&3), Some(&24));
    }
}
