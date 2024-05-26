use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::combat_resources::CombatResource;
use crate::damage_calculator::damage_rdps_profile::{FfxivRaidDamageTable, RaidDamageTable};
use crate::damage_calculator::DamageRdpsProfile;
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
use crate::rotation::ffxiv_priority_table::FfxivPriorityTable;
use crate::rotation::priority_table::{PriorityTable, SkillUsageInfo};
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{Skill, AUTO_ATTACK_ID};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::status::status_info::StatusInfo;
use crate::status::status_timer::StatusTimer;
use crate::{DamageType, IdType, StatusTable, TimeType};
use ffxiv_simbot_db::job::Job;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::MultiplierType;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::rc::Rc;

/// The Abstraction for an actual FFXIV Player in the combat.
pub struct FfxivPlayer {
    /// Stat/Job Data about the player
    pub id: IdType,
    pub job: Job,
    pub power: CharacterPower,

    pub priority_table: FfxivPriorityTable,

    /// Realtime Combat Data about the player
    pub buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
    pub(crate) combat_resources: RefCell<FfxivCombatResources>,
    pub(crate) internal_event_queue: RefCell<Vec<FfxivPlayerInternalEvent>>,
    pub event_queue: Rc<RefCell<FfxivEventQueue>>,
    pub turn_calculator: RefCell<PlayerTurnCalculator>,
    pub mana_available: Option<i32>,

    /// profile tables. Saved and returned later on the response the show combat simulation results.
    /// Saves how much % of the total damage each damage skill contributed to the total damage.
    pub skill_damage_profile_table: FfxivRaidDamageTable,
    pub skill_raw_damage_table: HashMap<IdType, DamageType>,
    pub start_turn: FfxivEvent,
}

impl Clone for FfxivPlayer {
    fn clone(&self) -> Self {
        FfxivPlayer {
            id: self.id,
            job: self.job.clone(),
            power: self.power.clone(),
            priority_table: self.priority_table.clone(),
            buff_list: Rc::new(RefCell::new(self.buff_list.borrow().clone())),
            combat_resources: RefCell::new(self.combat_resources.borrow().clone()),
            internal_event_queue: RefCell::new(self.internal_event_queue.borrow().clone()),
            event_queue: self.event_queue.clone(),
            turn_calculator: self.turn_calculator.clone(),
            mana_available: self.mana_available,
            skill_damage_profile_table: Default::default(),
            skill_raw_damage_table: Default::default(),
            start_turn: self.start_turn.clone(),
        }
    }
}

impl Player for FfxivPlayer {
    fn consume_internal_events(&self, debuffs: StatusTable<DebuffStatus>) {
        for internal_event in self.internal_event_queue.borrow().iter() {
            self.handle_internal_event(internal_event, debuffs.clone());
        }
        self.internal_event_queue.borrow_mut().clear();
    }

    fn handle_ffxiv_event(&mut self, mut event: FfxivEvent, debuffs: StatusTable<DebuffStatus>) {
        let debuffs_table = debuffs.clone();
        match event {
            FfxivEvent::PlayerTurn(_, turn_type, max_time, event_time) => {
                let turn_info = TurnInfo {
                    turn_type,
                    next_gcd_millisecond: max_time,
                    lower_bound_millisecond: event_time,
                };
                self.use_turn(turn_info, debuffs, event_time)
            }
            FfxivEvent::ApplyBuff(player_id, _, buff, duration, max_duration, _) => {
                self.add_status(buff, duration, max_duration, player_id)
            }
            FfxivEvent::ApplyBuffStack(player_id, _, buff, duration, refresh, _) => {
                self.add_status_stack(buff, duration, refresh, player_id)
            }
            FfxivEvent::UseSkill(_, skill_id, event_time) => {
                self.use_skill(skill_id, debuffs, event_time)
            }
            FfxivEvent::ApplyRaidBuff(player_id, buff, duration, max_duration, _) => {
                self.add_status(buff, duration, max_duration, player_id)
            }
            FfxivEvent::RemoveTargetBuff(player_id, _, buff_id, _) => {
                let key = StatusKey::new(buff_id, player_id);
                self.buff_list.borrow_mut().remove(&key);
            }
            FfxivEvent::IncreasePlayerResource(_, stack_id, increase_amount, _) => self
                .combat_resources
                .borrow_mut()
                .add_resource(stack_id, increase_amount),
            _ => {}
        }

        self.consume_internal_events(debuffs_table);
    }
}

