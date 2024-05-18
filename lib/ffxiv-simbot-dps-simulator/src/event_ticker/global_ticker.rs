use crate::event_ticker::{EventTicker, GLOBAL_TICK_INTERVAL_MILLISECOND};
use ffxiv_simbot_combat_components::event::ffxiv_event::FfxivEvent;
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::status::debuff_status::DebuffStatus;
use ffxiv_simbot_combat_components::{IdType, StatusTable, TimeType};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

/// Global DOT ticker

pub(crate) struct GlobalTicker {
    id: IdType,
    event_queue: Rc<RefCell<FfxivEventQueue>>,
}

impl EventTicker for GlobalTicker {
    fn generate_tick_event(
        &mut self,
        current_time_millisecond: TimeType,
        _: Option<Rc<RefCell<FfxivPlayer>>>,
        _: StatusTable<DebuffStatus>,
    ) {
        self.event_queue
            .borrow_mut()
            .push(Reverse(FfxivEvent::DotTick(current_time_millisecond)));
    }

    fn update_remaining_time(&mut self, _: TimeType) {}

    fn get_event_queue(&self) -> Rc<RefCell<FfxivEventQueue>> {
        self.event_queue.clone()
    }

    fn get_tick_interval(&self) -> TimeType {
        GLOBAL_TICK_INTERVAL_MILLISECOND
    }

    fn get_player_id(&self) -> Option<IdType> {
        None
    }
}

impl IdEntity for GlobalTicker {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl GlobalTicker {
    pub(crate) fn new(id: IdType, ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>) -> Self {
        Self {
            id,
            event_queue: ffxiv_event_queue,
        }
    }
}
