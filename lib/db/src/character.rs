use crate::jobclass::JobClass;
use crate::race::Race;
use crate::stat::{MainStat, SubStats};

#[derive(PartialEq, Eq)]
pub(crate) struct Character {
    weapon_attack: usize,
    race: Race,
    class: JobClass,
    main_stats: MainStat,
    sub_stats: SubStats,
}
