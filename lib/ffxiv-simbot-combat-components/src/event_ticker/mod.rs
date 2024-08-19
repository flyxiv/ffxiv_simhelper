use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{IdType, TimeType};
use crate::types::{PlayerIdType, StatusTable};
use std::cell::RefCell;
use std::cmp::{max, Reverse};
use std::rc::Rc;

pub mod auto_attack_ticker;
pub mod ffxiv_event_ticker;
pub mod global_ticker;
pub mod independent_ticker;

pub static GLOBAL_TICK_INTERVAL_MILLISECOND: TimeType = 3000;
pub type PercentType = u16;

/// Tickers that generate events based on combat's time rather than
/// the player's turn.
///
/// Kinds of tickers:
/// 1) Global ticker: Activates all DOT ticks every 3 seconds. Heal DOT tick/Regeneration DOT ticks
///    are all different, but damage-wise only the damage DOT tick is needed.
/// 2) Auto attack ticker: Physical jobs deal an auto attack in the same interval as their
///    GCD. However, the auto attack's delay is decided by the player's GCD delay at the time the
///    auto attack is used(it could change if GCD buff/debuff expires), so the ticker has to get
///    real time GCD delay from the player every time the auto attack is activated.
/// 3) Independent ticker: Tickers that run independently of any other events. Stack resource
///    ticks are usually this kind ex) bard's song ticks.
pub trait EventTicker: Sized + Clone {
    fn run_ticker(
        &mut self,
        current_time_millisecond: TimeType,
        player: Option<Rc<RefCell<FfxivPlayer>>>,
        debuff: StatusTable<DebuffStatus>,
    ) {
        self.generate_tick_event(current_time_millisecond, player, debuff);
        self.add_next_event_to_queue(current_time_millisecond);
    }
    fn generate_tick_event(
        &mut self,
        current_time_millisecond: TimeType,
        player: Option<Rc<RefCell<FfxivPlayer>>>,
        debuff: StatusTable<DebuffStatus>,
    );
    fn update_remaining_time(&mut self, elapsed_time: TimeType);

    fn force_tick(&self, current_time_millisecond: TimeType) {
        self.get_event_queue()
            .borrow_mut()
            .push(Reverse(FfxivEvent::Tick(
                self.get_id(),
                current_time_millisecond,
            )));
    }

    fn add_next_event_to_queue(&self, current_time_millisecond: TimeType) {
        self.get_event_queue()
            .borrow_mut()
            .push(Reverse(FfxivEvent::Tick(
                self.get_id(),
                max(current_time_millisecond + self.get_tick_interval(), 0),
            )));
    }

    fn get_event_queue(&self) -> Rc<RefCell<FfxivEventQueue>>;
    fn get_tick_interval(&self) -> TimeType;
    fn get_player_id(&self) -> Option<PlayerIdType>;
    fn get_id(&self) -> TickerKey;
    fn set_event_queue(&mut self, event_queue: Rc<RefCell<FfxivEventQueue>>);
    fn has_initial_tick(&self) -> bool;
    fn get_remaining_time(&self) -> TimeType;
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub struct TickerKey {
    pub ticker_id: IdType,
    pub player_id: PlayerIdType,
}

impl TickerKey {
    pub fn new(ticker_id: IdType, player_id: PlayerIdType) -> Self {
        Self {
            player_id,
            ticker_id,
        }
    }
}
