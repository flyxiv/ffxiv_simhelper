use crate::event_ticker::EventTicker;
use ffxiv_simbot_combat_components::event::ffxiv_event::FfxivEvent;
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::skill::attack_skill::AttackSkill;
use ffxiv_simbot_combat_components::skill::GCD_DEFAULT_DELAY_MILLISECOND;
use ffxiv_simbot_combat_components::status::debuff_status::DebuffStatus;
use ffxiv_simbot_combat_components::{IdType, StatusTable, TimeType};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

static DEFAULT_AUTO_ATTACK_COOLDOWN_MILLISECOND: TimeType = 2500;
static AUTO_ATTACK_ID: IdType = 10000;

/// Loads Auto Attack Event for Melee Jobs
pub struct AutoAttackTicker {
    id: IdType,
    player_id: IdType,
    event_queue: Rc<RefCell<FfxivEventQueue>>,
    auto_attack: AttackSkill,
    auto_attack_interval: TimeType,
}

impl EventTicker for AutoAttackTicker {
    fn generate_tick_event(
        &mut self,
        current_time_millisecond: TimeType,
        player: Option<Rc<RefCell<FfxivPlayer>>>,
        debuff: StatusTable<DebuffStatus>,
    ) {
        {
            if let Some(player) = player {
                let speed = player.borrow().get_gcd_delay_millisecond(&self.auto_attack);
                self.auto_attack_interval = speed;

                self.event_queue
                    .borrow_mut()
                    .push(Reverse(FfxivEvent::Damage(
                        self.player_id,
                        AUTO_ATTACK_ID,
                        self.auto_attack.get_potency(),
                        false,
                        false,
                        player.borrow().buff_list.borrow().clone(),
                        debuff.borrow().clone(),
                        current_time_millisecond,
                    )));
            }
        }
    }

    fn update_remaining_time(&mut self, _: TimeType) {}

    fn get_event_queue(&self) -> Rc<RefCell<FfxivEventQueue>> {
        self.event_queue.clone()
    }

    fn get_tick_interval(&self) -> TimeType {
        self.auto_attack_interval
    }

    fn get_player_id(&self) -> Option<IdType> {
        Some(self.player_id)
    }
}

impl IdEntity for AutoAttackTicker {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl AutoAttackTicker {
    pub(crate) fn new(
        id: IdType,
        player_id: IdType,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> Self {
        let mut auto_attack = AttackSkill::new(100000, String::from("Auto Attack"), player_id, 100);
        auto_attack.player_id = player_id;

        Self {
            id,
            player_id,
            event_queue: ffxiv_event_queue,
            auto_attack,
            auto_attack_interval: GCD_DEFAULT_DELAY_MILLISECOND,
        }
    }
}
