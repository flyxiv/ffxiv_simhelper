use crate::id_entity::IdEntity;
use crate::live_objects::player::Player;
use crate::live_objects::turn_type::{FfxivTurnType, PlayerTurn};
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::SkillInfo;
use crate::skill::{ResourceRequirements, Skill};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, ResourceType, StackType, TimeType};
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

pub enum PriorityResult {
    True,
    DelayOgcdFor(TimeType),
    False,
}

#[derive(Clone)]
pub(crate) enum SkillPrerequisite {
    Or(Box<SkillPrerequisite>, Box<SkillPrerequisite>),
    And(Box<SkillPrerequisite>, Box<SkillPrerequisite>),
    Not(Box<SkillPrerequisite>),
    Combo(Option<IdType>),
    HasBufforDebuff(IdType),
    BufforDebuffLessThan(IdType, TimeType),
    HasResource1(ResourceType),
    HasResource2(ResourceType),
    HasStacks(IdType, StackType),
    MillisecondsBeforeBurst(TimeType),
    RelatedSkillCooldownLessThan(IdType, TimeType),
}

#[derive(Clone)]
pub(crate) struct CombatInfo {
    buff_list: Rc<RefCell<Vec<BuffStatus>>>,
    debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    milliseconds_before_burst: TimeType,
}

pub enum SkillResult<S: Skill> {
    UseSkill(Vec<SkillInfo<S>>),
    Delay(TimeType),
}

