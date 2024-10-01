use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::event_ticker::{EventTicker, PercentType, TickerKey};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::{AUTO_ATTACK_ID, GCD_DEFAULT_DELAY_MILLISECOND};
use crate::status::debuff_status::DebuffStatus;
use crate::status::snapshot_status::snapshot_status_infos;
use crate::types::{MultiplierType, SkillIdType, TimeType};
use crate::types::{PlayerIdType, StatusTable};
use std::cell::RefCell;
use std::cmp::{max, Reverse};
use std::rc::Rc;

/// Loads Auto Attack Event for Melee Jobs
#[derive(Clone)]
pub struct AutoAttackTicker {
    id: SkillIdType,
    player_id: PlayerIdType,
    event_queue: Rc<RefCell<FfxivEventQueue>>,
    auto_attack_id: SkillIdType,
    auto_attack: AttackSkill,
    auto_attack_interval: TimeType,
    trait_percent: PercentType,
    duration_millisecond: TimeType,
    damage_category: DamageCategory,
    job_tuning_value: MultiplierType,
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
                let speed = (player.borrow().get_gcd_delay_millisecond(&self.auto_attack)
                    as MultiplierType
                    / self.job_tuning_value) as TimeType;
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
                        snapshot_status_infos(
                            &player.borrow().buff_list.borrow(),
                            &debuff.borrow(),
                            self.player_id,
                        ),
                        self.damage_category,
                        max(current_time_millisecond, 0),
                    )));
            }
        }
    }

    fn force_tick(&self, _: TimeType) {}

    fn update_remaining_time(&mut self, elapsed_time_millisecond: TimeType) {
        self.duration_millisecond -= elapsed_time_millisecond;
    }

    fn get_event_queue(&self) -> Rc<RefCell<FfxivEventQueue>> {
        self.event_queue.clone()
    }

    fn get_tick_interval(&self) -> TimeType {
        self.auto_attack_interval
    }

    fn get_player_id(&self) -> Option<PlayerIdType> {
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
        id: SkillIdType,
        player_id: PlayerIdType,
        job_abbrev: &String,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> Self {
        let potency = match job_abbrev.as_str() {
            "NIN" => 110,
            "DRK" | "WAR" | "GNB" => 90,
            "BRD" | "MCH" => 80,
            "VPR" => 80,
            "RPR" | "MNK" => 90,
            "DRG" => 80,
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
            duration_millisecond: 19000000,
            damage_category: DamageCategory::AutoAttack,
            job_tuning_value: get_auto_attack_interval_tuning_value_of_job(job_abbrev),
        }
    }

    pub fn new_with_auto_attack(
        id: SkillIdType,
        player_id: PlayerIdType,
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
            job_tuning_value: 0.7,
        }
    }
}

/// while ten-chi-jin or dancing jobs can't auto attack but we can't implement that.
/// so we adjust the auto attack interval based on actual logs so that auto attack swing
/// matches the actual logs.
fn get_auto_attack_interval_tuning_value_of_job(job_abbrev: &String) -> MultiplierType {
    match job_abbrev.as_str() {
        "NIN" => 0.86,
        "DNC" => 0.75,
        "SAM" => 0.88,
        "BRD" => 0.82,
        "RPR" => 0.76,
        "DRK" => 0.90,
        "WAR" => 0.75,
        "GNB" => 0.9,
        "DRG" => 0.9,
        "MNK" => 0.95,
        _ => 1.0,
    }
}
