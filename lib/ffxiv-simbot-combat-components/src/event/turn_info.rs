use crate::live_objects::turn_type::FfxivTurnType;
use crate::types::TimeType;

static TWO_MINUTES_IN_MILLISECOND: TimeType = 120000;
static BURST_START_TIME_MILLISECOND: TimeType = 7000;
static BURST_END_TIME_MILLISECOND: TimeType = 27000;

/// Extracts needed information from the player turn event.
pub struct TurnInfo {
    pub turn_type: FfxivTurnType,
    pub next_gcd_millisecond: TimeType,
    pub lower_bound_millisecond: TimeType,
}

impl TurnInfo {
    pub(crate) fn get_next_burst_time(&self) -> TimeType {
        let burst_number_offset = self.lower_bound_millisecond % TWO_MINUTES_IN_MILLISECOND;

        if burst_number_offset < BURST_START_TIME_MILLISECOND {
            BURST_START_TIME_MILLISECOND - burst_number_offset
        } else if burst_number_offset > BURST_END_TIME_MILLISECOND {
            (BURST_START_TIME_MILLISECOND + TWO_MINUTES_IN_MILLISECOND) - burst_number_offset
        } else {
            0
        }
    }
}
