use crate::live_objects::player::Player;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::SkillInfo;
use crate::skill::skill::Skill;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, ResourceType, StackType};
use std::cell::RefCell;
use std::rc::Rc;
use crate::live_objects::turn_type::FfxivTurnType;

pub(crate) enum SkillPrerequisite {
    Or(SkillPrerequisite, SkillPrerequisite),
    And(SkillPrerequisite, SkillPrerequisite),
    Combo(IdType),
    HasBufforDebuff(IdType),
    HasResource(ResourceType),
    HasStacks(StackType),
    IsBurst,
}

#[derive(Clone)]
pub(crate) struct CombatInfo {
    buff_list: Rc<RefCell<Vec<BuffStatus>>>,
    debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    is_burst: bool,
}

/// Stores the priority list of the job's offensive skills
/// And gets the next skill to use based on the priority list
pub trait PriorityTable<P: Player, S: Skill> {
    fn get_next_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
    ) -> Option<SkillInfo<S>> {
        let current_turn_type = player.get_turn_type();
        let mut next_skill = None;

        if self.is_opener() {
            next_skill = self.get_opener();
        } else {
            next_skill =
                self.get_highest_priority_skill(buff_list, debuff_list, player, current_turn_type)
        }

        self.start_cooldown(&next_skill);
        next_skill
    }

    fn is_opener(&self) -> bool;
    fn get_opener(&self) -> Option<SkillInfo<S>>;
    fn get_highest_priority_skill(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
        turn_type: &FfxivTurnType,
    ) -> Option<SkillInfo<S>> {
        let combat_info = CombatInfo {
            buff_list: buff_list.clone(),
            debuff_list: debuff_list.clone(),
            is_burst: turn_type.is_burst(),
        };

        for skill_priority_info in self.get_priority_table(turn_type).iter() {
            let skill = &skill_priority_info.skill;

            if self.meets_requirements(&skill_priority_info, &combat_info, skill, player) {
                return Some(self.make_skill_info(&skill_priority_info, player.));
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
    ) -> bool {
        if let Some(prerequisite) = &skill_priority_info.prerequisite {
            return self.meets_prequisite(prerequisite, combat_info, skill, player);
        }

        true
    }

    #[inline]
    fn make_skill_info(&self, skill_priority_info: &SkillPriorityInfo) -> SkillInfo<S> {
        SkillInfo {
            skill: skill_priority_info.skill.clone(),
            guaranteed_critical_hit: self.is_guaranteed_crit(&skill_priority_info.skill),
            guaranteed_direct_hit: self.is_guaranteed_direct_hit(&skill_priority_info.skill),
            damage_inflict_time_millisecond: None,
        }
    }

    fn get_skills_mut(&mut self) -> &mut Vec<S>;

    fn start_cooldown(&mut self, skill_info: &Option<SkillInfo<S>>) {
        if let Some(skill_info) = skill_info {
            for skill in self.get_skills_mut().iter_mut() {
                if skill.get_id() == skill_info.skill.get_id() {
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
            SkillPrerequisite::HasResource(resource) => self.get_resource() >= *resource,
            SkillPrerequisite::IsBurst => combat_info.is_burst,
            SkillPrerequisite::HasStacks(stacks) => self.get_skill_stack(skill.get_id()) >= *stacks,
        }
    }

    fn has_status(&self, combat_info: &CombatInfo, status_id: IdType) -> bool {
        let buff_list = combat_info.buff_list.borrow();
        let debuff_list = combat_info.debuff_list.borrow();

        buff_list.iter().any(|buff| buff.id == status_id)
            || debuff_list.iter().any(|debuff| debuff.id == status_id)
    }

    fn get_resource(&self) -> ResourceType;
    fn get_skill_stack(&self, skill_id: IdType) -> StackType;
    fn get_priority_table(&self, turn_type: &FfxivTurnType) -> &Vec<SkillPriorityInfo>;
    fn is_guaranteed_crit(&self, skill: &S) -> bool;
    fn is_guaranteed_direct_hit(&self, skill: &S) -> bool;
    fn get_current_combo(&self) -> Option<IdType>;
}
