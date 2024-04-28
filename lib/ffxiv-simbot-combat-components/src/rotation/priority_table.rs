use crate::live_objects::player::Player;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::SkillInfo;
use crate::skill::Skill;
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

pub(crate) enum SkillPrerequisite {
    Or(SkillPrerequisite, SkillPrerequisite),
    And(SkillPrerequisite, SkillPrerequisite),
    Not(SkillPrerequisite),
    Combo(IdType),
    HasBufforDebuff(IdType),
    BufforDebuffLessThan(IdType, TimeType),
    HasResource1(ResourceType),
    HasResource2(ResourceType),
    HasStacks(IdType, StackType),
    MillisecondsBeforeBurst(TimeType),
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
pub trait PriorityTable<P: Player, S: Skill> {
    fn get_next_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
    ) -> Option<SkillResult<S>> {
        let current_turn_type = player.get_turn_type();
        let mut next_skill = None;

        if self.is_opener() {
            next_skill = self.get_opener(player);
        } else {
            next_skill = self.get_highest_priority_skill(
                buff_list.clone(),
                debuff_list.clone(),
                player,
                current_turn_type,
            )
        }

        self.start_cooldown(&next_skill);

        if let Some(SkillResult::UseSkill(skills)) = &next_skill {
            self.update_stack_status(&skills[0].skill, buff_list, debuff_list);
        }
        next_skill
    }

    fn update_stack_status(
        &mut self,
        skill: &S,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    );

    fn is_opener(&self) -> bool;
    fn get_opener(&mut self, player: &P) -> Option<SkillResult<S>>;
    fn get_highest_priority_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
        turn_type: &FfxivTurnType,
    ) -> Option<SkillResult<S>> {
        let combat_info = CombatInfo {
            buff_list: buff_list.clone(),
            debuff_list: debuff_list.clone(),
            milliseconds_before_burst: player.get_millisecond_before_burst(),
        };

        for skill_priority_info in self.get_priority_table(turn_type).iter() {
            let skill = &skill_priority_info.skill;

            match self.meets_requirements(&skill_priority_info, &combat_info, skill, player) {
                PriorityResult::True => {
                    return Some(self.make_skill_result(
                        self.add_additional_skills(&skill_priority_info.skill, player),
                        player.get_damage_inflict_time_millisecond(skill),
                    ))
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
    ) -> PriorityResult {
        if skill.is_raidbuff() {
            if let Some(delay_time) = check_non_clipping_delay_time(skill, player) {
                return PriorityResult::DelayOgcdFor(delay_time);
            }
        }

        if let Some(prerequisite) = &skill_priority_info.prerequisite {
            return self
                .meets_prequisite(prerequisite, combat_info, skill, player)
                .into();
        }

        PriorityResult::True
    }

    fn add_additional_skills(&mut self, skill_priority_info: &S, player: &P) -> Vec<S>;

    fn make_skill_result(
        &self,
        skills: Vec<S>,
        damage_inflict_time_millisecond: Option<TimeType>,
    ) -> SkillResult<S> {
        let skill_results = skills
            .into_iter()
            .map(|skill| SkillInfo {
                guaranteed_critical_hit: self.is_guaranteed_crit(&skill),
                guaranteed_direct_hit: self.is_guaranteed_direct_hit(&skill),
                skill,
                damage_inflict_time_millisecond,
            })
            .collect_vec();

        SkillResult::UseSkill(skill_results)
    }

    fn get_skills_mut(&mut self) -> &mut SkillTable;

    fn start_cooldown(&mut self, skill_info: &Option<SkillInfo<S>>) {
        if let Some(skill_info) = skill_info {
            for (skill_id, skill) in self.get_skills_mut().iter_mut() {
                if *skill_id == skill_info.skill.get_id() {
                    skill.start_cooldown();
                }
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
                    current_combo_id == *combo_id
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
                *milliseconds <= combat_info.milliseconds_before_burst
            }
            SkillPrerequisite::HasStacks(skill_id, stacks) => {
                self.get_skill_stack(*skill_id) >= *stacks
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
