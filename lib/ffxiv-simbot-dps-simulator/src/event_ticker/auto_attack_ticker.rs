use crate::event_ticker::EventTicker;
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::{IdType, TimeType};

static DEFAULT_AUTO_ATTACK_COOLDOWN_MILLISECOND: TimeType = 2500;

/// Loads Auto Attack Event for Melee Jobs
pub(crate) struct AutoAttackTicker {
    id: IdType,
    player_id: IdType,
    most_recent_tick_time_millisecond: TimeType,
}

impl EventTicker for AutoAttackTicker {
    fn add_next_tick_event_to_queue(&mut self, player_gcd: Option<TimeType>) -> TimeType {
        let mut next_tick_time_millisecond = self.most_recent_tick_time_millisecond;
        next_tick_time_millisecond +=
            player_gcd.unwrap_or(DEFAULT_AUTO_ATTACK_COOLDOWN_MILLISECOND);

        self.update_tick(next_tick_time_millisecond);
        next_tick_time_millisecond
    }

    fn get_related_player_id(&self) -> Option<IdType> {
        Some(self.player_id)
    }
}

impl IdEntity for AutoAttackTicker {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl AutoAttackTicker {
    pub(crate) fn new(id: IdType, player_id: IdType) -> Self {
        Self {
            id,
            player_id,
            most_recent_tick_time_millisecond: 0,
        }
    }
}
