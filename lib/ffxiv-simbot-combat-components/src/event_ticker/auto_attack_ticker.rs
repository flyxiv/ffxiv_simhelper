use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::event_ticker::{EventTicker, PercentType, TickerKey};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::{AUTO_ATTACK_ID, GCD_DEFAULT_DELAY_MILLISECOND};
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, StatusTable, TimeType};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

/// Loads Auto Attack Event for Melee Jobs
#[derive(Clone)]
pub struct AutoAttackTicker {
    id: IdType,
    player_id: IdType,
    event_queue: Rc<RefCell<FfxivEventQueue>>,
    auto_attack_id: IdType,
    auto_attack: AttackSkill,
    auto_attack_interval: TimeType,
    trait_percent: PercentType,
    duration_millisecond: TimeType,
    damage_category: DamageCategory,
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
                        self.auto_attack_id,
                        self.auto_attack.get_potency(),
                        self.trait_percent,
                        false,
                        false,
                        player.borrow().buff_list.borrow().clone(),
                        debuff.borrow().clone(),
                        self.damage_category,
                        current_time_millisecond,
                    )));
            }
        }
    }

    fn update_remaining_time(&mut self, elapsed_time_millisecond: TimeType) {
        self.duration_millisecond -= elapsed_time_millisecond;
    }

    fn get_event_queue(&self) -> Rc<RefCell<FfxivEventQueue>> {
        self.event_queue.clone()
    }

    fn get_tick_interval(&self) -> TimeType {
        self.auto_attack_interval
    }

    fn get_player_id(&self) -> Option<IdType> {
        Some(self.player_id)
    }

    fn get_id(&self) -> TickerKey {
        TickerKey::new(self.id, self.player_id)
    }

    fn set_event_queue(&mut self, event_queue: Rc<RefCell<FfxivEventQueue>>) {
        self.event_queue = event_queue
    }

    fn has_initial_tick(&self) -> bool {
        false
    }
    fn get_remaining_time(&self) -> TimeType {
        self.duration_millisecond
    }
}

impl AutoAttackTicker {
    pub fn new(
        id: IdType,
        player_id: IdType,
        job_abbrev: &String,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> Self {
        let potency = match job_abbrev.as_str() {
            "PLD" | "WAR" | "GNB" | "DRK" | "MNK" | "DRG" | "NIN" | "SAM" | "RPR" | "DNC" => 110,
            _ => 100,
        };

        let mut auto_attack = AttackSkill::new(
            AUTO_ATTACK_ID,
            String::from("Auto Attack"),
            player_id,
            potency,
        );
        auto_attack.player_id = player_id;

        Self {
            id,
            player_id,
            event_queue: ffxiv_event_queue,
            auto_attack_id: AUTO_ATTACK_ID,
            auto_attack,
            auto_attack_interval: GCD_DEFAULT_DELAY_MILLISECOND,
            trait_percent: 100,
            duration_millisecond: TimeType::MAX,
            damage_category: DamageCategory::AutoAttack,
        }
    }

    pub fn new_with_auto_attack(
        id: IdType,
        player_id: IdType,
        auto_attack: AttackSkill,
        damage_category: DamageCategory,
        duration_millisecond: TimeType,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> Self {
        Self {
            id,
            player_id,
            event_queue: ffxiv_event_queue,
            auto_attack_id: auto_attack.id,
            trait_percent: auto_attack.trait_percent,
            auto_attack,
            auto_attack_interval: GCD_DEFAULT_DELAY_MILLISECOND,
            duration_millisecond,
            damage_category,
        }
    }
}
