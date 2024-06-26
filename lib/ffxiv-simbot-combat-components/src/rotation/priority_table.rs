use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::combat_resources::CombatResource;
use crate::event::turn_info::TurnInfo;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::simulate_status::simulate_status;
use crate::rotation::simulated_combat_resource::FirstSkillCombatSimulation;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{ResourceRequirements, NON_GCD_DELAY_MILLISECOND};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, ResourceType, StackType, TimeType};
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) enum Opener {
    GcdOpener(IdType),
    OgcdOpener((Option<IdType>, Option<IdType>)),
}

static OGCD_PENALTY: usize = 4;

#[derive(Clone)]
pub(crate) enum SkillPrerequisite {
    Or(Box<SkillPrerequisite>, Box<SkillPrerequisite>),
    And(Box<SkillPrerequisite>, Box<SkillPrerequisite>),
    Not(Box<SkillPrerequisite>),
    Combo(Option<IdType>),
    HasBufforDebuff(IdType),
    BufforDebuffLessThan(IdType, TimeType),
    HasResource(IdType, ResourceType),
    HasSkillStacks(IdType, StackType),
    MillisecondsBeforeBurst(TimeType),
    RelatedSkillCooldownLessOrEqualThan(IdType, TimeType),
    /// Greater resource id, Lesser resource id, Greater by how much amount
    /// example: (1, 2, 50), then ok if resource1 >= resource2 + 50
    ResourceGreaterOrEqualThanAnotherResourceBy(IdType, IdType, ResourceType),
}

pub(crate) struct CombatInfo {
    pub(crate) buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
    pub(crate) debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
    pub(crate) milliseconds_before_burst: TimeType,
}

impl Clone for CombatInfo {
    fn clone(&self) -> Self {
        Self {
            buff_list: Rc::new(RefCell::new(self.buff_list.borrow().clone())),
            debuff_list: Rc::new(RefCell::new(self.debuff_list.borrow().clone())),
            milliseconds_before_burst: self.milliseconds_before_burst,
        }
    }
}

#[derive(Clone)]
pub(crate) struct SkillUsageInfo {
    pub(crate) skill_id: IdType,
    pub(crate) use_later_time: Option<TimeType>,
}

#[derive(Clone)]
pub(crate) struct OgcdPlan {
    pub(crate) skill: SkillUsageInfo,
    pub(crate) priority_number: IdType,
}

impl SkillUsageInfo {
    pub(crate) fn new(skill_id: IdType) -> Self {
        Self {
            skill_id,
            use_later_time: None,
        }
    }

    pub(crate) fn new_delay(skill_id: IdType, use_later_time: Option<TimeType>) -> Self {
        Self {
            skill_id,
            use_later_time,
        }
    }
}

/// Stores the priority list of the job's offensive skills
/// And gets the next skill to use based on the priority list
pub(crate) trait PriorityTable: Sized + Clone {
    fn get_next_skill(
        &self,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        turn_info: &TurnInfo,
        player: &FfxivPlayer,
        combat_resource: &FfxivCombatResources,
    ) -> Vec<SkillUsageInfo> {
        let next_skills = if self.is_opener() {
            self.get_opener(&combat_resource, turn_info)
        } else {
            self.get_highest_priority_skills(
                debuff_list.clone(),
                player,
                turn_info,
                combat_resource,
            )
        };

        next_skills
    }

    fn is_opener(&self) -> bool {
        self.get_turn_count() < self.get_opener_len()
    }

    fn get_opener_len(&self) -> usize;

    fn get_opener_at(&self, index: usize) -> Opener;

    fn get_opener(
        &self,
        combat_resources: &FfxivCombatResources,
        turn_info: &TurnInfo,
    ) -> Vec<SkillUsageInfo> {
        self.increment_turn();
        let opener = self.get_opener_at(self.get_turn_count() - 1);

        match opener {
            Opener::GcdOpener(skill_id) => vec![SkillUsageInfo::new(skill_id)],
            Opener::OgcdOpener(ogcd_skills) => {
                let mut skills = vec![];
                let first_skill = ogcd_skills.0;
                let second_skill = ogcd_skills.1;

                let mut delay = 0;

                if let Some(first_skill_id) = first_skill {
                    let first_skill = combat_resources.get_skills().get(&first_skill_id).unwrap();
                    skills.push(SkillUsageInfo::new_delay(
                        first_skill_id,
                        Some(turn_info.lower_bound_millisecond),
                    ));
                    delay += first_skill.get_delay_millisecond();
                } else {
                    delay += NON_GCD_DELAY_MILLISECOND;
                }

                if let Some(second_skill_id) = second_skill {
                    skills.push(SkillUsageInfo::new_delay(
                        second_skill_id,
                        Some(turn_info.lower_bound_millisecond + delay),
                    ));
                }

                skills
            }
        }
    }

