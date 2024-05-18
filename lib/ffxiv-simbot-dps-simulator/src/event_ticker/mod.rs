use ffxiv_simbot_combat_components::event::ffxiv_event::FfxivEvent;
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::status::debuff_status::DebuffStatus;
use ffxiv_simbot_combat_components::{IdType, StatusTable, TimeType};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

pub(crate) mod auto_attack_ticker;
pub(crate) mod global_ticker;
pub(crate) mod independent_ticker;

pub(crate) static GLOBAL_TICK_INTERVAL_MILLISECOND: TimeType = 3000;
pub(crate) type PercentType = i32;

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
pub trait EventTicker: IdEntity {
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
    fn add_next_event_to_queue(&self, current_time_millisecond: TimeType) {
        self.get_event_queue()
            .borrow_mut()
            .push(Reverse(FfxivEvent::Tick(
                self.get_id(),
                current_time_millisecond + self.get_tick_interval(),
            )));
    }

    fn get_event_queue(&self) -> Rc<RefCell<FfxivEventQueue>>;
    fn get_tick_interval(&self) -> TimeType;
    fn get_player_id(&self) -> Option<IdType>;
}
