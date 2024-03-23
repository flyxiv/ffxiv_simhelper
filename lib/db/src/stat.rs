use std::ops;

/// Saves the main stats of the equipment/character/race.
#[derive(Eq, PartialEq, Clone, Copy)]
pub(crate) struct MainStats {
    pub(crate) strength: usize,
    pub(crate) dexterity: usize,
    pub(crate) vitality: usize,
    pub(crate) intelligence: usize,
    pub(crate) mind: usize,
}

/// Trait used for in-game entities that have main stat.
pub(crate) trait MainStatTrait {
    fn get_strength(&self) -> usize;
    fn get_dexterity(&self) -> usize;
    fn get_vitality(&self) -> usize;
    fn get_intelligence(&self) -> usize;
    fn get_mind(&self) -> usize;
}

impl MainStatTrait for MainStats {
    fn get_strength(&self) -> usize {
        self.strength
    }

    fn get_dexterity(&self) -> usize {
        self.dexterity
    }

    fn get_vitality(&self) -> usize {
        self.vitality
    }

    fn get_intelligence(&self) -> usize {
        self.intelligence
    }

    fn get_mind(&self) -> usize {
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

impl<T> From<T> for MainStats
where
    T: MainStatTrait,
{
    fn from(main_stat_trait: T) -> Self {
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
pub(crate) trait SubStatTrait {
    fn get_critical_strike(&self) -> usize;
    fn get_direct_hit(&self) -> usize;
    fn get_determination(&self) -> usize;
    fn get_skill_speed(&self) -> usize;
    fn get_spell_speed(&self) -> usize;
    fn get_tenacity(&self) -> usize;
    fn get_piety(&self) -> usize;

    fn add_sub_stats(&self, sub_stats: &SubStats) -> SubStats {
        SubStats {
            critical_strike: self.get_critical_strike() + sub_stats.critical_strike,
            direct_hit: self.get_direct_hit() + sub_stats.direct_hit,
            determination: self.get_determination() + sub_stats.determination,
            skill_speed: self.get_skill_speed() + sub_stats.skill_speed,
            spell_speed: self.get_spell_speed() + sub_stats.spell_speed,
            tenacity: self.get_tenacity() + sub_stats.tenacity,
            piety: self.get_piety() + sub_stats.piety,
        }
    }
}

impl<T> From<T> for SubStats
where
    T: SubStatTrait,
{
    fn from(sub_stat_trait: T) -> Self {
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

#[derive(Eq, PartialEq)]
pub(crate) struct SubStats {
    pub(crate) critical_strike: usize,
    pub(crate) direct_hit: usize,
    pub(crate) determination: usize,
    pub(crate) skill_speed: usize,
    pub(crate) spell_speed: usize,
    pub(crate) tenacity: usize,
    pub(crate) piety: usize,
}

impl SubStatTrait for SubStats {
    fn get_critical_strike(&self) -> usize {
        self.critical_strike
    }

    fn get_direct_hit(&self) -> usize {
        self.direct_hit
    }

    fn get_determination(&self) -> usize {
        self.determination
    }

    fn get_skill_speed(&self) -> usize {
        self.skill_speed
    }

    fn get_spell_speed(&self) -> usize {
        self.spell_speed
    }

    fn get_tenacity(&self) -> usize {
        self.tenacity
    }

    fn get_piety(&self) -> usize {
        self.piety
    }
}

/// Trait for stats that aren't included in main/sub stats.
pub(crate) trait SpecialStatTrait {
    fn get_hp(&self) -> usize;
}
