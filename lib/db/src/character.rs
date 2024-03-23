use crate::jobclass::JobClass;
use crate::race::Race;
use crate::stat::{MainStat, SubStats};

pub trait Character {
    fn get_race(&self) -> Race;
    fn get_job(&self) -> JobClass;
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