impl FfxivPlayer {
    fn use_turn(
        &self,
        turn_info: TurnInfo,
        debuffs: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        combat_time_millisecond: TimeType,
    ) {
        let next_skill_ids = self.get_next_skill(&turn_info, debuffs.clone());
        self.insert_turn_update_internal_event(&next_skill_ids, turn_info);

        let next_skill_ids = next_skill_ids;

        for use_skill in next_skill_ids.clone() {
            if let Some(use_later_time) = use_skill.use_later_time {
                self.event_queue
                    .borrow_mut()
                    .push(Reverse(FfxivEvent::UseSkill(
                        self.id,
                        use_skill.skill_id,
                        use_later_time,
                    )));
            } else {
                self.use_skill(use_skill.skill_id, debuffs.clone(), combat_time_millisecond);
            }
        }
    }

    fn use_skill(
        &self,
        skill_id: IdType,
        debuffs: StatusTable<DebuffStatus>,
        combat_time_millisecond: TimeType,
    ) {
        let (ffxiv_events, internal_events) = self
            .combat_resources
            .borrow()
            .get_skill(skill_id)
            .generate_skill_events(
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
            self.event_queue.borrow_mut().push(Reverse(ffxiv_event));
        }
    }

    fn insert_turn_update_internal_event(
        &self,
        next_skill_ids: &Vec<SkillUsageInfo>,
        turn_info: TurnInfo,
    ) {
        let update_turn_event = match turn_info.turn_type {
            FfxivTurnType::Gcd => {
                let gcd_skill_id = next_skill_ids[0].skill_id;

                let time_info = self
                    .combat_resources
                    .borrow()
                    .get_skill(gcd_skill_id)
                    .get_time_related_informations(&self);

                FfxivPlayerInternalEvent::UpdateTurn(
                    FfxivTurnType::Gcd,
                    turn_info.lower_bound_millisecond,
                    time_info.charge_time_millisecond,
                    time_info.cast_time_millisecond,
                    time_info.gcd_cooldown_millisecond,
                    time_info.delay_millisecond,
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

    pub(crate) fn handle_internal_event(
        &self,
        internal_event: &FfxivPlayerInternalEvent,
        debuffs: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
    ) {
        match internal_event {
            FfxivPlayerInternalEvent::UpdateTurn(_, _, _, _, _, _) => {
                self.turn_calculator
                    .borrow_mut()
                    .update_internal_status(&internal_event);
                self.turn_calculator.borrow().produce_event_to_queue();
            }
            FfxivPlayerInternalEvent::UpdateCombo(combo) => {
                self.combat_resources.borrow_mut().update_combo(&combo)
            }
            FfxivPlayerInternalEvent::StartCooldown(skill_id) => {
                self.combat_resources.borrow_mut().start_cooldown(*skill_id)
            }
            FfxivPlayerInternalEvent::IncreaseResource(resource_id, resource_amount) => self
                .combat_resources
                .borrow_mut()
                .add_resource(*resource_id, *resource_amount),
            FfxivPlayerInternalEvent::RemoveBuff(buff_id) => {
                let key = StatusKey::new(*buff_id, self.get_id());
                let mut buff_list = self.buff_list.borrow_mut();
                let mut delete = true;

                if let Some(buff) = buff_list.get_mut(&key) {
                    buff.stacks -= 1;
                    delete = buff.stacks == 0;
                }

                if delete {
                    buff_list.remove(&key);
                }
            }
            FfxivPlayerInternalEvent::RemoveDebuff(debuff_id) => {
                let key = StatusKey::new(*debuff_id, self.get_id());
                let mut debuff_list = debuffs.borrow_mut();
                let mut delete = true;

                if let Some(debuff) = debuff_list.get_mut(&key) {
                    debuff.stacks -= 1;
                    delete = debuff.stacks == 0;
                }

                if delete {
                    debuff_list.remove(&key);
                }
            }
            FfxivPlayerInternalEvent::UseResource(resource_id, resource_amount) => self
                .combat_resources
                .borrow_mut()
                .use_resource(*resource_id, *resource_amount),
            _ => {}
        }
    }

    fn get_next_skill(
        &self,
        turn_info: &TurnInfo,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
    ) -> Vec<SkillUsageInfo> {
        self.priority_table.get_next_skill(
            debuff_list,
            turn_info,
            self,
            &self.combat_resources.borrow(),
        )
    }

    #[inline]
    pub(crate) fn get_damage_inflict_time_millisecond(&self, skill: &AttackSkill) -> TimeType {
        skill.charging_time_millisecond + self.get_cast_time(skill)
    }

    fn has_resources_for_skill<S: Skill>(&self, _: S) -> bool {
        // TODO: Implement mana resource check for casters.
        return false;
    }

    pub fn get_gcd_delay_millisecond(&self, skill: &AttackSkill) -> TimeType {
        let gcd_cooldown_millisecond = skill.gcd_cooldown_millisecond;

        if skill.is_speed_buffed() {
            self.get_speed_buffed_time(gcd_cooldown_millisecond)
        } else {
            gcd_cooldown_millisecond
        }
    }

    pub fn print_skill_debug(&self, skill_id: IdType) -> String {
        if skill_id == AUTO_ATTACK_ID {
            format!(
                "skill id: {}, name: Auto Attack: player {}",
                skill_id, self.id
            )
        } else {
            format!(
                "skill id: {}, name: {}",
                skill_id,
                self.combat_resources.borrow().get_skill(skill_id).name
            )
        }
    }
}

impl IdEntity for FfxivPlayer {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl CooldownTimer for FfxivPlayer {
    fn update_cooldown(&mut self, time_passed: TimeType) {
        self.combat_resources
            .borrow_mut()
            .update_cooldown(time_passed);
    }
}

impl FfxivPlayer {
    pub(crate) fn get_gcd(&self, skill: &AttackSkill) -> TimeType {
        let mut gcd_cooldown_millisecond = skill.gcd_cooldown_millisecond;

        let charging_time = skill.charging_time_millisecond;

        if skill.is_speed_buffed() {
            gcd_cooldown_millisecond = self.get_speed_buffed_time(gcd_cooldown_millisecond)
        }

        charging_time + gcd_cooldown_millisecond
    }

    pub(crate) fn get_cast_time(&self, skill: &AttackSkill) -> TimeType {
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
            self.power.speed_multiplier,
            speed_buff_multiplier,
        )
    }

    fn get_gcd_buff_multiplier(&self) -> MultiplierType {
        let mut gcd_buffs_multiplier = 1.0;
        for buff in self.buff_list.borrow().values() {
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
        priority_table: FfxivPriorityTable,
        buff_list: HashMap<StatusKey, BuffStatus>,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
        start_turn: FfxivEvent,
    ) -> FfxivPlayer {
        FfxivPlayer {
            id,
            combat_resources: RefCell::new(FfxivCombatResources::new(&job, id)),
            job,
            power,
            priority_table,
            buff_list: Rc::new(RefCell::new(buff_list)),
            internal_event_queue: RefCell::new(vec![]),
            turn_calculator: RefCell::new(PlayerTurnCalculator::new(id, event_queue.clone())),
            event_queue,
            mana_available: None,
            skill_damage_profile_table: Default::default(),
            skill_raw_damage_table: Default::default(),
            start_turn,
        }
    }
    pub fn update_skill_damage_table(
        &mut self,
        skill_id: IdType,
        damage_profile: &DamageRdpsProfile,
    ) {
        self.skill_damage_profile_table
            .update_table(&damage_profile.status_rdps_contribution);
        self.skill_raw_damage_table
            .insert(skill_id, damage_profile.raw_damage);
    }

    pub fn is_melee(&self) -> bool {
        match self.job.abbrev.as_str() {
            "NIN" | "MNK" | "DRG" | "SAM" | "GNB" | "DRK" | "PLD" | "WAR" | "BRD" | "MCH"
            | "DNC" => true,
            _ => false,
        }
    }
}

impl GcdCalculator for FfxivPlayer {}

impl StatusHolder<BuffStatus> for FfxivPlayer {
    fn get_status_table(&self) -> Rc<RefCell<HashMap<StatusKey, BuffStatus>>> {
        self.buff_list.clone()
    }
}

impl StatusTimer<BuffStatus> for FfxivPlayer {}
