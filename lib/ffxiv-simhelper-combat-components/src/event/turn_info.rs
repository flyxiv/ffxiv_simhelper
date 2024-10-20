use crate::live_objects::turn_type::FfxivTurnType;
use crate::types::TimeType;

static TWO_MINUTES_IN_MILLISECOND: TimeType = 120000;
static BURST_START_TIME_MILLISECOND: TimeType = 7000;
static BURST_END_TIME_MILLISECOND: TimeType = 27000;

/// Extracts needed time information from the player turn event.
pub struct TurnInfo {
    pub turn_type: FfxivTurnType,
    pub next_gcd_millisecond: TimeType,
    pub lower_bound_millisecond: TimeType,
}

impl TurnInfo {
    /// Currently, FFXIV's burst time is fixated to 2 minutes,
    /// Meaning that burst happens ```every 2n:07 - 2n:27, where n is an integer greater or equal than 0```
    /// so it is possible to calculate how much millisecond is left until the next burst time.
    /// This data is used to notify the user whether current time is bursting phase,
    /// or on jobs like RPR that need preparation before the burst phase to decide whether to start their preparation.
    ///
    /// ex)
    /// ```rust
    /// use ffxiv_simhelper_combat_components::event::turn_info::TurnInfo;
    /// use ffxiv_simhelper_combat_components::live_objects::turn_type::FfxivTurnType;
    ///
    /// let turn_info1 = TurnInfo {
    ///     turn_type: FfxivTurnType::Gcd,
    ///     next_gcd_millisecond: 0,
    ///     
    ///     // 1:32 -> 35 seconds left until 2:07, which is the beginning of the next burst phase
    ///     lower_bound_millisecond: 92000,
    ///
    /// };
    ///
    /// let turn_info2 = TurnInfo {
    ///     turn_type: FfxivTurnType::Gcd,
    ///     next_gcd_millisecond: 0,
    ///
    ///     // 2:26 -> already in burst -> 0 seconds
    ///     lower_bound_millisecond: 146000,
    /// };
    ///
    /// let turn_info3 = TurnInfo {
    ///     turn_type: FfxivTurnType::Gcd,
    ///     next_gcd_millisecond: 0,
    ///
    ///     // 2:27.100 -> 2min burst just ended, and is 1:39.900 left until 4:07, which is the beginning of the next burst phase
    ///     lower_bound_millisecond: 147100,
    /// };
    ///
    /// assert_eq!(turn_info1.get_next_burst_time(), 35000);
    /// assert_eq!(turn_info2.get_next_burst_time(), 0);
    /// assert_eq!(turn_info3.get_next_burst_time(), 99900);
    /// ```
    pub fn get_next_burst_time(&self) -> TimeType {
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
