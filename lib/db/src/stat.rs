use std::collections::HashMap;

/// Saves the main stat of the equipment/character.
/// 0 -> strength, 1 -> dexterity, 2 -> intelligence, 3 -> mind
/// For equipments, MainStat Ideally should have only one key-value pair,
/// since in FFXIV no equipment has more than one main stat as of now.
pub(crate) struct MainStat {
    strength: usize,
    dexterity: usize,
    vitality: usize,
    intelligence: usize,
    mind: usize,
}

/// Trait used for in-game entities that have main stat.
pub(crate) trait MainStatTrait {
    fn get_strength(&self) -> usize;
    fn get_dexterity(&self) -> usize;
    fn get_vitality(&self) -> usize;
    fn get_intelligence(&self) -> usize;
    fn get_mind(&self) -> usize;
}

impl MainStatTrait for MainStat {
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

/// Trait for in-game entities that have sub stats
pub(crate) trait SubStatTrait {
    fn get_critical_strike(&self) -> usize;
    fn get_direct_hit(&self) -> usize;
    fn get_determination(&self) -> usize;
    fn get_skill_speed(&self) -> usize;
    fn get_spell_speed(&self) -> usize;
    fn get_tenacity(&self) -> usize;
    fn get_piety(&self) -> usize;
}

pub(crate) struct SubStat {
    critical_strike: usize,
    direct_hit: usize,
    determination: usize,
    skill_speed: usize,
    spell_speed: usize,
    tenacity: usize,
    piety: usize,
}

impl SubStatTrait for SubStat {
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