    fn get_highest_priority_skills(
        &self,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        player: &FfxivPlayer,
        turn_info: &TurnInfo,
        combat_resource: &FfxivCombatResources,
    ) -> Vec<SkillUsageInfo> {
        let combat_info = CombatInfo {
            buff_list: player.buff_list.clone(),
            debuff_list: debuff_list.clone(),
            milliseconds_before_burst: turn_info.get_next_burst_time(),
        };

        if matches!(turn_info.turn_type, FfxivTurnType::Gcd) {
            self.find_highest_gcd_skill(combat_info, player, combat_resource)
        } else {
            if let Some(ogcd_plans) = self.find_best_ogcd_skill_combination(
                &combat_info,
                player,
                combat_resource,
                turn_info,
            ) {
                ogcd_plans.into_iter().map(|plan| plan.skill).collect_vec()
            } else {
                vec![]
            }
        }
    }

    fn find_best_ogcd_skill_combination(
        &self,
        combat_info: &CombatInfo,
        player: &FfxivPlayer,
        combat_resource: &FfxivCombatResources,
        turn_info: &TurnInfo,
    ) -> Option<Vec<OgcdPlan>> {
        let ogcd_priority_table = self.get_ogcd_priority_table();
        let next_gcd_millisecond = turn_info.next_gcd_millisecond;
        let mut best_one_ogcd = None;

        for (priority_number, skill_priority) in ogcd_priority_table.iter().enumerate() {
            let skill = combat_resource.get_skill(skill_priority.skill_id);
            let skill_delay_millisecond = skill.get_delay_millisecond();
            let latest_time_to_use = next_gcd_millisecond - skill_delay_millisecond;

            let first_skill_start_time =
                turn_info.lower_bound_millisecond + skill.current_cooldown_millisecond;

            if first_skill_start_time <= latest_time_to_use {
                let mut combat_info_simulation = combat_info.clone();
                let mut combat_resource_simulation = combat_resource.clone();
                advance_time(
                    &mut combat_info_simulation,
                    &mut combat_resource_simulation,
                    skill.current_cooldown_millisecond,
                );

                if self.can_use_skill(
                    skill_priority,
                    &combat_info_simulation,
                    &player,
                    &combat_resource_simulation,
                ) {
                    if best_one_ogcd.is_none() {
                        best_one_ogcd = Some(vec![OgcdPlan {
                            skill: SkillUsageInfo::new_delay(
                                skill_priority.skill_id,
                                Some(first_skill_start_time),
                            ),
                            priority_number,
                        }]);
                    }

                    let second_skill_start_time = first_skill_start_time + skill_delay_millisecond;

                    if second_skill_start_time > next_gcd_millisecond - NON_GCD_DELAY_MILLISECOND {
                        continue;
                    }

                    let first_skill_simulation =
                        FirstSkillCombatSimulation::new(player.get_id(), skill);

                    advance_time(
                        &mut combat_info_simulation,
                        &mut combat_resource_simulation,
                        skill_delay_millisecond,
                    );

                    simulate_status(
                        &mut combat_info_simulation,
                        &mut combat_resource_simulation,
                        &first_skill_simulation,
                        skill.get_id(),
                    );
                    if let Some(second_skill_plan) = self.find_second_highest_ogcd_skill(
                        combat_info_simulation,
                        player,
                        combat_resource_simulation,
                        &ogcd_priority_table,
                        second_skill_start_time,
                        turn_info,
                        skill.get_id(),
                    ) {
                        let first_skill_plan = OgcdPlan {
                            skill: SkillUsageInfo::new_delay(
                                skill_priority.skill_id,
                                Some(first_skill_start_time),
                            ),
                            priority_number,
                        };

                        return Some(vec![first_skill_plan, second_skill_plan]);
                    }
                }
            }
        }

        best_one_ogcd
    }

