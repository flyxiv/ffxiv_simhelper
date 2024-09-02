use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::combat_resources::CombatResource;
use crate::event::turn_info::TurnInfo;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::priority_simulation_data::{
    to_priority_decision_table, PriorityDecisionTable, TruncatedAttackSkill, TruncatedBuffStatus,
    TruncatedDebuffStatus,
};
use crate::rotation::simulate_status::simulate_status;
use crate::rotation::simulated_combat_resource::FirstSkillCombatSimulation;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{ResourceRequirements, NON_GCD_DELAY_MILLISECOND};
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, SkillIdType, StatusIdType, TimeType};
use crate::types::{ResourceType, StackType};
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) enum Opener {
    GcdOpener(SkillIdType),
    OgcdOpener((Option<SkillIdType>, Option<SkillIdType>)),
}

#[derive(Clone)]
pub(crate) enum SkillPrerequisite {
    Or(Box<SkillPrerequisite>, Box<SkillPrerequisite>),
    And(Box<SkillPrerequisite>, Box<SkillPrerequisite>),
    Not(Box<SkillPrerequisite>),
    Combo(ComboType),
    HasBufforDebuff(SkillIdType),
    BufforDebuffLessThan(SkillIdType, TimeType),
    HasResource(ResourceIdType, ResourceType),
    HasResourceExactly(ResourceIdType, ResourceType),
    HasSkillStacks(SkillIdType, StackType),
    MillisecondsBeforeBurst(TimeType),
    RelatedSkillCooldownLessOrEqualThan(SkillIdType, TimeType),

    /// Greater resource id, Lesser resource id, Greater by how much amount
    /// example: (1, 2, 50), then ok if resource1 >= resource2 + 50
    ResourceGreaterOrEqualThanAnotherResourceBy(ResourceIdType, ResourceIdType, ResourceType),

    /// Skill1 id, Skill2 id
    BuffGreaterDurationThan(SkillIdType, SkillIdType),
}

#[derive(Clone)]
pub(crate) struct SkillUsageInfo {
    pub(crate) skill_id: SkillIdType,
    pub(crate) use_later_time: Option<TimeType>,
}

#[derive(Clone)]
pub(crate) struct OgcdPlan {
    pub(crate) skill: SkillUsageInfo,

    #[allow(unused)]
    pub(crate) priority_number: SkillIdType,
}

impl SkillUsageInfo {
    pub(crate) fn new(skill_id: SkillIdType) -> Self {
        Self {
            skill_id,
            use_later_time: None,
        }
    }

    pub(crate) fn new_delay(skill_id: SkillIdType, use_later_time: Option<TimeType>) -> Self {
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
        self.get_turn_count() < self.get_opener_len() as SkillIdType
    }

    fn get_opener_len(&self) -> usize;

    fn get_opener_at(&self, index: usize) -> Opener;

