/// Implements functions needed to save Materia data
use crate::stat::SubStats;

/// Class for Materias
/// Materias only need three fields: substat, value, and whether it can be pentamelded
/// Only Account Combat Materias as of now.
#[derive(Eq, PartialEq)]
pub struct Materia {
    substat: SubStats,
    value: usize,
    pub(crate) penta_meldable: bool,
}

impl Materia {
    pub(crate) fn new(substat: SubStats, value: usize, penta_meldable: bool) -> Self {
        Materia {
            substat,
            value,
            penta_meldable,
        }
    }
}
