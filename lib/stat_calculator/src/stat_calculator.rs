pub trait StatCalculator {
    fn calculate_crit_stat_ladder(&self, crit: usize) -> usize;
    fn calculate_dh_stat_ladder(&self, dh: usize) -> usize;
    fn calculate_det_stat_ladder(&self, det: usize) -> usize;
    fn calculate_speed_stat_ladder(&self, speed: usize) -> usize;
}

pub struct EndwalkerStatCalculator {
    /// Base main stat for each main stat job, since
    /// base dexterity for a Dexterity Job(ex) Ninja, Bard)
    /// is different from base main stat of jobs whose main stat isn't dexterity.
    base_mainstats: MainStat,
    base_substats: SubStats
}

impl EndwalkerStatCalculator