    fn get_opener(
        &self,
        combat_resources: &FfxivCombatResources,
        turn_info: &TurnInfo,
    ) -> Vec<SkillUsageInfo> {
        let opener = self.get_opener_at(self.get_turn_count() as usize);
        self.increment_turn();

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
        let buffs_only_self: HashMap<StatusKey, TruncatedBuffStatus> = player
            .buff_list
            .borrow()
            .iter()
            .filter_map(|(&key, buff)| {
                if key.player_id == player.get_id() {
                    Some((key, TruncatedBuffStatus::from(buff)))
                } else {
                    None
                }
            })
            .collect();
        let debuffs_only_self: HashMap<StatusKey, TruncatedDebuffStatus> = debuff_list
            .borrow()
            .iter()
            .filter_map(|(&key, debuff)| {
                if key.player_id == player.get_id() {
                    Some((key, TruncatedDebuffStatus::from(debuff)))
                } else {
                    None
                }
            })
            .collect();

        if matches!(turn_info.turn_type, FfxivTurnType::Gcd) {
            self.find_highest_gcd_skill(
                combat_resource,
                buffs_only_self,
                debuffs_only_self,
                turn_info.get_next_burst_time(),
                player,
            )
        } else {
            let priority_simulation_data = None;

            if let Some(ogcd_plans) = self.find_best_ogcd_skill_combination(
                priority_simulation_data,
                buffs_only_self,
                debuffs_only_self,
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
        mut priority_simulation_data: Option<PriorityDecisionTable>,
        buffs_only_self: HashMap<StatusKey, TruncatedBuffStatus>,
        debuffs_only_self: HashMap<StatusKey, TruncatedDebuffStatus>,
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
            let skill_cooldown = if skill.stacks >= 1 {
                0
            } else {
                skill.current_cooldown_millisecond
            };

            let first_skill_start_time = turn_info.lower_bound_millisecond + skill_cooldown;

            if first_skill_start_time <= latest_time_to_use {
                if priority_simulation_data.is_none() {
                    priority_simulation_data = Some(to_priority_decision_table(
                        &buffs_only_self,
                        &debuffs_only_self,
                        turn_info.get_next_burst_time(),
                        combat_resource,
                    ));
                }

                let mut priority_simulation_data_simulation =
                    priority_simulation_data.clone().unwrap();
                advance_time(&mut priority_simulation_data_simulation, skill_cooldown);

                if self.can_use_skill(
                    skill_priority,
                    &priority_simulation_data_simulation,
                    &player,
                ) {
                    if best_one_ogcd.is_none() {
                        best_one_ogcd = Some(vec![OgcdPlan {
                            skill: SkillUsageInfo::new_delay(
                                skill_priority.skill_id,
                                Some(first_skill_start_time),
                            ),
                            priority_number: priority_number as SkillIdType,
                        }]);
                    }

                    let second_skill_start_time = first_skill_start_time + skill_delay_millisecond;

                    if second_skill_start_time > next_gcd_millisecond - NON_GCD_DELAY_MILLISECOND {
                        continue;
                    }

                    let first_skill_simulation =
                        FirstSkillCombatSimulation::new(player.get_id(), skill);

                    advance_time(
                        &mut priority_simulation_data_simulation,
                        skill_delay_millisecond,
                    );

                    simulate_status(
                        &mut priority_simulation_data_simulation,
                        &first_skill_simulation,
                        skill.get_id(),
                    );
                    if let Some(second_skill_plan) = self.find_second_highest_ogcd_skill(
                        priority_simulation_data_simulation,
                        player,
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
                            priority_number: priority_number as SkillIdType,
                        };

                        let double_weave_plan = Some(vec![first_skill_plan, second_skill_plan]);

                        if has_important_skill(&best_one_ogcd)
                            && !has_important_skill(&double_weave_plan)
                        {
                            return best_one_ogcd;
                        } else {
                            return double_weave_plan;
                        }
                    } else {
                        return best_one_ogcd;
                    }
                }
            }
        }

        best_one_ogcd
    }

    fn find_second_highest_ogcd_skill(
        &self,
        priority_simulation_data: PriorityDecisionTable,
        player: &FfxivPlayer,
        ogcd_priority_table: &[SkillPriorityInfo],
        start_time: TimeType,
        turn_info: &TurnInfo,
        first_used_skill_id: SkillIdType,
    ) -> Option<OgcdPlan> {
        let next_gcd_millisecond = turn_info.next_gcd_millisecond;

        for (priority_number, skill_priority) in ogcd_priority_table.iter().enumerate() {
            let skill = priority_simulation_data
                .skill_list
                .get(&skill_priority.skill_id)
                .unwrap();

            if skill.id == first_used_skill_id {
                continue;
            }

            let skill_delay_millisecond = skill.get_delay_millisecond();
            let latest_time_to_use = next_gcd_millisecond - skill_delay_millisecond;

            let skill_cooldown = if skill.stacks >= 1 {
                0
            } else {
                skill.current_cooldown_millisecond
            };

            let skill_start_time = start_time + skill_cooldown;

            if skill_start_time <= latest_time_to_use {
                if self.can_use_skill(skill_priority, &priority_simulation_data, &player) {
                    return Some(OgcdPlan {
                        skill: SkillUsageInfo::new_delay(
                            skill_priority.skill_id,
                            Some(skill_start_time),
                        ),
                        priority_number: priority_number as SkillIdType,
                    });
                }
            }
        }

        None
    }

    fn find_highest_gcd_skill(
        &self,
        combat_resource: &FfxivCombatResources,
        buffs_only_self: HashMap<StatusKey, TruncatedBuffStatus>,
        debuffs_only_self: HashMap<StatusKey, TruncatedDebuffStatus>,
        milliseconds_before_burst: TimeType,
        player: &FfxivPlayer,
    ) -> Vec<SkillUsageInfo> {
        let gcd_priority_table = self.get_gcd_priority_table();

        for skill_priority in gcd_priority_table {
            if self.can_use_skill_gcd(
                skill_priority,
                &buffs_only_self,
                &debuffs_only_self,
                milliseconds_before_burst,
                player,
                combat_resource,
            ) {
                return vec![SkillUsageInfo::new(skill_priority.skill_id)];
            }
        }

        return vec![];
    }

