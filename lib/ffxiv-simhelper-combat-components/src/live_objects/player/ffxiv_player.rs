use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::combat_resources::CombatResource;
use crate::damage_calculator::DamageRdpsProfile;
use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::event::turn_info::TurnInfo;
use crate::event::FfxivEventQueue;
use crate::live_objects::player::gcd_calculator::GcdCalculator;
use crate::live_objects::player::logs::{DamageLog, SkillLog};
use crate::live_objects::player::player_power::PlayerPower;
use crate::live_objects::player::player_turn_calculator::PlayerTurnCalculator;
use crate::live_objects::player::{Player, StatusKey};
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::ffxiv_priority_table::FfxivPriorityTable;
use crate::rotation::priority_table::{PriorityTable, SkillUsageInfo};
use crate::skill::attack_skill::AttackSkill;
use crate::skill::use_type::UseType;
use crate::skill::{Skill, AUTO_ATTACK_ID};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::status::status_info::StatusInfo;
use crate::status::status_timer::StatusTimer;
use crate::types::{MultiplierType, StatusTable};
use crate::types::{PlayerIdType, SkillIdType, TimeType};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::rc::Rc;

pub const FFXIV_EVENT_QUEUE_INITIAL_CAPACITY: usize = 20;
pub static TARGET_ID: PlayerIdType = 100;

/// The Abstraction for an actual FFXIV Player in the combat.
pub struct FfxivPlayer {
    /// Stat/Job Data about the player
    pub id: PlayerIdType,
    pub job_abbrev: String,
    pub power: PlayerPower,

    pub priority_table: FfxivPriorityTable,

    /// Realtime Combat Data about the player
    pub buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
    pub(crate) combat_resources: RefCell<FfxivCombatResources>,
    pub(crate) internal_event_queue: RefCell<Vec<FfxivPlayerInternalEvent>>,
    pub event_queue: Rc<RefCell<FfxivEventQueue>>,
    pub turn_calculator: RefCell<PlayerTurnCalculator>,

    /// profile tables. Saved and returned later on the response the show combat simulation results.
    /// Saves how much % of the total damage each damage skill contributed to the total damage.
    pub damage_logs: Vec<DamageLog>,
    pub skill_logs: Vec<SkillLog>,
    pub start_turn: FfxivEvent,

    pub elapsed_time: TimeType,
}

impl Clone for FfxivPlayer {
    fn clone(&self) -> Self {
        FfxivPlayer {
            id: self.id,
            job_abbrev: self.job_abbrev.clone(),
            power: self.power.clone(),
            priority_table: self.priority_table.clone(),
            buff_list: Rc::new(RefCell::new(self.buff_list.borrow().clone())),
            combat_resources: RefCell::new(self.combat_resources.borrow().clone()),
            internal_event_queue: RefCell::new(self.internal_event_queue.borrow().clone()),
            event_queue: self.event_queue.clone(),
            turn_calculator: self.turn_calculator.clone(),
            damage_logs: Default::default(),
            skill_logs: Default::default(),
            start_turn: self.start_turn.clone(),
            elapsed_time: self.elapsed_time,
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

    fn handle_ffxiv_event(&mut self, event: FfxivEvent, debuffs: StatusTable<DebuffStatus>) {
        if self.elapsed_time > 0 {
            self.update_player_time_informations();
        }
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
            FfxivEvent::UseSkill(_, target_id, skill_id, event_time) => {
                self.use_skill(skill_id, target_id, debuffs, event_time)
            }
            FfxivEvent::ApplyRaidBuff(player_id, buff, duration, max_duration, _) => {
                self.add_status(buff, duration, max_duration, player_id)
            }
            FfxivEvent::RefreshBuff(player_id, _, buff, duration, max_duration, _) => {
                let key = StatusKey::new(buff.id, player_id);
                let buff_search = self.buff_list.borrow().contains_key(&key);

                if buff_search {
                    self.add_status(buff, duration, max_duration, player_id)
                }
            }
            FfxivEvent::RemoveTargetBuff(player_id, _, buff_id, _) => {
                let key = StatusKey::new(buff_id, player_id);
                self.buff_list.borrow_mut().remove(&key);
            }
            FfxivEvent::IncreasePlayerResource(_, stack_id, increase_amount, _) => self
                .combat_resources
                .borrow_mut()
                .add_resource(stack_id, increase_amount),
            FfxivEvent::ReduceSkillCooldown(_, skill_id, reduce_amount, _) => self
                .combat_resources
                .borrow_mut()
                .reduce_cooldown(skill_id, reduce_amount),
            _ => {}
        }

        self.consume_internal_events(debuffs_table);
    }
}

impl FfxivPlayer {
    pub fn get_id(&self) -> PlayerIdType {
        self.id
    }