    fn find_second_highest_ogcd_skill(
        &self,
        combat_info: CombatInfo,
        player: &FfxivPlayer,
        combat_resource: FfxivCombatResources,
        ogcd_priority_table: &Vec<SkillPriorityInfo>,
        start_time: TimeType,
        turn_info: &TurnInfo,
        first_used_skill_id: IdType,
    ) -> Option<OgcdPlan> {
        let next_gcd_millisecond = turn_info.next_gcd_millisecond;

        for (priority_number, skill_priority) in ogcd_priority_table.iter().enumerate() {
            let skill = combat_resource.get_skill(skill_priority.skill_id);

            if skill.get_id() == first_used_skill_id {
                continue;
            }

            let skill_delay_millisecond = skill.get_delay_millisecond();
            let latest_time_to_use = next_gcd_millisecond - skill_delay_millisecond;

            let skill_start_time = start_time + skill.current_cooldown_millisecond;

            if skill_start_time <= latest_time_to_use {
                if self.can_use_skill(skill_priority, &combat_info, &player, &combat_resource) {
                    return Some(OgcdPlan {
                        skill: SkillUsageInfo::new_delay(
                            skill_priority.skill_id,
                            Some(skill_start_time),
                        ),
                        priority_number,
                    });
                }
            }
        }

        None
    }

    fn find_highest_gcd_skill(
        &self,
        combat_info: CombatInfo,
        player: &FfxivPlayer,
        combat_resource: &FfxivCombatResources,
    ) -> Vec<SkillUsageInfo> {
        let gcd_priority_table = self.get_gcd_priority_table();

        for skill_priority in gcd_priority_table {
            if self.can_use_skill(skill_priority, &combat_info, player, &combat_resource) {
                return vec![SkillUsageInfo::new(skill_priority.skill_id)];
            }
        }

        return vec![];
    }

    fn can_use_skill(
        &self,
        skill_priority: &SkillPriorityInfo,
        combat_info: &CombatInfo,
        player: &FfxivPlayer,
        combat_resource: &FfxivCombatResources,
    ) -> bool {
        let skill = combat_resource.get_skill(skill_priority.skill_id);

        if combat_resource.get_stack(skill.get_id()) == 0
            || !self.meets_requirements(combat_info, combat_resource, skill, player.get_id())
        {
            return false;
        }

        if skill_priority.prerequisite.is_none() {
            return true;
        }

        let prerequisite = skill_priority.prerequisite.clone().unwrap();
        self.meets_prequisite(&prerequisite, &combat_resource, &combat_info, skill, player)
    }

    fn get_gcd_priority_table(&self) -> &Vec<SkillPriorityInfo>;
    fn get_ogcd_priority_table(&self) -> &Vec<SkillPriorityInfo>;

