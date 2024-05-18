use crate::event_ticker::{EventTicker, PercentType, GLOBAL_TICK_INTERVAL_MILLISECOND};
use ffxiv_simbot_combat_components::event::ffxiv_event::FfxivEvent;
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::status::debuff_status::DebuffStatus;
use ffxiv_simbot_combat_components::{IdType, StatusTable, TimeType};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

/// Generator of ticks that run independent of any other events
/// ex) Bard's Song Ticks

pub(crate) struct IndependentTicker {
    id: IdType,
    player_id: IdType,
    tick_millisecond: TimeType,
    time_left_millisecond: TimeType,
    tick_event: FfxivEvent,
    ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    proc_percent: PercentType,
}

impl EventTicker for IndependentTicker {
    fn generate_tick_event(
        &mut self,
        current_time_millisecond: TimeType,
        _: Option<Rc<RefCell<FfxivPlayer>>>,
        _: StatusTable<DebuffStatus>,
    ) {
        if self.tick_millisecond <= self.time_left_millisecond {
            let event = self.generate_event(current_time_millisecond);
            self.ffxiv_event_queue.borrow_mut().push(Reverse(event));
        }
    }

    fn update_remaining_time(&mut self, elapsed_time: TimeType) {
        self.time_left_millisecond -= elapsed_time;
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
}

impl IdEntity for IndependentTicker {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl IndependentTicker {
    pub(crate) fn new(
        id: IdType,
        current_time_millisecond: TimeType,
        time_left_millisecond: TimeType,
        tick_event: FfxivEvent,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_id: IdType,
        proc_percent: PercentType,
    ) -> Self {
        let ticker = Self {
            id,
            player_id,
            tick_event,
            ffxiv_event_queue,
            proc_percent,
            time_left_millisecond,
            tick_millisecond: GLOBAL_TICK_INTERVAL_MILLISECOND,
        };

        ticker.add_next_event_to_queue(current_time_millisecond);
        ticker
    }

    pub(crate) fn generate_event(&self, current_time_millisecond: TimeType) -> FfxivEvent {
        match self.tick_event {
            FfxivEvent::IncreasePlayerResource(player_id, stack_id, amount, _) => {
                FfxivEvent::IncreasePlayerResource(
                    player_id,
                    stack_id,
                    amount,
                    current_time_millisecond,
                )
            }
            _ => FfxivEvent::IncreasePlayerResource(0, 0, 0, current_time_millisecond),
        }
    }
}