    fn update_player_time_informations(&mut self) {
        let elapsed_time = self.elapsed_time;
        self.update_status_time(elapsed_time);
        self.update_cooldown(elapsed_time);
        self.elapsed_time = 0;
    }

    fn use_turn(
        &mut self,
        turn_info: TurnInfo,
        debuffs: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        combat_time_millisecond: TimeType,
    ) {
        let next_skill_ids = self.get_next_skill(&turn_info, debuffs.clone());
        self.insert_turn_update_internal_event(&next_skill_ids, turn_info);

        let next_skill_ids = next_skill_ids;

        for use_skill in next_skill_ids.clone() {
            let combat_resources = self.combat_resources.borrow();
            let skill = combat_resources.get_skill(use_skill.skill_id);
            let target_id = match skill.use_type {
                UseType::UseOnPartyMember => {
                    Some(combat_resources.get_next_buff_target(use_skill.skill_id))
                }
                UseType::UseOnTarget => Some(TARGET_ID),
                UseType::NoTarget => None,
            };

            if let Some(use_later_time) = use_skill.use_later_time {
                self.event_queue
                    .borrow_mut()
                    .push(Reverse(FfxivEvent::UseSkill(
                        self.id,
                        target_id,
                        use_skill.skill_id,
                        use_later_time,
                    )));
            } else {
                drop(combat_resources);

                self.use_skill(
                    use_skill.skill_id,
                    target_id,
                    debuffs.clone(),
                    combat_time_millisecond,
                );
            }
        }
    }

    fn use_skill(
        &mut self,
        skill_id: SkillIdType,
        target_id: Option<PlayerIdType>,
        debuffs: StatusTable<DebuffStatus>,
        combat_time_millisecond: TimeType,
    ) {
        let skill = self.combat_resources.borrow().get_skill(skill_id).clone();
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
            self.event_queue.borrow_mut().push(Reverse(ffxiv_event));
        }

        self.skill_logs.push(SkillLog {
            time: combat_time_millisecond,
            skill_id,
            target_id,
            buffs: self
                .buff_list
                .borrow()
                .keys()
                .map(|key| key.status_id)
                .collect(),
            debuffs: debuffs
                .borrow()
                .iter()
                .filter_map(|(key, debuff_status)| {
                    if key.player_id == self.id || debuff_status.is_raidwide {
                        Some(key.status_id)
                    } else {
                        None
                    }
                })
                .collect(),
        });
    }

