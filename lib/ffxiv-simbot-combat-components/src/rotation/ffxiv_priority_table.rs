use crate::jobs_skill_data::bard::priorities::BardPriorityTable;
use crate::jobs_skill_data::dancer::priorities::DancerPriorityTable;
use crate::jobs_skill_data::ninja::priorities::NinjaPriorityTable;
use crate::jobs_skill_data::sage::priorities::SagePriorityTable;
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::IdType;

#[derive(Clone)]
pub(crate) enum FfxivPriorityTable {
    Ninja(NinjaPriorityTable),
    Sage(SagePriorityTable),
    Bard(BardPriorityTable),
    Dancer(DancerPriorityTable),
}

impl PriorityTable for FfxivPriorityTable {
    fn get_opener_len(&self) -> usize {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_opener_len(),
            FfxivPriorityTable::Sage(sage) => sage.get_opener_len(),
            FfxivPriorityTable::Bard(bard) => bard.get_opener_len(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_opener_len(),
        }
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_opener_at(index),
            FfxivPriorityTable::Sage(sage) => sage.get_opener_at(index),
            FfxivPriorityTable::Bard(bard) => bard.get_opener_at(index),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_opener_at(index),
        }
    }

    fn get_gcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_gcd_priority_table(),
            FfxivPriorityTable::Sage(sage) => sage.get_gcd_priority_table(),
            FfxivPriorityTable::Bard(bard) => bard.get_gcd_priority_table(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_gcd_priority_table(),
        }
    }

    fn get_ogcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_ogcd_priority_table(),
            FfxivPriorityTable::Sage(sage) => sage.get_ogcd_priority_table(),
            FfxivPriorityTable::Bard(bard) => bard.get_ogcd_priority_table(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_ogcd_priority_table(),
        }
    }

    fn increment_turn(&self) {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.increment_turn(),
            FfxivPriorityTable::Sage(sage) => sage.increment_turn(),
            FfxivPriorityTable::Bard(bard) => bard.increment_turn(),
            FfxivPriorityTable::Dancer(dancer) => dancer.increment_turn(),
        }
    }

    fn get_turn_count(&self) -> IdType {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_turn_count(),
            FfxivPriorityTable::Sage(sage) => sage.get_turn_count(),
            FfxivPriorityTable::Bard(bard) => bard.get_turn_count(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_turn_count(),
        }
    }
}
