use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::event_ticker::{EventTicker, TickerKey, GLOBAL_TICK_INTERVAL_MILLISECOND};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::status::debuff_status::DebuffStatus;
use crate::types::StatusTable;
use crate::{IdType, TimeType};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

/// Global DOT ticker
#[derive(Clone)]
pub struct GlobalTicker {
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

    fn get_id(&self) -> TickerKey {
        TickerKey::new(self.id, self.id)
    }

    fn set_event_queue(&mut self, event_queue: Rc<RefCell<FfxivEventQueue>>) {
        self.event_queue = event_queue;
    }

    fn has_initial_tick(&self) -> bool {
        false
    }
    fn get_remaining_time(&self) -> TimeType {
        TimeType::MAX - 1
    }
}

impl GlobalTicker {
    pub fn new(id: IdType, ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>) -> Self {
        Self {
            id,
            event_queue: ffxiv_event_queue,
        }
    }
}
