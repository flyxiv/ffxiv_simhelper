/// Implements functions needed to save Materia data
use crate::stat::{StatType, SubStatTrait, SubStats};

/// Class for Materias
/// Materias only need three fields: substat, value, and whether it can be pentamelded
/// Only Account Combat Materias as of now.
#[derive(Eq, PartialEq, Clone)]
pub struct Materia {
    substat: SubStats,
    pub(crate) penta_meldable: bool,
}

impl Materia {
    pub(crate) fn new(substat: SubStats, penta_meldable: bool) -> Self {
        Materia {
            substat,
            penta_meldable,
        }
    }
}

impl SubStatTrait for Materia {
    fn get_critical_strike(&self) -> StatType {
        self.substat.get_critical_strike()
    }

    fn get_direct_hit(&self) -> StatType {
        self.substat.get_direct_hit()
    }

    fn get_determination(&self) -> StatType {
        self.substat.get_determination()
    }

    fn get_skill_speed(&self) -> StatType {
        self.substat.get_skill_speed()
    }

    fn get_spell_speed(&self) -> StatType {
        self.substat.get_spell_speed()
    }

    fn get_tenacity(&self) -> StatType {
        self.substat.get_tenacity()
    }

    fn get_piety(&self) -> StatType {
        self.substat.get_piety()
    }
}