    fn insert_turn_update_internal_event(
        &self,
        next_skill_ids: &[SkillUsageInfo],
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
            FfxivPlayerInternalEvent::StartCooldown(skill_id) => self
                .combat_resources
                .borrow_mut()
                .start_cooldown(*skill_id, &self),
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

    pub fn get_gcd_delay_millisecond(&self, skill: &AttackSkill) -> TimeType {
        let gcd_cooldown_millisecond = skill.get_gcd_cooldown_millisecond();
        let charging_time = skill.charging_time_millisecond;

        let gcd_cooldown = if skill.is_speed_buffed() {
            let is_auto_attack = skill.id == AUTO_ATTACK_ID;
            self.get_speed_buffed_time(gcd_cooldown_millisecond, is_auto_attack)
        } else {
            gcd_cooldown_millisecond
        };

        gcd_cooldown + charging_time
    }

    pub fn update_on_crit(&self) {
        self.combat_resources.borrow_mut().trigger_on_crit();
    }

    pub fn print_skill_debug(&self, skill_id: SkillIdType) -> String {
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

impl CooldownTimer for FfxivPlayer {
    fn update_cooldown(&mut self, time_passed: TimeType) {
        self.combat_resources
            .borrow_mut()
            .update_cooldown(time_passed);
    }
}

impl FfxivPlayer {
    pub(crate) fn get_cast_time(&self, skill: &AttackSkill) -> TimeType {
        let cast_time = skill.get_gcd_cast_time();

        if skill.is_speed_buffed() {
            self.get_speed_buffed_time(cast_time, false)
        } else {
            cast_time
        }
    }

    pub(crate) fn get_speed_buffed_time(
        &self,
        time_millisecond: TimeType,
        is_auto_attack: bool,
    ) -> TimeType {
        let speed_buff_multiplier = self.get_gcd_buff_reducer(is_auto_attack);

        self.calculate_speed_buffed_cooldown_millisecond(
            time_millisecond,
            self.power.speed_multiplier,
            speed_buff_multiplier,
        )
    }

    /// GCD buffs are calculated as "reduction multiplier" rather than divider
    /// ex) 10% speed buff, GCD isn't 2.50 / 1.10 = 2.27, but 2.50 * 0.90 = 2.25
    fn get_gcd_buff_reducer(&self, is_auto_attack: bool) -> MultiplierType {
        let mut gcd_buffs_multiplier = 1.0;
        for buff in self.buff_list.borrow().values() {
            for status_info in &buff.status_info {
                match status_info {
                    StatusInfo::SpeedPercent(buff_increase_percent) => {
                        gcd_buffs_multiplier = gcd_buffs_multiplier
                            * (1.0 - (*buff_increase_percent as MultiplierType / 100.0));
                    }
                    StatusInfo::SpeedByStack(buff_increase_percents) => {
                        let stack = (buff.stacks - 1) as usize;
                        let buff_increase = buff_increase_percents[stack] as MultiplierType;
                        gcd_buffs_multiplier =
                            gcd_buffs_multiplier * (1.0 - (buff_increase / 100.0));
                    }
                    StatusInfo::SpeedOnlyAutoAttack(buff_increase_percent) => {
                        if is_auto_attack {
                            gcd_buffs_multiplier = gcd_buffs_multiplier
                                * (1.0 - (*buff_increase_percent as MultiplierType / 100.0));
                        }
                    }
                    _ => {}
                }
            }
        }
        gcd_buffs_multiplier
    }

    pub fn new(
        id: PlayerIdType,
        job_abbrev: String,
        power: PlayerPower,
        partner_player_id1: Option<PlayerIdType>,
        partner_player_id2: Option<PlayerIdType>,
        priority_table: FfxivPriorityTable,
        buff_list: HashMap<StatusKey, BuffStatus>,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
        start_turn: FfxivEvent,
        gcd_start_time_millisecond: Option<TimeType>,
        player_count: usize,
    ) -> FfxivPlayer {
        let mut internal_event_queue = vec![];
        internal_event_queue.reserve(FFXIV_EVENT_QUEUE_INITIAL_CAPACITY);
        FfxivPlayer {
            id,
            combat_resources: RefCell::new(FfxivCombatResources::new(
                &job_abbrev,
                id,
                partner_player_id1,
                partner_player_id2,
                event_queue.clone(),
                player_count,
            )),
            job_abbrev,
            power,
            priority_table,
            buff_list: Rc::new(RefCell::new(buff_list)),
            internal_event_queue: RefCell::new(internal_event_queue),
            turn_calculator: RefCell::new(PlayerTurnCalculator::new(
                id,
                event_queue.clone(),
                gcd_start_time_millisecond,
            )),
            event_queue,
            damage_logs: vec![],
            skill_logs: vec![],
            start_turn,
            elapsed_time: 0,
        }
    }
    pub fn update_damage_log(
        &mut self,
        skill_id: SkillIdType,
        damage_profile: &DamageRdpsProfile,
        current_time_millisecond: TimeType,
    ) {
        self.damage_logs.push(DamageLog::new(
            FfxivPlayer::convert_related_skill_ids(skill_id),
            damage_profile,
            current_time_millisecond,
        ));
    }

    /// buffed skills like ley line fire IV, life surge buffed heaven's thrust
    /// are treated as separate skills by the engine, but is the same skill to the
    /// app user.
    /// Merge these buffed skills to the un-buffed original skills
    fn convert_related_skill_ids(skill_id: SkillIdType) -> SkillIdType {
        match skill_id {
            /// WAR
            /// fell cleave + inner release -> fell cleave
            111 => 104,

            /// DRK
            /// edge of shadow proc -> edge of shadow
            217 => 203,

            /// AST
            /// fall malefic lightspeed -> fall malefic
            511 => 500,

            /// DRG
            /// heavens thrust + surge
            812 => 805,
            /// drakesbane + surge
            813 => 809,

            /// MNK
            /// perfect leaping opo -> leaping opo
            914 => 900,
            /// perfect rising raptor -> rising raptor
            915 => 901,
            /// perfect pouncing coeurl -> pouncing coeurl
            916 => 902,
            /// perfect twin snakes -> twin snakes
            917 => 903,
            /// perfect demolish -> demolish
            918 => 904,
            /// perfect dragon kick -> dragon kick
            919 => 905,

            /// NIN
            /// bhavacakra meisui -> bhavacakra
            1021 => 1012,
            /// zesho meppo meisui -> zesho meppo
            1025 => 1000,

            /// SAM
            /// gekko meikyo
            1110 => 1102,
            /// kasha meikyo
            1111 => 1104,
            /// yuki meikyo
            1112 => 1105,
            /// higanbana two(kasha sen)
            1119 => 1118,
            /// higanbana three(yuki sen)
            1120 => 1118,

            /// RPR
            /// refresh slice -> slice
            1223 => 1200,
            /// refresh waxing slice -> waxing slice
            1224 => 1201,
            /// refresh infernal slice -> infernal slice
            1225 => 1202,

            /// MCH
            /// drill reassemble -> drill
            1414 => 1403,
            /// air anchor reassemble -> air anchor
            1415 => 1404,
            /// chain saw reassemble -> chain saw
            1416 => 1405,
            /// excavator reassemble -> excavator
            1419 => 1417,

            /// BLM
            /// Fire 4 Triplecast -> Fire 4
            1704 => 1703,
            /// Fire 4 Swiftcast -> Fire 4
            1719 => 1703,

            /// Fire 3 astral fire 1 -> Fire 3 umbral ice 3
            1706 => 1705,
            /// Fire 3 Opener -> Fire 3 umbral ice 3
            1718 => 1705,
            /// Fire 3 -> Fire 3 umbral ice 3

            /// Despair Triplecast -> Despair
            1708 => 1707,

            /// Blizzard 3 transpose triplecast -> Blizzard 3
            1721 => 1712,
            /// Blizzard 3 transpose swiftcast -> Blizzard 3
            1723 => 1712,

            /// flarestar triplecast -> flarestar
            1722 => 1720,

            /// RDM
            /// veraero swiftcast -> veraero
            1802 => 1801,
            /// veraero dualcast -> veraero
            1803 => 1801,
            /// veraero acceleration -> veraero
            1804 => 1801,
            /// verthunder dualcast -> verthunder swiftcast
            1806 => 1805,
            /// verthunder acceleration -> verthunder swiftcast
            1807 => 1805,
            /// riposte manafication -> enchanted riposte
            1827 => 1815,
            /// zwerchhau manafication -> enchanted zwerchhau
            1828 => 1816,
            /// redoublement manafication -> enchanted redoublement
            1829 => 1817,
            /// verholy manafication -> verholy
            1830 => 1818,
            /// verflare manafication -> verflare
            1831 => 1819,
            /// scorch manafication -> scorch
            1832 => 1822,
            /// resolution manafication -> resolution
            1833 => 1823,

            /// PCT
            /// Comet in Black Hyperphantasia -> Comet in Black
            2032 => 2022,
            /// Blizzard in Cyan Hyperphantasia -> Blizzard in Cyan
            2033 => 2013,
            /// Stone in Yellow Hyperphantasia -> Stone in Yellow
            2034 => 2014,
            /// Thunder in Magenta Hyperphantasia -> Thunder in Magenta
            2035 => 2015,

            _ => skill_id,
        }
    }

    pub fn is_melee(&self) -> bool {
        match self.job_abbrev.as_str() {
            "NIN" | "MNK" | "DRG" | "SAM" | "GNB" | "DRK" | "PLD" | "WAR" | "BRD" | "MCH"
            | "RPR" | "VPR" | "DNC" => true,
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