    fn meets_requirements(
        &self,
        combat_info: &CombatInfo,
        combat_resource: &FfxivCombatResources,
        skill: &AttackSkill,
        player_id: IdType,
    ) -> bool {
        for resource_required in &skill.resource_required {
            match resource_required {
                ResourceRequirements::Resource(id, resource) => {
                    if combat_resource.get_resource(*id) < *resource {
                        return false;
                    }
                }
                ResourceRequirements::UseBuff(status_id) => {
                    if !self.has_status(combat_info, *status_id, player_id) {
                        return false;
                    }
                }
                ResourceRequirements::UseDebuff(status_id) => {
                    if !self.has_status(combat_info, *status_id, player_id) {
                        return false;
                    }
                }
                ResourceRequirements::CheckStatus(status_id) => {
                    if !self.has_status(combat_info, *status_id, player_id) {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    fn meets_prequisite(
        &self,
        prerequisite: &SkillPrerequisite,
        combat_resources: &FfxivCombatResources,
        combat_info: &CombatInfo,
        skill: &AttackSkill,
        player: &FfxivPlayer,
    ) -> bool {
        match prerequisite {
            SkillPrerequisite::Or(left, right) => {
                self.meets_prequisite(left, combat_resources, combat_info, skill, player)
                    || self.meets_prequisite(right, combat_resources, combat_info, skill, player)
            }
            SkillPrerequisite::And(left, right) => {
                self.meets_prequisite(left, combat_resources, combat_info, skill, player)
                    && self.meets_prequisite(right, combat_resources, combat_info, skill, player)
            }
            SkillPrerequisite::Not(prerequisite) => {
                !self.meets_prequisite(prerequisite, combat_resources, combat_info, skill, player)
            }
            SkillPrerequisite::Combo(combo_id) => {
                if let Some(current_combo_id) = combat_resources.get_current_combo() {
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
                self.has_status(combat_info, *status_id, player.get_id())
            }
            SkillPrerequisite::BufforDebuffLessThan(status_id, time_millisecond) => {
                let status_remaining_time =
                    self.find_status_remaining_time(combat_info, *status_id, player.get_id());

                if let Some(remaining_time) = status_remaining_time {
                    remaining_time <= *time_millisecond
                } else {
                    false
                }
            }
            SkillPrerequisite::HasResource(resource_id, resource) => {
                combat_resources.get_resource(*resource_id) >= *resource
            }

            SkillPrerequisite::MillisecondsBeforeBurst(milliseconds) => {
                *milliseconds >= combat_info.milliseconds_before_burst
            }
            SkillPrerequisite::HasSkillStacks(skill_id, stacks) => {
                combat_resources.get_stack(*skill_id) >= *stacks
            }
            SkillPrerequisite::RelatedSkillCooldownLessOrEqualThan(
                related_skill_id,
                time_millisecond,
            ) => {
                let related_skill = combat_resources.get_skills().get(related_skill_id).unwrap();

                related_skill.get_current_cooldown_millisecond() <= *time_millisecond
            }
            SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(
                greater_resource_id,
                lesser_resource_id,
                amount,
            ) => {
                let greater_resource = combat_resources.get_resource(*greater_resource_id);
                let lesser_resource = combat_resources.get_resource(*lesser_resource_id);

                greater_resource >= lesser_resource + *amount
            }
        }
    }

    #[inline]
    fn has_status(&self, combat_info: &CombatInfo, status_id: IdType, player_id: IdType) -> bool {
        let key = StatusKey::new(status_id, player_id);

        combat_info.buff_list.borrow().get(&key).is_some()
            || combat_info.debuff_list.borrow().get(&key).is_some()
    }

    fn find_status_remaining_time(
        &self,
        combat_info: &CombatInfo,
        status_id: IdType,
        player_id: IdType,
    ) -> Option<TimeType> {
        let key = StatusKey::new(status_id, player_id);
        let buff_list = combat_info.buff_list.borrow();
        let debuff_list = combat_info.debuff_list.borrow();

        let buff_search = buff_list.get(&key);
        let debuff_search = debuff_list.get(&key);

        if let Some(buff) = buff_search {
            return Some(buff.duration_left_millisecond);
        }

        if let Some(debuff) = debuff_search {
            return Some(debuff.duration_left_millisecond);
        }

        None
    }

    fn increment_turn(&self);
    fn get_turn_count(&self) -> IdType;
}

fn advance_time(
    combat_info: &mut CombatInfo,
    combat_resources: &mut FfxivCombatResources,
    elapsed_time: TimeType,
) {
    for buff in combat_info.buff_list.borrow_mut().values_mut() {
        buff.duration_left_millisecond -= elapsed_time;
    }

    for debuff in combat_info.debuff_list.borrow_mut().values_mut() {
        debuff.duration_left_millisecond -= elapsed_time;
    }

    combat_info
        .buff_list
        .borrow_mut()
        .retain(|_, buff| buff.duration_left_millisecond > 0);
    combat_info
        .debuff_list
        .borrow_mut()
        .retain(|_, debuff| debuff.duration_left_millisecond > 0);
    combat_info.milliseconds_before_burst =
        max(combat_info.milliseconds_before_burst - elapsed_time, 0);

    combat_resources.update_cooldown(elapsed_time);
}

/// first_set must have the bigger ogcd count.
fn get_higher_priority_ogcd_set(
    first_set: Vec<OgcdPlan>,
    second_set: Vec<OgcdPlan>,
) -> Vec<OgcdPlan> {
    let penalty = if first_set.len() > second_set.len() {
        OGCD_PENALTY
    } else {
        0
    };

    let first_set_priority = first_set
        .iter()
        .map(|plan| plan.priority_number)
        .sum::<IdType>();
    let second_set_priority = second_set
        .iter()
        .map(|plan| plan.priority_number)
        .sum::<IdType>();

    let first_set_better = first_set_priority <= second_set_priority + penalty;

    if first_set_better {
        first_set
    } else {
        second_set
    }
}
