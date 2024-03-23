/// Implements functions needed to save Medicine data.
/// Medicines only need three categories: MainStat, Value, and Duration.
use crate::stat::{MainStatTrait, MainStats};

pub struct Medicine {
    pub(crate) main_stats: MainStats,
    pub(crate) value: usize,
    pub(crate) duration: usize,
}

impl Medicine {
    pub(crate) fn new(main_stats: MainStats, value: usize, duration: usize) -> Self {
        Medicine {
            main_stats,
            value,
            duration,
        }
    }
}

impl MainStatTrait for Medicine {
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