/// Stores the priority list of the job's offensive skills
/// And gets the next skill to use based on the priority list
pub(crate) trait PriorityTable<P: Player, S: Skill>: CooldownTimer {
    fn get_next_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
    ) -> Option<SkillResult<S>> {
        let current_turn = player.get_player_turn();
        let mut next_skill = None;

        if self.is_opener() {
            next_skill = self.get_opener(player);
        } else {
            next_skill = self.get_highest_priority_skill(
                buff_list.clone(),
                debuff_list.clone(),
                player,
                current_turn,
            )
        }

        self.start_cooldown(&next_skill);

        if let Some(SkillResult::UseSkill(mut skills)) = next_skill {
            self.update_stack_status(&skills, buff_list, debuff_list);
            self.add_buff_distribute_to(&mut skills, player);
            Some(SkillResult::UseSkill(
                self.add_additional_skills(&skills, player),
            ))
        } else {
            None
        }
    }

    fn add_buff_distribute_to(&self, skills: &mut Vec<SkillInfo<S>>, player: &P);

    fn update_stack_status(
        &mut self,
        skills: &Vec<SkillInfo<S>>,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    ) {
        for skill in skills.iter() {
            let skill = &skill.skill;

            self.add_resource1(skill.get_resource1_created());
            self.add_resource2(skill.get_resource2_created());

            self.update_combo(skill.get_combo());

            for resource in skill.get_resource_required() {
                match resource {
                    ResourceRequirements::StackResource1(required_resource) => {
                        self.add_resource1(-required_resource);
                    }
                    ResourceRequirements::StackResource2(required_resource) => {
                        self.add_resource2(-required_resource);
                    }
                    ResourceRequirements::UseStatus(status_id) => {
                        for debuff in debuff_list.borrow_mut().iter_mut() {
                            if debuff.get_id() == *status_id {
                                debuff.duration_left_millisecond = 0;
                            }
                        }

                        for buff in buff_list.borrow_mut().iter_mut() {
                            if buff.get_id() == *status_id {
                                buff.duration_left_millisecond = 0;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn add_resource1(&self, resource: ResourceType);
    fn add_resource2(&self, resource: ResourceType);

    fn update_combo(&mut self, combo_id: Option<IdType>);
    fn is_opener(&self) -> bool {
        self.get_turn_count() < self.get_opener_len()
    }

    fn get_opener_len(&self) -> usize;

    fn get_opener_at(&self, index: usize) -> &Option<S>;

    fn get_turn_count(&self) -> IdType;

    fn get_opener(&mut self, player: &P) -> Option<SkillResult<S>> {
        self.increment_turn();
        let next_skill = self.get_opener_at(self.get_turn_count() - 1);

        if next_skill.is_some() {
            let next_skill = next_skill.clone().unwrap();
            Some(self.make_skill_result(vec![next_skill], player))
        } else {
            None
        }
    }

    fn increment_turn(&mut self);

    fn get_highest_priority_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
        player_turn: &PlayerTurn,
    ) -> Option<SkillResult<S>> {
        let combat_info = CombatInfo {
            buff_list: buff_list.clone(),
            debuff_list: debuff_list.clone(),
            milliseconds_before_burst: player.get_millisecond_before_burst(),
        };

        for skill_priority_info in self.get_priority_table(&player_turn.turn_type).iter() {
            let skill = self
                .get_skills()
                .get(&skill_priority_info.skill_id)
                .unwrap();

            match self.meets_requirements(
                &skill_priority_info,
                &combat_info,
                skill,
                player,
                player_turn.delay_left,
            ) {
                PriorityResult::True => {
                    return Some(SkillResult::UseSkill(vec![SkillInfo {
                        guaranteed_critical_hit: self.is_guaranteed_crit(skill),
                        guaranteed_direct_hit: self.is_guaranteed_direct_hit(skill),
                        skill: skill.clone(),
                        damage_inflict_time_millisecond: player
                            .get_damage_inflict_time_millisecond(skill),
                    }]))
                }
                PriorityResult::DelayOgcdFor(delay_time) => {
                    return Some(SkillResult::Delay(delay_time))
                }
                PriorityResult::False => continue,
            }
        }

        None
    }

    fn meets_requirements(
        &self,
        skill_priority_info: &SkillPriorityInfo,
        combat_info: &CombatInfo,
        skill: &S,
        player: &P,
        delay_left: TimeType,
    ) -> PriorityResult {
        if skill.is_raidbuff() {
            if let Some(delay_time) = check_non_clipping_delay_time(skill, player) {
                return PriorityResult::DelayOgcdFor(delay_time);
            }
        }

        if delay_left < skill.get_delay_millisecond() || self.get_stack(skill) == 0 {
            return PriorityResult::False;
        }

        if let Some(prerequisite) = &skill_priority_info.prerequisite {
            self.meets_prequisite(prerequisite, combat_info, skill, player)
                .into()
        } else {
            PriorityResult::True
        }
    }

    fn get_stack(&self, skill: &S) -> StackType {
        let skill_table = self.get_skills();

        let stack_skill = skill_table.get(&skill.stack_skill_id()).unwrap();
        stack_skill.get_stacks()
    }

    fn add_additional_skills(&self, skills: &Vec<SkillInfo<S>>, player: &P) -> Vec<SkillInfo<S>>;

    fn make_skill_result(&self, skills: Vec<S>, player: &P) -> SkillResult<S> {
        let skill_results = skills
            .into_iter()
            .map(|skill| SkillInfo {
                guaranteed_critical_hit: self.is_guaranteed_crit(&skill),
                guaranteed_direct_hit: self.is_guaranteed_direct_hit(&skill),
                damage_inflict_time_millisecond: player.get_damage_inflict_time_millisecond(&skill),
                skill,
            })
            .collect_vec();

        SkillResult::UseSkill(skill_results)
    }

    fn get_skills_mut(&mut self) -> &mut SkillTable<S>;
    fn get_skills(&self) -> &SkillTable<S>;

    fn start_cooldown(&mut self, skill_info: &Option<SkillResult<S>>) {
        if let Some(skill_result) = skill_info {
            match skill_result {
                SkillResult::UseSkill(skills) => {
                    for skill_info in skills.iter() {
                        let cooldown_skill_id = skill_info.skill.stack_skill_id();

                        let cooldown_skill =
                            self.get_skills_mut().get_mut(&cooldown_skill_id).unwrap();
                        cooldown_skill.start_cooldown();
                    }
                }
                SkillResult::Delay(_) => {}
            }
        }
    }

    fn meets_prequisite(
        &self,
        prerequisite: &SkillPrerequisite,
        combat_info: &CombatInfo,
        skill: &S,
        player: &P,
    ) -> bool
    where
        P: Player,
    {
        match prerequisite {
            SkillPrerequisite::Or(left, right) => {
                self.meets_prequisite(left, combat_info, skill, player)
                    || self.meets_prequisite(right, combat_info, skill, player)
            }
            SkillPrerequisite::And(left, right) => {
                self.meets_prequisite(left, combat_info, skill, player)
                    && self.meets_prequisite(right, combat_info, skill, player)
            }
            SkillPrerequisite::Not(prerequisite) => {
                !self.meets_prequisite(prerequisite, combat_info, skill, player)
            }
            SkillPrerequisite::Combo(combo_id) => {
                if let Some(current_combo_id) = self.get_current_combo() {
                    if let Some(combo_id) = combo_id {
                        current_combo_id == *combo_id
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            SkillPrerequisite::HasBufforDebuff(status_id) => {
                self.has_status(combat_info, *status_id)
            }
            SkillPrerequisite::BufforDebuffLessThan(status_id, time_millisecond) => {
                let status_remaining_time =
                    self.find_status_remaining_time(combat_info, *status_id);

                if let Some(remaining_time) = status_remaining_time {
                    remaining_time <= *time_millisecond
                } else {
                    false
                }
            }
            SkillPrerequisite::HasResource1(resource) => self.get_resource(0) >= *resource,
            SkillPrerequisite::HasResource2(resource) => self.get_resource(1) >= *resource,

            SkillPrerequisite::MillisecondsBeforeBurst(milliseconds) => {
                *milliseconds >= combat_info.milliseconds_before_burst
            }
            SkillPrerequisite::HasStacks(skill_id, stacks) => {
                self.get_skill_stack(*skill_id) >= *stacks
            }
            SkillPrerequisite::RelatedSkillCooldownLessThan(related_skill_id, time_millisecond) => {
                let related_skill = self.get_skills().get(related_skill_id).unwrap();

                related_skill.get_current_cooldown_millisecond() <= *time_millisecond
            }
        }
    }

    fn has_status(&self, combat_info: &CombatInfo, status_id: IdType) -> bool {
        let buff_list = combat_info.buff_list.borrow();
        let debuff_list = combat_info.debuff_list.borrow();

        buff_list.iter().any(|buff| buff.id == status_id)
            || debuff_list.iter().any(|debuff| debuff.id == status_id)
    }

    fn find_status_remaining_time(
        &self,
        combat_info: &CombatInfo,
        status_id: IdType,
    ) -> Option<TimeType> {
        let buff_list = combat_info.buff_list.borrow();
        let debuff_list = combat_info.debuff_list.borrow();

        let buff_search = buff_list.iter().find(|buff| buff.id == status_id);
        let debuff_search = debuff_list.iter().find(|debuff| debuff.id == status_id);

        if let Some(buff) = buff_search {
            return Some(buff.duration_left_millisecond);
        }

        if let Some(debuff) = debuff_search {
            return Some(debuff.duration_left_millisecond);
        }

        None
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType;
    fn get_skill_stack(&self, skill_id: IdType) -> StackType;
    fn get_priority_table(&self, turn_type: &FfxivTurnType) -> &Vec<SkillPriorityInfo>;
    fn is_guaranteed_crit(&self, skill: &S) -> bool;
    fn is_guaranteed_direct_hit(&self, skill: &S) -> bool;
    fn get_current_combo(&self) -> Option<IdType>;
    fn make_skill_info(&self, skill: S, player: &P) -> SkillInfo<S> {
        SkillInfo {
            guaranteed_critical_hit: self.is_guaranteed_crit(&skill),
            guaranteed_direct_hit: self.is_guaranteed_direct_hit(&skill),
            damage_inflict_time_millisecond: player.get_damage_inflict_time_millisecond(&skill),
            skill,
        }
    }
}

fn check_non_clipping_delay_time<P: Player, S: Skill>(
    raidbuff_skill: &S,
    player: &P,
) -> Option<TimeType> {
    let current_combat_time = player.get_delay() + player.get_last_gcd_time_millisecond();
    let skill_cooldown_left = raidbuff_skill.get_current_cooldown_millisecond();

    let time_skill_comes_off_cooldown = current_combat_time + skill_cooldown_left;

    if current_combat_time - raidbuff_skill.get_delay_millisecond() > time_skill_comes_off_cooldown
    {
        Some(time_skill_comes_off_cooldown)
    } else {
        None
    }
}

impl From<bool> for PriorityResult {
    fn from(value: bool) -> Self {
        if value {
            PriorityResult::True
        } else {
            PriorityResult::False
        }
    }
}
