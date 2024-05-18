use crate::rotation::job_priorities::ninja::NinjaPriorityTable;
use crate::rotation::job_priorities::priority_table::{Opener, PriorityTable};
use crate::rotation::job_priorities::sage::SagePriorityTable;
use crate::rotation::SkillPriorityInfo;
use crate::IdType;

#[derive(Clone)]
pub(crate) enum FfxivPriorityTable {
    Ninja(NinjaPriorityTable),
    Sage(SagePriorityTable),
}

impl PriorityTable for FfxivPriorityTable {
    fn get_opener_len(&self) -> usize {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_opener_len(),
            FfxivPriorityTable::Sage(sage) => sage.get_opener_len(),
        }
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_opener_at(index),
            FfxivPriorityTable::Sage(sage) => sage.get_opener_at(index),
        }
    }

    fn get_gcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_gcd_priority_table(),
            FfxivPriorityTable::Sage(sage) => sage.get_gcd_priority_table(),
        }
    }

    fn get_ogcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_ogcd_priority_table(),
            FfxivPriorityTable::Sage(sage) => sage.get_ogcd_priority_table(),
        }
    }

    fn increment_turn(&self) {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.increment_turn(),
            FfxivPriorityTable::Sage(sage) => sage.increment_turn(),
        }
    }

    fn get_turn_count(&self) -> IdType {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_turn_count(),
            FfxivPriorityTable::Sage(sage) => sage.get_turn_count(),
        }
    }
}
