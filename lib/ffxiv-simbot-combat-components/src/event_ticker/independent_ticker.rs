use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::event_ticker::{EventTicker, PercentType, TickerKey, GLOBAL_TICK_INTERVAL_MILLISECOND};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::status::debuff_status::DebuffStatus;
use crate::types::StatusTable;
use crate::types::{IdType, TimeType};
use log::debug;
use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

/// Generator of ticks that run independent of any other events
/// ex) Bard's Song Ticks
#[derive(Clone)]
pub struct IndependentTicker {
    id: IdType,
    player_id: IdType,
    tick_millisecond: TimeType,
    time_left_millisecond: TimeType,
    tick_event: Vec<FfxivEvent>,
    ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    proc_percent: PercentType,
    initial_tick: bool,
}

impl EventTicker for IndependentTicker {
    fn generate_tick_event(
        &mut self,
        current_time_millisecond: TimeType,
        _: Option<Rc<RefCell<FfxivPlayer>>>,
        _: StatusTable<DebuffStatus>,
    ) {
        if self.tick_millisecond <= self.time_left_millisecond {
            let proc_value = thread_rng().gen_range(0..100);
            if proc_value <= self.proc_percent {
                let events = self.generate_event(current_time_millisecond);

                for event in events {
                    self.ffxiv_event_queue.borrow_mut().push(Reverse(event));
                }
            }
        }
    }

    fn update_remaining_time(&mut self, elapsed_time: TimeType) {
        self.time_left_millisecond -= elapsed_time;
        debug!(
            "ticker {} remaining time: {}",
            self.id, self.time_left_millisecond
        );
    }

    fn get_event_queue(&self) -> Rc<RefCell<FfxivEventQueue>> {
        self.ffxiv_event_queue.clone()
    }

    fn get_tick_interval(&self) -> TimeType {
        self.tick_millisecond
    }

    fn get_player_id(&self) -> Option<IdType> {
        Some(self.player_id)
    }

    fn get_id(&self) -> TickerKey {
        TickerKey::new(self.id, self.player_id)
    }

    fn set_event_queue(&mut self, event_queue: Rc<RefCell<FfxivEventQueue>>) {
        self.ffxiv_event_queue = event_queue;
    }

    fn has_initial_tick(&self) -> bool {
        self.initial_tick
    }
    fn get_remaining_time(&self) -> TimeType {
        self.time_left_millisecond
    }
}

impl IndependentTicker {
    pub fn new(
        id: IdType,
        _: TimeType,
        time_left_millisecond: TimeType,
        tick_event: Vec<FfxivEvent>,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_id: IdType,
        proc_percent: PercentType,
        initial_tick: bool,
    ) -> Self {
        Self {
            id,
            player_id,
            tick_event,
            ffxiv_event_queue,
            proc_percent,
            time_left_millisecond,
            tick_millisecond: GLOBAL_TICK_INTERVAL_MILLISECOND,
            initial_tick,
        }
    }

    pub fn generate_event(&self, current_time_millisecond: TimeType) -> Vec<FfxivEvent> {
        let mut events = vec![];

        for tick_event in &self.tick_event {
            events.push(
                tick_event
                    .clone()
                    .add_time_to_event(current_time_millisecond),
            );
        }

        events
    }
}
