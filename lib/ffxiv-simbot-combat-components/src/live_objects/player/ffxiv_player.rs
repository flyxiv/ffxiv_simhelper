use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::event::turn_info::TurnInfo;
use crate::event::FfxivEventQueue;
use crate::id_entity::IdEntity;
use crate::live_objects::player::gcd_calculator::GcdCalculator;
use crate::live_objects::player::player_turn_calculator::PlayerTurnCalculator;
use crate::live_objects::player::{Player, StatusKey};
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::job_priorities::ninja::NinjaPriorityTable;
use crate::rotation::priority_table::{PriorityTable, SkillUsageInfo};
use crate::skill::attack_skill::AttackSkill;
use crate::skill::job_abilities::ninja_abilities::get_huton_status;
use crate::skill::Skill;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::status::status_info::StatusInfo;
use crate::status::status_timer::StatusTimer;
use crate::{IdType, TimeType};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::job::Job;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::MultiplierType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// The Abstraction for an actual FFXIV Player in the combat.
pub struct FfxivPlayer {
    /// Stat/Job Data about the player
    pub id: IdType,
    pub job: Job,
    pub power: CharacterPower,

    pub priority_table: Box<dyn PriorityTable>,

    /// Realtime Combat Data about the player
    pub buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
    pub(crate) combat_resources: RefCell<Box<dyn CombatResource>>,
    pub(crate) internal_event_queue: RefCell<Vec<FfxivPlayerInternalEvent>>,
    pub event_queue: Rc<RefCell<FfxivEventQueue>>,
    pub turn_calculator: RefCell<PlayerTurnCalculator>,
    pub mana_available: Option<i32>,
}

impl Clone for FfxivPlayer {
    fn clone(&self) -> Self {
        FfxivPlayer {
            id: self.id,
            job: self.job.clone(),
            power: self.power.clone(),
            priority_table: Box::new(self.priority_table.clone()),
            buff_list: Rc::new(RefCell::new(self.buff_list.borrow().clone())),
            combat_resources: RefCell::new(Box::new(self.combat_resources.clone())),
            internal_event_queue: RefCell::new(self.internal_event_queue.borrow().clone()),
            event_queue: self.event_queue.clone(),
            turn_calculator: self.turn_calculator.clone(),
            mana_available: self.mana_available,
        }
    }
}

impl Player for FfxivPlayer {
    fn use_turn(
        &self,
        turn_info: TurnInfo,
        debuffs: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        combat_time_millisecond: TimeType,
    ) {
        let next_skill_ids = self.get_next_skill(&turn_info, debuffs.clone());
        self.insert_turn_update_internal_event(&next_skill_ids, turn_info);

        if next_skill_ids.is_none() {
            return;
        }

        let next_skill_ids = next_skill_ids.unwrap();

        for use_skill in next_skill_ids.clone() {
            if let Some(use_later_time) = use_skill.use_later_time {
                self.event_queue.borrow_mut().push(FfxivEvent::UseSkill(
                    self.id,
                    use_skill.skill_id,
                    use_later_time,
                ));
            } else {
                self.use_skill(use_skill.skill_id, debuffs.clone(), combat_time_millisecond)
            }
        }
    }

    fn use_skill(
        &self,
        skill_id: IdType,
        debuffs: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        combat_time_millisecond: TimeType,
    ) {
        let skill = self.combat_resources.borrow().get_skill(skill_id);
        let (ffxiv_events, internal_events) = skill.generate_skill_events(
            self.buff_list.clone(),
            debuffs.clone(),
            combat_time_millisecond,
            self,
        );

        self.internal_event_queue
            .borrow_mut()
            .extend(internal_events);

        for ffxiv_event in ffxiv_events {
            // extend doesn't insert and sort, so we need to push one by one.
            self.event_queue.borrow_mut().push(ffxiv_event);
        }

        self.consume_internal_events(debuffs);
    }

    fn consume_internal_events(&self, debuffs: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>) {
        for internal_event in self.internal_event_queue.borrow().iter() {
            self.handle_internal_event(internal_event, debuffs.clone());
        }
        self.internal_event_queue.borrow_mut().clear();
    }
}

impl FfxivPlayer {
    fn insert_turn_update_internal_event(
        &self,
        next_skill_ids: Vec<SkillUsageInfo>,
        turn_info: TurnInfo,
    ) {
        let update_turn_event = match turn_info.turn_type {
            FfxivTurnType::Gcd => {
                let gcd_skill_id = next_skill_ids.unwrap()[0].skill_id;
                let gcd_skill = self.combat_resources.borrow().get_skill(gcd_skill_id);

                FfxivPlayerInternalEvent::UpdateTurn(
                    FfxivTurnType::Gcd,
                    turn_info.lower_bound_millisecond,
                    gcd_skill.charging_time_millisecond,
                    self.get_cast_time(gcd_skill),
                    self.get_gcd(gcd_skill),
                    gcd_skill.get_delay_millisecond(),
                )
            }
            FfxivTurnType::Ogcd => {
                FfxivPlayerInternalEvent::UpdateTurn(FfxivTurnType::Ogcd, 0, 0, 0, 0, 0)
            }
        };

        self.internal_event_queue
            .borrow_mut()
            .push(update_turn_event);
    }

