use crate::event_ticker::{EventTicker, TICK_MILLISECOND};
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::{IdType, TimeType};

/// Global DOT ticker

pub(crate) struct GlobalTicker {
    id: IdType,
    most_recent_tick_time_millisecond: TimeType,
}

impl EventTicker for GlobalTicker {
    fn add_next_tick_event_to_queue(&mut self, _: Option<TimeType>) -> TimeType {
        let next_tick_time_millisecond = self.most_recent_tick_time_millisecond + TICK_MILLISECOND;

        self.update_tick(next_tick_time_millisecond);
        next_tick_time_millisecond
    }

    fn get_related_player_id(&self) -> Option<IdType> {
        None
    }
}

impl IdEntity for GlobalTicker {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl GlobalTicker {
    pub(crate) fn update_tick(&mut self, time: TimeType) {
        self.most_recent_tick_time_millisecond = time;
    }

    pub(crate) fn new(id: IdType) -> Self {
        Self {
            id,
            most_recent_tick_time_millisecond: 0,
        }
    }
}
