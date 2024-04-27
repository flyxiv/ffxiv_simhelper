use serde::{Deserialize, Serialize};
pub type StatType = i32;
pub type HpType = usize;

pub trait StatFrom<T>: Sized {
    fn stat_from(stat: &T) -> Self;
}

/// Saves the main stats of the equipment/character/race.
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug, Serialize)]
pub struct MainStats {
    pub strength: StatType,
    pub dexterity: StatType,
    pub vitality: StatType,
    pub intelligence: StatType,
    pub mind: StatType,
}

/// Trait used for in-game entities that have main stat.
pub trait MainStatTrait: Sized {
    fn get_strength(&self) -> StatType;
    fn get_dexterity(&self) -> StatType;
    fn get_vitality(&self) -> StatType;
    fn get_intelligence(&self) -> StatType;
    fn get_mind(&self) -> StatType;
}

impl MainStatTrait for MainStats {
    fn get_strength(&self) -> StatType {
        self.strength
    }

    fn get_dexterity(&self) -> StatType {
        self.dexterity
    }

    fn get_vitality(&self) -> StatType {
        self.vitality
    }

    fn get_intelligence(&self) -> StatType {
        self.intelligence
    }

    fn get_mind(&self) -> StatType {
        self.mind
    }
}

/// add main stats of two different MainStatTrait entities
pub fn add_main_stats<T, U>(main_stats1: &T, main_stats2: &U) -> MainStats
where
    T: MainStatTrait,
    U: MainStatTrait,
{
    MainStats {
        strength: main_stats1.get_strength() + main_stats2.get_strength(),
        dexterity: main_stats1.get_dexterity() + main_stats2.get_dexterity(),
        vitality: main_stats1.get_vitality() + main_stats2.get_vitality(),
        intelligence: main_stats1.get_intelligence() + main_stats2.get_intelligence(),
        mind: main_stats1.get_mind() + main_stats2.get_mind(),
    }
}

/// add sub stats of two different MainStatTrait entities
pub fn add_sub_stats<T, U>(sub_stats1: &T, sub_stats2: &U) -> SubStats
where
    T: SubStatTrait,
    U: SubStatTrait,
{
    SubStats {
        critical_strike: sub_stats1.get_critical_strike() + sub_stats2.get_critical_strike(),
        direct_hit: sub_stats1.get_direct_hit() + sub_stats2.get_direct_hit(),
        determination: sub_stats1.get_determination() + sub_stats2.get_determination(),
        skill_speed: sub_stats1.get_skill_speed() + sub_stats2.get_skill_speed(),
        spell_speed: sub_stats1.get_spell_speed() + sub_stats2.get_spell_speed(),
        tenacity: sub_stats1.get_tenacity() + sub_stats2.get_tenacity(),
        piety: sub_stats1.get_piety() + sub_stats2.get_piety(),
    }
}

impl<T> StatFrom<T> for MainStats
where
    T: MainStatTrait,
{
    fn stat_from(main_stat_trait: &T) -> Self {
        MainStats {
            strength: main_stat_trait.get_strength(),
            dexterity: main_stat_trait.get_dexterity(),
            vitality: main_stat_trait.get_vitality(),
            intelligence: main_stat_trait.get_intelligence(),
            mind: main_stat_trait.get_mind(),
        }
    }
}

/// Trait for in-game entities that have sub stats
pub trait SubStatTrait: Sized {
    fn get_critical_strike(&self) -> StatType;
    fn get_direct_hit(&self) -> StatType;
    fn get_determination(&self) -> StatType;
    fn get_skill_speed(&self) -> StatType;
    fn get_spell_speed(&self) -> StatType;
    fn get_tenacity(&self) -> StatType;
    fn get_piety(&self) -> StatType;
}

impl<T> StatFrom<T> for SubStats
where
    T: SubStatTrait,
{
    fn stat_from(sub_stat_trait: &T) -> Self {
        SubStats {
            critical_strike: sub_stat_trait.get_critical_strike(),
            direct_hit: sub_stat_trait.get_direct_hit(),
            determination: sub_stat_trait.get_determination(),
            skill_speed: sub_stat_trait.get_skill_speed(),
            spell_speed: sub_stat_trait.get_spell_speed(),
            tenacity: sub_stat_trait.get_tenacity(),
            piety: sub_stat_trait.get_piety(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Deserialize, Serialize, Debug)]
pub struct SubStats {
    pub critical_strike: StatType,
    pub direct_hit: StatType,
    pub determination: StatType,
    pub skill_speed: StatType,
    pub spell_speed: StatType,
    pub tenacity: StatType,
    pub piety: StatType,
}

impl SubStatTrait for SubStats {
    fn get_critical_strike(&self) -> StatType {
        self.critical_strike
    }

    fn get_direct_hit(&self) -> StatType {
        self.direct_hit
    }

    fn get_determination(&self) -> StatType {
        self.determination
    }

    fn get_skill_speed(&self) -> StatType {
        self.skill_speed
    }

    fn get_spell_speed(&self) -> StatType {
        self.spell_speed
    }

    fn get_tenacity(&self) -> StatType {
        self.tenacity
    }

    fn get_piety(&self) -> StatType {
        self.piety
    }
}

/// Trait for stats that aren't included in main/sub stats.
pub(crate) trait SpecialStatTrait {
    fn get_hp(&self) -> HpType;
}

#[cfg(test)]
mod tests {
    use crate::stat::{MainStatTrait, MainStats, SubStatTrait, SubStats};

    #[test]
    fn main_stat_basic_test() {
        let main_stats = MainStats {
            strength: 1,
            dexterity: 2,
            vitality: 3,
            intelligence: 4,
            mind: 5,
        };

        assert_eq!(main_stats.get_strength(), 1);
        assert_eq!(main_stats.get_dexterity(), 2);
        assert_eq!(main_stats.get_vitality(), 3);
        assert_eq!(main_stats.get_intelligence(), 4);
        assert_eq!(main_stats.get_mind(), 5);
    }

    fn sub_stat_basic_test() {
        let substats = SubStats {
            critical_strike: 1,
            direct_hit: 2,
            determination: 3,
            skill_speed: 4,
            spell_speed: 5,
            tenacity: 6,
            piety: 7,
        };

        assert_eq!(substats.get_critical_strike(), 1);
        assert_eq!(substats.get_direct_hit(), 2);
        assert_eq!(substats.get_determination(), 3);
        assert_eq!(substats.get_skill_speed(), 4);
        assert_eq!(substats.get_spell_speed(), 5);
        assert_eq!(substats.get_tenacity(), 6);
        assert_eq!(substats.get_piety(), 7);
    }
}
