use crate::event::FfxivEventQueue;
use crate::event_ticker::auto_attack_ticker::AutoAttackTicker;
use crate::event_ticker::global_ticker::GlobalTicker;
use crate::event_ticker::independent_ticker::IndependentTicker;
use crate::event_ticker::{EventTicker, TickerKey};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::status::debuff_status::DebuffStatus;
use crate::types::TimeType;
use crate::types::{PlayerIdType, StatusTable};
use std::cell::RefCell;
use std::rc::Rc;

/// Generator of various kinds of tick events that happen in FFXIV Combat.
#[derive(Clone)]
pub enum FfxivEventTicker {
    /// The "Server Tick" for DOT damages. Activates all DOT ticks every 3 seconds.
    GlobalTicker(GlobalTicker),

    /// Physical jobs deal an auto attack in the same interval as their
    /// GCD. However, the auto attack's delay is decided by the player's GCD delay at the time the
    /// auto attack is used(it could change if GCD buff/debuff expires), so the ticker has to get
    /// real time GCD delay from the player every time the next auto attack is activated.
    /// SMN's pet auto attacks are also included in this ticker.
    AutoAttackTicker(AutoAttackTicker),

    /// Tickers that run independently of any other events. Stack resource
    /// ticks are usually this kind ex) bard's song ticks.
    IndependentTicker(IndependentTicker),
}

impl EventTicker for FfxivEventTicker {
    fn generate_tick_event(
        &mut self,
        current_time_millisecond: TimeType,
        player: Option<Rc<RefCell<FfxivPlayer>>>,
        debuff: StatusTable<DebuffStatus>,
    ) {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => {
                ticker.generate_tick_event(current_time_millisecond, player, debuff);
            }
            FfxivEventTicker::GlobalTicker(ticker) => {
                ticker.generate_tick_event(current_time_millisecond, player, debuff);
            }
            FfxivEventTicker::IndependentTicker(ticker) => {
                ticker.generate_tick_event(current_time_millisecond, player, debuff);
            }
        }
    }

    fn update_remaining_time(&mut self, elapsed_time: TimeType) {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => {
                ticker.update_remaining_time(elapsed_time);
            }
            FfxivEventTicker::GlobalTicker(ticker) => {
                ticker.update_remaining_time(elapsed_time);
            }
            FfxivEventTicker::IndependentTicker(ticker) => {
                ticker.update_remaining_time(elapsed_time);
            }
        }
    }

    fn force_tick(&self, current_time_millisecond: TimeType) {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => {
                ticker.force_tick(current_time_millisecond);
            }
            FfxivEventTicker::GlobalTicker(ticker) => {
                ticker.force_tick(current_time_millisecond);
            }
            FfxivEventTicker::IndependentTicker(ticker) => {
                ticker.force_tick(current_time_millisecond);
            }
        }
    }

    fn get_event_queue(&self) -> Rc<RefCell<FfxivEventQueue>> {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => ticker.get_event_queue(),
            FfxivEventTicker::GlobalTicker(ticker) => ticker.get_event_queue(),
            FfxivEventTicker::IndependentTicker(ticker) => ticker.get_event_queue(),
        }
    }

    fn get_tick_interval(&self) -> TimeType {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => ticker.get_tick_interval(),
            FfxivEventTicker::GlobalTicker(ticker) => ticker.get_tick_interval(),
            FfxivEventTicker::IndependentTicker(ticker) => ticker.get_tick_interval(),
        }
    }

    fn get_player_id(&self) -> Option<PlayerIdType> {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => ticker.get_player_id(),
            FfxivEventTicker::GlobalTicker(ticker) => ticker.get_player_id(),
            FfxivEventTicker::IndependentTicker(ticker) => ticker.get_player_id(),
        }
    }

    fn get_id(&self) -> TickerKey {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => ticker.get_id(),
            FfxivEventTicker::GlobalTicker(ticker) => ticker.get_id(),
            FfxivEventTicker::IndependentTicker(ticker) => ticker.get_id(),
        }
    }

    fn set_event_queue(&mut self, event_queue: Rc<RefCell<FfxivEventQueue>>) {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => ticker.set_event_queue(event_queue),
            FfxivEventTicker::GlobalTicker(ticker) => ticker.set_event_queue(event_queue),
            FfxivEventTicker::IndependentTicker(ticker) => ticker.set_event_queue(event_queue),
        }
    }

    fn has_initial_tick(&self) -> bool {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => ticker.has_initial_tick(),
            FfxivEventTicker::GlobalTicker(ticker) => ticker.has_initial_tick(),
            FfxivEventTicker::IndependentTicker(ticker) => ticker.has_initial_tick(),
        }
    }

    fn get_remaining_time(&self) -> TimeType {
        match self {
            FfxivEventTicker::AutoAttackTicker(ticker) => ticker.get_remaining_time(),
            FfxivEventTicker::GlobalTicker(ticker) => ticker.get_remaining_time(),
            FfxivEventTicker::IndependentTicker(ticker) => ticker.get_remaining_time(),
        }
    }
}
