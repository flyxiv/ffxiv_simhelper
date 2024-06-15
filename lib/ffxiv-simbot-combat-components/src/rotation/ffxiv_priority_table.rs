use crate::jobs_skill_data::bard::priorities::BardPriorityTable;
use crate::jobs_skill_data::black_mage::priorities::BlackmagePriorityTable;
use crate::jobs_skill_data::dancer::priorities::DancerPriorityTable;
use crate::jobs_skill_data::dragoon::priorities::DragoonPriorityTable;
use crate::jobs_skill_data::monk::priorities::MonkPriorityTable;
use crate::jobs_skill_data::ninja::priorities::NinjaPriorityTable;
use crate::jobs_skill_data::paladin::priorities::PaladinPriorityTable;
use crate::jobs_skill_data::sage::priorities::SagePriorityTable;
use crate::jobs_skill_data::scholar::priorities::ScholarPriorityTable;
use crate::jobs_skill_data::warrior::priorities::WarriorPriorityTable;
use crate::jobs_skill_data::white_mage::priorities::WhitemagePriorityTable;
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::IdType;

#[derive(Clone)]
pub(crate) enum FfxivPriorityTable {
    Ninja(NinjaPriorityTable),
    Sage(SagePriorityTable),
    Bard(BardPriorityTable),
    Dancer(DancerPriorityTable),
    Monk(MonkPriorityTable),
    Dragoon(DragoonPriorityTable),
    Blackmage(BlackmagePriorityTable),
    Whitemage(WhitemagePriorityTable),
    Paladin(PaladinPriorityTable),
    Warrior(WarriorPriorityTable),
    Scholar(ScholarPriorityTable),
}

impl PriorityTable for FfxivPriorityTable {
    fn get_opener_len(&self) -> usize {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_opener_len(),
            FfxivPriorityTable::Sage(sage) => sage.get_opener_len(),
            FfxivPriorityTable::Bard(bard) => bard.get_opener_len(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_opener_len(),
            FfxivPriorityTable::Monk(monk) => monk.get_opener_len(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_opener_len(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_opener_len(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_opener_len(),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_opener_len(),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_opener_len(),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_opener_len(),
        }
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_opener_at(index),
            FfxivPriorityTable::Sage(sage) => sage.get_opener_at(index),
            FfxivPriorityTable::Bard(bard) => bard.get_opener_at(index),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_opener_at(index),
            FfxivPriorityTable::Monk(monk) => monk.get_opener_at(index),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_opener_at(index),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_opener_at(index),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_opener_at(index),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_opener_at(index),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_opener_at(index),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_opener_at(index),
        }
    }

    fn get_gcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_gcd_priority_table(),
            FfxivPriorityTable::Sage(sage) => sage.get_gcd_priority_table(),
            FfxivPriorityTable::Bard(bard) => bard.get_gcd_priority_table(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_gcd_priority_table(),
            FfxivPriorityTable::Monk(monk) => monk.get_gcd_priority_table(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_gcd_priority_table(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_gcd_priority_table(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_gcd_priority_table(),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_gcd_priority_table(),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_gcd_priority_table(),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_gcd_priority_table(),
        }
    }

    fn get_ogcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_ogcd_priority_table(),
            FfxivPriorityTable::Sage(sage) => sage.get_ogcd_priority_table(),
            FfxivPriorityTable::Bard(bard) => bard.get_ogcd_priority_table(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_ogcd_priority_table(),
            FfxivPriorityTable::Monk(monk) => monk.get_ogcd_priority_table(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_ogcd_priority_table(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_ogcd_priority_table(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_ogcd_priority_table(),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_ogcd_priority_table(),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_ogcd_priority_table(),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_ogcd_priority_table(),
        }
    }

    fn increment_turn(&self) {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.increment_turn(),
            FfxivPriorityTable::Sage(sage) => sage.increment_turn(),
            FfxivPriorityTable::Bard(bard) => bard.increment_turn(),
            FfxivPriorityTable::Dancer(dancer) => dancer.increment_turn(),
            FfxivPriorityTable::Monk(monk) => monk.increment_turn(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.increment_turn(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.increment_turn(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.increment_turn(),
            FfxivPriorityTable::Paladin(paladin) => paladin.increment_turn(),
            FfxivPriorityTable::Warrior(warrior) => warrior.increment_turn(),
            FfxivPriorityTable::Scholar(scholar) => scholar.increment_turn(),
        }
    }

    fn get_turn_count(&self) -> IdType {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_turn_count(),
            FfxivPriorityTable::Sage(sage) => sage.get_turn_count(),
            FfxivPriorityTable::Bard(bard) => bard.get_turn_count(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_turn_count(),
            FfxivPriorityTable::Monk(monk) => monk.get_turn_count(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_turn_count(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_turn_count(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_turn_count(),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_turn_count(),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_turn_count(),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_turn_count(),
        }
    }
}
