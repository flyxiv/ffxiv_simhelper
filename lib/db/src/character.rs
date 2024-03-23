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

impl Character {
    fn create(race: Race, job_class: JobClass) -> Self {
        Character {
            weapon_attack: 0,
            race,
            job_class,
            main_stats: MainStat::new(0, 0, 0, 0, 0, 0),
        }
    }
}