    fn handle_internal_event(
        &self,
        internal_event: &FfxivPlayerInternalEvent,
        debuffs: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
    ) {
        match internal_event {
            FfxivPlayerInternalEvent::UpdateTurn(_, _, _, _, _, _) => {
                self.turn_calculator
                    .borrow_mut()
                    .update_internal_status(internal_event);
                self.turn_calculator.borrow().produce_event_to_queue();
            }
            FfxivPlayerInternalEvent::UpdateCombo(combo) => {
                self.combat_resources.borrow_mut().update_combo(combo)
            }
            FfxivPlayerInternalEvent::StartCooldown(skill_id) => {
                self.combat_resources.borrow_mut().start_cooldown(*skill_id)
            }
            FfxivPlayerInternalEvent::IncreaseResource(resource_id, resource_amount) => self
                .combat_resources
                .borrow_mut()
                .increase_resource(*resource_id, *resource_amount),
            FfxivPlayerInternalEvent::RemoveBuff(buff_id) => {
                self.buff_list.borrow_mut().remove(buff_id);
            }
            FfxivPlayerInternalEvent::RemoveDebuff(debuff_id) => {
                debuffs.borrow_mut().remove(debuff_id);
            }
        }
    }

    fn get_next_skill(
        &self,
        turn_info: &TurnInfo,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
    ) -> Vec<SkillUsageInfo> {
        self.priority_table
            .get_next_skill(debuff_list, turn_info, self)
    }

    #[inline]
    pub(crate) fn get_damage_inflict_time_millisecond(&self, skill: &AttackSkill) -> TimeType {
        skill.get_charging_time_millisecond() + self.get_cast_time(skill);
    }

    fn has_resources_for_skill<S: Skill>(&self, _: S) -> bool {
        // TODO: Implement mana resource check for casters.
        return false;
    }

    fn get_gcd_delay_millisecond<S: Skill>(&self, skill: &S) -> TimeType {
        let gcd_cooldown_millisecond = skill.get_gcd_cooldown_millsecond();

        if skill.is_speed_buffed() {
            self.get_speed_buffed_time(gcd_cooldown_millisecond)
        } else {
            gcd_cooldown_millisecond
        }
    }

    pub(crate) fn get_millisecond_before_burst(&self) -> TimeType {
        let combat_time = self.turn_calculator.borrow().combat_time_millisecond;
        self.get_next_burst_time(combat_time)
    }
}

impl IdEntity for FfxivPlayer {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl CooldownTimer for FfxivPlayer {
    fn update_cooldown(&mut self, time_passed: TimeType) {
        self.combat_resources.borrow().update_cooldown(time_passed);
    }
}

impl FfxivPlayer {
    fn get_gcd<S: Skill>(&self, skill: &S) -> TimeType {
        let mut gcd_cooldown_millisecond = skill.get_gcd_cooldown_millsecond();

        let charging_time = skill.get_charging_time_millisecond();

        if skill.is_speed_buffed() {
            gcd_cooldown_millisecond = self.get_speed_buffed_time(gcd_cooldown_millisecond)
        }

        charging_time + gcd_cooldown_millisecond
    }

    fn get_cast_time<S>(&self, skill: &S) -> TimeType
    where
        S: Skill,
    {
        let cast_time = skill.get_gcd_cast_time();

        if skill.is_speed_buffed() {
            self.get_speed_buffed_time(cast_time)
        } else {
            cast_time
        }
    }

    fn get_speed_buffed_time(&self, time_millisecond: TimeType) -> TimeType {
        let speed_buff_multiplier = self.get_gcd_buff_multiplier();

        self.calculate_gcd_millisecond(
            time_millisecond,
            self.get_player_power().speed_multiplier,
            speed_buff_multiplier,
        )
    }

    fn get_gcd_buff_multiplier(&self) -> MultiplierType {
        let mut gcd_buffs_multiplier = 1.0;
        for buff in self.buff_list.borrow().iter() {
            match buff.status_info {
                StatusInfo::SpeedPercent(buff_increase_percent) => {
                    gcd_buffs_multiplier = gcd_buffs_multiplier
                        * (1.0 + (buff_increase_percent as MultiplierType / 100.0));
                }
                _ => {}
            }
        }
        gcd_buffs_multiplier
    }

    pub fn new(
        id: IdType,
        job: Job,
        power: CharacterPower,
        priority_table: Box<dyn PriorityTable>,
        buff_list: HashMap<StatusKey, BuffStatus>,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
        start_time_millisecond: TimeType,
    ) -> FfxivPlayer {
        // Add player's first turn to event queue
        event_queue.borrow_mut().push(FfxivEvent::PlayerTurn(
            id,
            FfxivTurnType::Gcd,
            start_time_millisecond,
            start_time_millisecond,
        ));

        FfxivPlayer {
            id,
            job,
            power,
            priority_table,
            buff_list: Rc::new(RefCell::new(buff_list)),
            combat_resources: RefCell::new(Box::new(())),
            internal_event_queue: RefCell::new(vec![]),
            event_queue,
            turn_calculator: RefCell::new(PlayerTurnCalculator::new(id, start_time_millisecond)),
            mana_available: None,
        }
    }
}

impl GcdCalculator for FfxivPlayer {}

impl StatusHolder<BuffStatus> for FfxivPlayer {
    fn get_status_list(&self) -> Rc<RefCell<Vec<BuffStatus>>> {
        self.buff_list.clone()
    }
}

impl StatusTimer<BuffStatus> for FfxivPlayer {}
