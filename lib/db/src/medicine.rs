/// Implements functions needed to save Medicine data.
/// Medicines only need three categories: MainStat, Value, and Duration.
use crate::stat::{MainStatTrait, MainStats, StatType};

pub struct Medicine {
    pub(crate) main_stats: MainStats,
    pub(crate) duration: StatType,
}

impl Medicine {
    pub(crate) fn new(main_stats: MainStats, duration: StatType) -> Self {
        Medicine {
            main_stats,
            duration,
        }
    }
}

impl MainStatTrait for Medicine {
    fn get_strength(&self) -> StatType {
        self.main_stats.get_strength()
    }

    fn get_dexterity(&self) -> StatType {
        self.main_stats.get_dexterity()
    }
    fn get_vitality(&self) -> StatType {
        self.main_stats.get_vitality()
    }

    fn get_intelligence(&self) -> StatType {
        self.main_stats.get_intelligence()
    }

    fn get_mind(&self) -> StatType {
        self.main_stats.get_mind()
    }
}

#[cfg(test)]
mod tests {
    use crate::medicine::Medicine;
    use crate::stat::{MainStatTrait, MainStats};

    #[test]
    fn medicine_basic_test() {
        let medicine = Medicine::new(
            MainStats {
                strength: 1,
                dexterity: 2,
                vitality: 3,
                intelligence: 4,
                mind: 5,
            },
            20,
        );

        assert_eq!(medicine.get_strength(), 1);
        assert_eq!(medicine.get_dexterity(), 2);
        assert_eq!(medicine.get_vitality(), 3);
        assert_eq!(medicine.get_intelligence(), 4);
        assert_eq!(medicine.get_mind(), 5);
        assert_eq!(medicine.duration, 20);
    }
}
