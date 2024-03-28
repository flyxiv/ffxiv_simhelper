/// Implements functions needed to save Materia data
use crate::stat::{StatType, SubStatTrait, SubStats};

/// Class for Materias
/// Materias only need three fields: substat, value, and whether it can be pentamelded
/// Only Account Combat Materias as of now.
#[derive(Eq, PartialEq, Clone)]
pub struct Materia {
    sub_stats: SubStats,
    pub(crate) penta_meldable: bool,
}

impl Materia {
    pub(crate) fn new(sub_stats: SubStats, penta_meldable: bool) -> Self {
        Materia {
            sub_stats,
            penta_meldable,
        }
    }
}

impl SubStatTrait for Materia {
    fn get_critical_strike(&self) -> StatType {
        self.sub_stats.get_critical_strike()
    }

    fn get_direct_hit(&self) -> StatType {
        self.sub_stats.get_direct_hit()
    }

    fn get_determination(&self) -> StatType {
        self.sub_stats.get_determination()
    }

    fn get_skill_speed(&self) -> StatType {
        self.sub_stats.get_skill_speed()
    }

    fn get_spell_speed(&self) -> StatType {
        self.sub_stats.get_spell_speed()
    }

    fn get_tenacity(&self) -> StatType {
        self.sub_stats.get_tenacity()
    }

    fn get_piety(&self) -> StatType {
        self.sub_stats.get_piety()
    }
}

#[cfg(test)]
mod tests {
    use crate::materia::Materia;
    use crate::stat::{SubStatTrait, SubStats};
    #[test]
    fn materia_basic_test() {
        let materia = Materia {
            sub_stats: SubStats {
                critical_strike: 76,
                direct_hit: 0,
                determination: 0,
                skill_speed: 0,
                spell_speed: 0,
                tenacity: 0,
                piety: 0,
            },
            penta_meldable: true,
        };

        assert_eq!(materia.get_critical_strike(), 7);
        assert_eq!(materia.get_direct_hit(), 0);
        assert_eq!(materia.get_determination(), 0);
        assert_eq!(materia.get_skill_speed(), 0);
        assert_eq!(materia.get_spell_speed(), 0);
        assert_eq!(materia.get_tenacity(), 0);
        assert_eq!(materia.get_piety(), 0);
        assert!(materia.penta_meldable);
    }

    #[test]
    fn materia_all_stats_test() {
        let materia = Materia {
            sub_stats: SubStats {
                critical_strike: 0,
                direct_hit: 1,
                determination: 2,
                skill_speed: 3,
                spell_speed: 4,
                tenacity: 5,
                piety: 6,
            },
            penta_meldable: false,
        };

        assert_eq!(materia.get_critical_strike(), 0);
        assert_eq!(materia.get_direct_hit(), 1);
        assert_eq!(materia.get_determination(), 2);
        assert_eq!(materia.get_skill_speed(), 3);
        assert_eq!(materia.get_spell_speed(), 4);
        assert_eq!(materia.get_tenacity(), 5);
        assert_eq!(materia.get_piety(), 6);
        assert!(!materia.penta_meldable);
    }
}
