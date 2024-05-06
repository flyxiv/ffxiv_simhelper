use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::{IdType, TimeType};

mod auto_attack_ticker;
pub(crate) mod global_ticker;
mod independent_ticker;

pub(crate) static TICK_MILLISECOND: TimeType = 3000;

pub(crate) trait EventTicker: IdEntity {
    fn add_next_tick_event_to_queue(&self, player_gcd: Option<TimeType>) -> TimeType;
    fn get_related_player_id(&self) -> Option<IdType>;
}
