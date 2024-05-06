use crate::event_ticker::{EventTicker, TICK_MILLISECOND};
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::{IdType, TimeType};

/// Generator of ticks that run independant of any other events
/// ex) Bard's Song Ticks

pub(crate) struct IndependentTicker {
    id: IdType,
    tick_millisecond: TimeType,
    most_recent_tick_time_millisecond: TimeType,
}

impl EventTicker for IndependentTicker {
    fn add_next_tick_event_to_queue(&mut self, _: Option<TimeType>) -> TimeType {
        let next_tick_time_millisecond =
            self.most_recent_tick_time_millisecond + self.tick_millisecond;

        self.update_tick(next_tick_time_millisecond);
        next_tick_time_millisecond
    }

    fn get_related_player_id(&self) -> Option<IdType> {
        None
    }
}

impl IdEntity for IndependentTicker {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl IndependentTicker {
    pub(crate) fn new(id: IdType, start_time_millisecond: TimeType) -> Self {
        Self {
            id,
            tick_millisecond: TICK_MILLISECOND,
            most_recent_tick_time_millisecond: start_time_millisecond,
        }
    }

    pub(crate) fn new_with_tick(
        id: IdType,
        tick_millisecond: TimeType,
        start_time_millisecond: TimeType,
    ) -> Self {
        Self {
            id,
            tick_millisecond,
            most_recent_tick_time_millisecond: start_time_millisecond,
        }
    }
}
