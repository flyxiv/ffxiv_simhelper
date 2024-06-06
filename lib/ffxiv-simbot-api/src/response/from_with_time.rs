use crate::response::aggregate_damage_simulation_data::PlayerDamageAggregate;
use crate::response::simulation_api_response::SimulationSummaryResponse;
use ffxiv_simbot_combat_components::{DamageType, DpsType, TimeType};

pub(crate) trait FromWithTime<T>: Sized {
    fn from_with_time(value: T, time_millisecond: TimeType) -> Self;
}