    fn meets_requirements_gcd(
        &self,
        buff_list: &HashMap<StatusKey, TruncatedBuffStatus>,
        debuff_list: &HashMap<StatusKey, TruncatedDebuffStatus>,
        combat_resource: &FfxivCombatResources,
        skill: &AttackSkill,
        player_id: PlayerIdType,
    ) -> bool {
        for resource_required in &skill.resource_required {
            match resource_required {
                ResourceRequirements::Resource(id, resource) => {
                    if combat_resource.get_resource(*id) < *resource {
                        return false;
                    }
                }
                ResourceRequirements::UseBuff(status_id) => {
                    if !self.has_status_gcd(buff_list, debuff_list, *status_id, player_id) {
                        return false;
                    }
                }
                ResourceRequirements::UseDebuff(status_id) => {
                    if !self.has_status_gcd(buff_list, debuff_list, *status_id, player_id) {
                        return false;
                    }
                }
                ResourceRequirements::CheckStatus(status_id) => {
                    if !self.has_status_gcd(buff_list, debuff_list, *status_id, player_id) {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    fn can_use_skill_gcd(
        &self,
        skill_priority: &SkillPriorityInfo,
        buff_list: &HashMap<StatusKey, TruncatedBuffStatus>,
        debuff_list: &HashMap<StatusKey, TruncatedDebuffStatus>,
        milliseconds_before_burst: TimeType,
        player: &FfxivPlayer,
        combat_resource: &FfxivCombatResources,
    ) -> bool {
        let skill = combat_resource.get_skill(skill_priority.skill_id);

        if combat_resource.get_stack(skill.get_id()) == 0
            || !self.meets_requirements_gcd(
                buff_list,
                debuff_list,
                combat_resource,
                skill,
                player.get_id(),
            )
        {
            return false;
        }

        if skill_priority.prerequisite.is_none() {
            return true;
        }

        let prerequisite = skill_priority.prerequisite.clone().unwrap();
        self.meets_prequisite_gcd(
            &prerequisite,
            combat_resource,
            &buff_list,
            &debuff_list,
            milliseconds_before_burst,
            skill,
            player,
        )
    }

    fn meets_prequisite_gcd(
        &self,
        prerequisite: &SkillPrerequisite,
        combat_resources: &FfxivCombatResources,
        buff_list: &HashMap<StatusKey, TruncatedBuffStatus>,
        debuff_list: &HashMap<StatusKey, TruncatedDebuffStatus>,
        milliseconds_before_burst: TimeType,
        skill: &AttackSkill,
        player: &FfxivPlayer,
    ) -> bool {
        match prerequisite {
            SkillPrerequisite::Or(left, right) => {
                self.meets_prequisite_gcd(
                    left,
                    combat_resources,
                    buff_list,
                    debuff_list,
                    milliseconds_before_burst,
                    skill,
                    player,
                ) || self.meets_prequisite_gcd(
                    right,
                    combat_resources,
                    buff_list,
                    debuff_list,
                    milliseconds_before_burst,
                    skill,
                    player,
                )
            }
            SkillPrerequisite::And(left, right) => {
                self.meets_prequisite_gcd(
                    left,
                    combat_resources,
                    buff_list,
                    debuff_list,
                    milliseconds_before_burst,
                    skill,
                    player,
                ) && self.meets_prequisite_gcd(
                    right,
                    combat_resources,
                    buff_list,
                    debuff_list,
                    milliseconds_before_burst,
                    skill,
                    player,
                )
            }
            SkillPrerequisite::Not(prerequisite) => !self.meets_prequisite_gcd(
                prerequisite,
                combat_resources,
                buff_list,
                debuff_list,
                milliseconds_before_burst,
                skill,
                player,
            ),
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
                self.has_status_gcd(buff_list, debuff_list, *status_id, player.get_id())
            }
            SkillPrerequisite::BufforDebuffLessThan(status_id, time_millisecond) => {
                let status_remaining_time = self.find_status_remaining_time_gcd(
                    buff_list,
                    debuff_list,
                    *status_id,
                    player.get_id(),
                );

                if let Some(remaining_time) = status_remaining_time {
                    remaining_time <= *time_millisecond
                } else {
                    true
                }
            }
            SkillPrerequisite::HasResource(resource_id, resource) => {
                combat_resources.get_resource(*resource_id) >= *resource
            }
            SkillPrerequisite::HasResourceExactly(resource_id, resource) => {
                combat_resources.get_resource(*resource_id) == *resource
            }
            SkillPrerequisite::MillisecondsBeforeBurst(milliseconds) => {
                *milliseconds >= milliseconds_before_burst
            }
            SkillPrerequisite::HasSkillStacks(skill_id, stacks) => {
                combat_resources.get_stack(*skill_id) >= *stacks
            }
            SkillPrerequisite::RelatedSkillCooldownLessOrEqualThan(
                related_skill_id,
                time_millisecond,
            ) => {
                let related_skill = combat_resources.get_skills().get(related_skill_id).unwrap();

                related_skill.current_cooldown_millisecond <= *time_millisecond
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
            SkillPrerequisite::BuffGreaterDurationThan(buff1_id, buff2_id) => {
                let buff1_remaining_time = self.find_status_remaining_time_gcd(
                    buff_list,
                    debuff_list,
                    *buff1_id,
                    player.get_id(),
                );
                let buff2_remaining_time = self.find_status_remaining_time_gcd(
                    buff_list,
                    debuff_list,
                    *buff2_id,
                    player.get_id(),
                );

                if let Some(buff1_remaining_time) = buff1_remaining_time {
                    if let Some(buff2_remaining_time) = buff2_remaining_time {
                        buff1_remaining_time > buff2_remaining_time
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
        }
    }

    fn find_status_remaining_time_gcd(
        &self,
        buff_list: &HashMap<StatusKey, TruncatedBuffStatus>,
        debuff_list: &HashMap<StatusKey, TruncatedDebuffStatus>,
        status_id: StatusIdType,
        player_id: PlayerIdType,
    ) -> Option<TimeType> {
        let key = StatusKey::new(status_id, player_id);

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

    #[inline]
    fn has_status_gcd(
        &self,
        buff_list: &HashMap<StatusKey, TruncatedBuffStatus>,
        debuff_list: &HashMap<StatusKey, TruncatedDebuffStatus>,
        status_id: StatusIdType,
        player_id: PlayerIdType,
    ) -> bool {
        let key = StatusKey::new(status_id, player_id);

        buff_list.get(&key).is_some() || debuff_list.get(&key).is_some()
    }

    fn can_use_skill(
        &self,
        skill_priority: &SkillPriorityInfo,
        priority_simulation_data: &PriorityDecisionTable,
        player: &FfxivPlayer,
    ) -> bool {
        let skill = priority_simulation_data
            .skill_list
            .get(&skill_priority.skill_id)
            .unwrap();

        if skill.stacks == 0
            || !self.meets_requirements(priority_simulation_data, skill, player.get_id())
        {
            return false;
        }

        if skill_priority.prerequisite.is_none() {
            return true;
        }

        let prerequisite = skill_priority.prerequisite.clone().unwrap();
        self.meets_prequisite(&prerequisite, &priority_simulation_data, skill, player)
    }

    fn get_gcd_priority_table(&self) -> &[SkillPriorityInfo];
    fn get_ogcd_priority_table(&self) -> &[SkillPriorityInfo];

    fn meets_requirements(
        &self,
        priority_simulation_data: &PriorityDecisionTable,
        skill: &TruncatedAttackSkill,
        player_id: PlayerIdType,
    ) -> bool {
        for resource_required in &skill.resource_required {
            match resource_required {
                ResourceRequirements::Resource(id, resource) => {
                    if priority_simulation_data.resources[*id as usize] < *resource {
                        return false;
                    }
                }
                ResourceRequirements::UseBuff(status_id) => {
                    if !self.has_status(priority_simulation_data, *status_id, player_id) {
                        return false;
                    }
                }
                ResourceRequirements::UseDebuff(status_id) => {
                    if !self.has_status(priority_simulation_data, *status_id, player_id) {
                        return false;
                    }
                }
                ResourceRequirements::CheckStatus(status_id) => {
                    if !self.has_status(priority_simulation_data, *status_id, player_id) {
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
        priority_simulation_data: &PriorityDecisionTable,
        skill: &TruncatedAttackSkill,
        player: &FfxivPlayer,
    ) -> bool {
        match prerequisite {
            SkillPrerequisite::Or(left, right) => {
                self.meets_prequisite(left, priority_simulation_data, skill, player)
                    || self.meets_prequisite(right, priority_simulation_data, skill, player)
            }
            SkillPrerequisite::And(left, right) => {
                self.meets_prequisite(left, priority_simulation_data, skill, player)
                    && self.meets_prequisite(right, priority_simulation_data, skill, player)
            }
            SkillPrerequisite::Not(prerequisite) => {
                !self.meets_prequisite(prerequisite, priority_simulation_data, skill, player)
            }
            SkillPrerequisite::Combo(combo_id) => {
                if let Some(current_combo_id) = priority_simulation_data.combo {
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
                self.has_status(priority_simulation_data, *status_id, player.get_id())
            }
            SkillPrerequisite::BufforDebuffLessThan(status_id, time_millisecond) => {
                let status_remaining_time = self.find_status_remaining_time(
                    priority_simulation_data,
                    *status_id,
                    player.get_id(),
                );

                if let Some(remaining_time) = status_remaining_time {
                    remaining_time <= *time_millisecond
                } else {
                    true
                }
            }
            SkillPrerequisite::HasResource(resource_id, resource) => {
                priority_simulation_data.resources[*resource_id as usize] >= *resource
            }
            SkillPrerequisite::HasResourceExactly(resource_id, resource) => {
                priority_simulation_data.get_resource(*resource_id) == *resource
            }
            SkillPrerequisite::MillisecondsBeforeBurst(milliseconds) => {
                *milliseconds >= priority_simulation_data.milliseconds_before_burst
            }
            SkillPrerequisite::HasSkillStacks(skill_id, stacks) => {
                priority_simulation_data
                    .skill_list
                    .get(skill_id)
                    .unwrap()
                    .stacks
                    >= *stacks
            }
            SkillPrerequisite::RelatedSkillCooldownLessOrEqualThan(
                related_skill_id,
                time_millisecond,
            ) => {
                let related_skill = priority_simulation_data
                    .skill_list
                    .get(related_skill_id)
                    .unwrap();

                related_skill.current_cooldown_millisecond <= *time_millisecond
            }
            SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(
                greater_resource_id,
                lesser_resource_id,
                amount,
            ) => {
                let greater_resource = priority_simulation_data.get_resource(*greater_resource_id);
                let lesser_resource = priority_simulation_data.get_resource(*lesser_resource_id);

                greater_resource >= lesser_resource + *amount
            }
            SkillPrerequisite::BuffGreaterDurationThan(buff1_id, buff2_id) => {
                let buff1_remaining_time = self.find_status_remaining_time(
                    priority_simulation_data,
                    *buff1_id,
                    player.get_id(),
                );
                let buff2_remaining_time = self.find_status_remaining_time(
                    priority_simulation_data,
                    *buff2_id,
                    player.get_id(),
                );

                if let Some(buff1_remaining_time) = buff1_remaining_time {
                    if let Some(buff2_remaining_time) = buff2_remaining_time {
                        buff1_remaining_time > buff2_remaining_time
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
        }
    }

    #[inline]
    fn has_status(
        &self,
        priority_simulation_data: &PriorityDecisionTable,
        status_id: SkillIdType,
        player_id: PlayerIdType,
    ) -> bool {
        let key = StatusKey::new(status_id, player_id);

        priority_simulation_data.buff_list.get(&key).is_some()
            || priority_simulation_data.debuff_list.get(&key).is_some()
    }

    fn find_status_remaining_time(
        &self,
        priority_simulation_data: &PriorityDecisionTable,
        status_id: SkillIdType,
        player_id: PlayerIdType,
    ) -> Option<TimeType> {
        let key = StatusKey::new(status_id, player_id);
        let buff_list = &priority_simulation_data.buff_list;
        let debuff_list = &priority_simulation_data.debuff_list;

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
    fn get_turn_count(&self) -> SkillIdType;
}

fn advance_time(priority_simulation_data: &mut PriorityDecisionTable, elapsed_time: TimeType) {
    for buff in priority_simulation_data.buff_list.values_mut() {
        buff.duration_left_millisecond -= elapsed_time;
    }

    for debuff in priority_simulation_data.debuff_list.values_mut() {
        debuff.duration_left_millisecond -= elapsed_time;
    }

    priority_simulation_data
        .buff_list
        .retain(|_, buff| buff.duration_left_millisecond > 0);
    priority_simulation_data
        .debuff_list
        .retain(|_, debuff| debuff.duration_left_millisecond > 0);
    priority_simulation_data.milliseconds_before_burst = max(
        priority_simulation_data.milliseconds_before_burst - elapsed_time,
        0,
    );

    priority_simulation_data.update_cooldown(elapsed_time);
}

fn has_important_skill(ogcd_plans: &Option<Vec<OgcdPlan>>) -> bool {
    if let Some(ogcd_plans) = ogcd_plans {
        for plan in ogcd_plans {
            if plan.skill.skill_id == 1716 {
                return true;
            }
        }
    }

    false
}
