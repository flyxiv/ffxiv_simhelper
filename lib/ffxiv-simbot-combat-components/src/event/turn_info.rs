use crate::live_objects::turn_type::FfxivTurnType;
use crate::TimeType;

static TWO_MINUTES_IN_MILLISECOND: TimeType = 120000;
static BURST_START_TIME_MILLISECOND: TimeType = 7000;
static BURST_END_TIME_MILLISECOND: TimeType = 23000;

/// Extracts needed information from the player turn event.
pub(crate) struct TurnInfo {
    pub(crate) turn_type: FfxivTurnType,
    pub(crate) next_gcd_millisecond: TimeType,
    pub(crate) lower_bound_millisecond: TimeType,
}

impl TurnInfo {
    pub(crate) fn get_next_burst_time(&self) -> TimeType {
        let burst_number_offset = self.lower_bound_millisecond % TWO_MINUTES_IN_MILLISECOND;

        if burst_number_offset < BURST_START_TIME_MILLISECOND {
            BURST_START_TIME_MILLISECOND - burst_number_offset
        } else if burst_number_offset > BURST_END_TIME_MILLISECOND {
            burst_number_offset - BURST_END_TIME_MILLISECOND
        } else {
            0
        }
    }
}
