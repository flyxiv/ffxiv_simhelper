use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::combat_resources::CombatResource;
use crate::event::turn_info::TurnInfo;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::information_needed_for_rotation_decision::InformationNeededForRotationDecision;
use crate::rotation::skill_simulation_event::{
    extract_skill_simulation_event, simulate_resources, SkillSimulationEvent,
};
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{ResourceRequirements, NON_GCD_DELAY_MILLISECOND};
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, ResourceIdType, SkillIdType, StatusIdType, TimeType};
use crate::types::{ResourceType, StackType};
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::{max, min};
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
        let buffs = player.buff_list.borrow();
        let debuffs = debuff_list.borrow();

        let information_needed = InformationNeededForRotationDecision::new(
            &buffs,
            &debuffs,
            combat_resource,
            turn_info,
            player.get_id(),
        );

        if matches!(turn_info.turn_type, FfxivTurnType::Gcd) {
            self.find_highest_gcd_skill(information_needed)
        } else {
            if let Some(ogcd_plans) =
                self.find_best_ogcd_skill_combination(information_needed, turn_info)
            {
                ogcd_plans.into_iter().map(|plan| plan.skill).collect_vec()
            } else {
                vec![]
            }
        }
    }

    fn find_best_ogcd_skill_combination(
        &self,
        information_needed: InformationNeededForRotationDecision,
        turn_info: &TurnInfo,
    ) -> Option<Vec<OgcdPlan>> {
        let ogcd_priority_table = self.get_ogcd_priority_table();
        let next_gcd_millisecond = turn_info.next_gcd_millisecond;
        let mut best_one_ogcd = None;

        for (priority_number, skill_priority) in ogcd_priority_table.iter().enumerate() {
            let skill = information_needed
                .combat_resources
                .get_skill(skill_priority.skill_id);
            let skill_delay_millisecond = skill.get_delay_millisecond();
            let latest_time_to_use = next_gcd_millisecond - skill_delay_millisecond;

            let skill_cooldown = if skill.stacks >= 1 {
                0
            } else {
                skill.current_cooldown_millisecond
            };

            let first_skill_start_time = turn_info.lower_bound_millisecond + skill_cooldown;

            if first_skill_start_time <= latest_time_to_use {
                let mut first_skill_offset_time = skill_cooldown;

                if self.can_use_skill(
                    information_needed,
                    skill_priority,
                    &[],
                    first_skill_offset_time,
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
                    let simulated_events = extract_skill_simulation_event(
                        skill,
                        first_skill_offset_time,
                        information_needed.player_id,
                    );

                    let second_skill_start_time = first_skill_start_time + skill_delay_millisecond;

                    if second_skill_start_time > next_gcd_millisecond - NON_GCD_DELAY_MILLISECOND {
                        continue;
                    }

                    first_skill_offset_time += skill_delay_millisecond;
                    return if let Some(second_skill_plan) = self.find_second_highest_ogcd_skill(
                        information_needed,
                        &ogcd_priority_table,
                        &simulated_events,
                        turn_info,
                        second_skill_start_time,
                        skill.get_id(),
                        first_skill_offset_time,
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
                            best_one_ogcd
                        } else {
                            double_weave_plan
                        }
                    } else {
                        best_one_ogcd
                    };
                }
            }
        }

        best_one_ogcd
    }

    fn find_second_highest_ogcd_skill(
        &self,
        information_needed: InformationNeededForRotationDecision,
        ogcd_priority_table: &[SkillPriorityInfo],
        simulation_events: &[SkillSimulationEvent],
        turn_info: &TurnInfo,
        start_time: TimeType,
        first_used_skill_id: SkillIdType,
        time_offset: TimeType,
    ) -> Option<OgcdPlan> {
        let next_gcd_millisecond = turn_info.next_gcd_millisecond;

        for (priority_number, skill_priority) in ogcd_priority_table.iter().enumerate() {
            let skill = information_needed
                .combat_resources
                .get_skill(skill_priority.skill_id);

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

            let skill_start_time = start_time + max(skill_cooldown - time_offset, 0);
            let time_offset = time_offset + skill_cooldown;

            if skill_start_time <= latest_time_to_use {
                if self.can_use_skill(
                    information_needed,
                    skill_priority,
                    simulation_events,
                    time_offset,
                ) {
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
        information_needed: InformationNeededForRotationDecision,
    ) -> Vec<SkillUsageInfo> {
        let gcd_priority_table = self.get_gcd_priority_table();

        for skill_priority in gcd_priority_table {
            let empty_array = [];
            if self.can_use_skill(information_needed, skill_priority, &empty_array, 0) {
                return vec![SkillUsageInfo::new(skill_priority.skill_id)];
            }
        }

        return vec![];
    }

    fn can_use_skill(
        &self,
        information_needed: InformationNeededForRotationDecision,
        skill_priority: &SkillPriorityInfo,
        simulation_events: &[SkillSimulationEvent],
        time_offset: TimeType,
    ) -> bool {
        let skill = information_needed
            .combat_resources
            .get_skill(skill_priority.skill_id);
        let stack_skill = if let Some(stack_skill_id) = skill.stack_skill_id {
            information_needed
                .combat_resources
                .get_skill(stack_skill_id)
        } else {
            skill
        };

        if stack_skill.stack_in_future(simulation_events, time_offset) == 0
            || !self.meets_requirements(information_needed, simulation_events, skill, time_offset)
        {
            return false;
        }

        if skill_priority.prerequisite.is_none() {
            return true;
        }

        let prerequisite = skill_priority.prerequisite.as_ref().unwrap();
        self.meets_prequisite(
            &prerequisite,
            information_needed,
            simulation_events,
            skill,
            time_offset,
        )
    }

    fn get_gcd_priority_table(&self) -> &[SkillPriorityInfo];
    fn get_ogcd_priority_table(&self) -> &[SkillPriorityInfo];

    fn meets_requirements(
        &self,
        information_needed: InformationNeededForRotationDecision,
        simulation_events: &[SkillSimulationEvent],
        skill: &AttackSkill,
        time_offset: TimeType,
    ) -> bool {
        for resource_required in &skill.resource_required {
            match resource_required {
                ResourceRequirements::Resource(id, resource) => {
                    let simulated_resource = simulate_resources(
                        information_needed.combat_resources,
                        &simulation_events,
                        *id,
                    );

                    if simulated_resource < *resource {
                        return false;
                    }
                }
                ResourceRequirements::UseBuff(status_id)
                | ResourceRequirements::UseDebuff(status_id)
                | ResourceRequirements::CheckStatus(status_id) => {
                    if !self.has_status(
                        information_needed,
                        simulation_events,
                        time_offset,
                        *status_id,
                    ) {
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
        information_needed: InformationNeededForRotationDecision,
        simulation_events: &[SkillSimulationEvent],
        skill: &AttackSkill,
        time_offset: TimeType,
    ) -> bool {
        match prerequisite {
            SkillPrerequisite::Or(left, right) => {
                self.meets_prequisite(
                    left,
                    information_needed,
                    simulation_events,
                    skill,
                    time_offset,
                ) || self.meets_prequisite(
                    right,
                    information_needed,
                    simulation_events,
                    skill,
                    time_offset,
                )
            }
            SkillPrerequisite::And(left, right) => {
                self.meets_prequisite(
                    left,
                    information_needed,
                    simulation_events,
                    skill,
                    time_offset,
                ) && self.meets_prequisite(
                    right,
                    information_needed,
                    simulation_events,
                    skill,
                    time_offset,
                )
            }
            SkillPrerequisite::Not(prerequisite) => !self.meets_prequisite(
                prerequisite,
                information_needed,
                simulation_events,
                skill,
                time_offset,
            ),
            SkillPrerequisite::Combo(combo_id) => {
                for simulation_event in simulation_events {
                    match simulation_event {
                        SkillSimulationEvent::UpdateCombo(update_combo_id) => {
                            return if let Some(combo_id) = combo_id {
                                *update_combo_id == *combo_id
                            } else {
                                false
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(current_combo_id) =
                    information_needed.combat_resources.get_current_combo()
                {
                    if let Some(combo_id) = combo_id {
                        current_combo_id == *combo_id
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            SkillPrerequisite::HasBufforDebuff(status_id) => self.has_status(
                information_needed,
                simulation_events,
                time_offset,
                *status_id,
            ),
            SkillPrerequisite::BufforDebuffLessThan(status_id, time_millisecond) => {
                let mut buff_remaining_time = if let Some(buff) = information_needed
                    .buffs
                    .get(&StatusKey::new(*status_id, information_needed.player_id))
                {
                    buff.duration_left_millisecond
                } else if let Some(debuff) = information_needed
                    .debuffs
                    .get(&StatusKey::new(*status_id, information_needed.player_id))
                {
                    debuff.duration_left_millisecond
                } else {
                    0
                };

                buff_remaining_time = max(buff_remaining_time - time_offset, 0);

                for simulated_event in simulation_events {
                    match simulated_event {
                        SkillSimulationEvent::AddBuff(buff_id, duration, max_duration, refresh) => {
                            if *buff_id == *status_id && *refresh {
                                buff_remaining_time =
                                    min(buff_remaining_time + duration, *max_duration);
                            }
                        }
                        SkillSimulationEvent::AddDebuff(
                            debuff_id,
                            duration,
                            max_duration,
                            refresh,
                        ) => {
                            if *debuff_id == *status_id && *refresh {
                                buff_remaining_time =
                                    max(buff_remaining_time + duration, *max_duration);
                            }
                        }
                        _ => {}
                    }
                }

                buff_remaining_time < *time_millisecond
            }
            SkillPrerequisite::HasResource(resource_id, resource) => {
                let simulated_resource = simulate_resources(
                    information_needed.combat_resources,
                    &simulation_events,
                    *resource_id,
                );

                simulated_resource >= *resource
            }
            SkillPrerequisite::HasResourceExactly(resource_id, resource) => {
                let simulated_resource = simulate_resources(
                    information_needed.combat_resources,
                    &simulation_events,
                    *resource_id,
                );

                simulated_resource == *resource
            }
            SkillPrerequisite::MillisecondsBeforeBurst(milliseconds) => {
                *milliseconds >= information_needed.milliseconds_before_burst
            }
            SkillPrerequisite::HasSkillStacks(skill_id, stacks) => {
                information_needed
                    .combat_resources
                    .get_skill(*skill_id)
                    .stack_in_future(simulation_events, time_offset)
                    >= *stacks
            }
            SkillPrerequisite::RelatedSkillCooldownLessOrEqualThan(
                related_skill_id,
                time_millisecond,
            ) => {
                let mut related_skill_cooldown = information_needed
                    .combat_resources
                    .get_skill(*related_skill_id)
                    .current_cooldown_millisecond;

                for simulation_event in simulation_events {
                    match simulation_event {
                        SkillSimulationEvent::ReduceCooldown(skill_id, amount) => {
                            if *skill_id == *related_skill_id {
                                related_skill_cooldown = max(related_skill_cooldown - *amount, 0);
                            }
                        }
                        _ => {}
                    }
                }

                max(related_skill_cooldown - time_offset, 0) <= *time_millisecond
            }
            SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(
                greater_resource_id,
                lesser_resource_id,
                amount,
            ) => {
                let mut greater_resource = information_needed
                    .combat_resources
                    .get_resource(*greater_resource_id);
                let mut lesser_resource = information_needed
                    .combat_resources
                    .get_resource(*lesser_resource_id);

                for simulation_event in simulation_events {
                    match simulation_event {
                        SkillSimulationEvent::AddResource(resource_id, amount) => {
                            // TODO: add max resource API?
                            if *resource_id == *greater_resource_id {
                                greater_resource += amount;
                            } else if *resource_id == *lesser_resource_id {
                                lesser_resource += amount;
                            }
                        }

                        SkillSimulationEvent::UseResource(resource_id, amount) => {
                            if *resource_id == *greater_resource_id {
                                greater_resource = min(greater_resource - amount, 0);
                            } else if *resource_id == *lesser_resource_id {
                                lesser_resource = min(lesser_resource - amount, 0);
                            }
                        }
                        _ => {}
                    }
                }

                greater_resource >= lesser_resource + *amount
            }
            SkillPrerequisite::BuffGreaterDurationThan(buff1_id, buff2_id) => {
                let buff1_search = information_needed
                    .buffs
                    .get(&StatusKey::new(*buff1_id, information_needed.player_id));
                let buff2_search = information_needed
                    .buffs
                    .get(&StatusKey::new(*buff2_id, information_needed.player_id));

                if let Some(buff1_remaining_time) = buff1_search {
                    if let Some(buff2_remaining_time) = buff2_search {
                        let mut buff1_remaining_time =
                            buff1_remaining_time.duration_left_millisecond;
                        let mut buff2_remaining_time =
                            buff2_remaining_time.duration_left_millisecond;

                        buff1_remaining_time = max(buff1_remaining_time - time_offset, 0);
                        buff2_remaining_time = max(buff2_remaining_time - time_offset, 0);

                        for simulated_event in simulation_events {
                            match simulated_event {
                                SkillSimulationEvent::AddBuff(
                                    buff_id,
                                    duration,
                                    max_duration,
                                    refresh,
                                ) => {
                                    if *buff_id == *buff1_id && *refresh {
                                        buff1_remaining_time =
                                            min(buff1_remaining_time + duration, *max_duration);
                                    }

                                    if *buff_id == *buff2_id && *refresh {
                                        buff2_remaining_time =
                                            min(buff2_remaining_time + duration, *max_duration);
                                    }
                                }
                                _ => {}
                            }
                        }

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

    fn has_status(
        &self,
        information_needed: InformationNeededForRotationDecision,
        simulation_events: &[SkillSimulationEvent],
        time_offset: TimeType,
        status_id: StatusIdType,
    ) -> bool {
        let key = StatusKey::new(status_id, information_needed.player_id);

        for simulation_event in simulation_events {
            match simulation_event {
                SkillSimulationEvent::AddBuff(buff_id, _, _, _) => {
                    if *buff_id == status_id {
                        return true;
                    }
                }
                SkillSimulationEvent::AddDebuff(debuff_id, _, _, _) => {
                    if *debuff_id == status_id {
                        return true;
                    }
                }
                SkillSimulationEvent::RemoveBuff(buff_id) => {
                    if *buff_id == status_id {
                        return false;
                    }
                }
                SkillSimulationEvent::RemoveDebuff(debuff_id) => {
                    if *debuff_id == status_id {
                        return false;
                    }
                }

                _ => {}
            }
        }

        if let Some(buff) = information_needed.buffs.get(&key) {
            return buff.duration_left_millisecond > time_offset;
        }

        if let Some(debuff) = information_needed.debuffs.get(&key) {
            return debuff.duration_left_millisecond > time_offset;
        }

        false
    }

    fn increment_turn(&self);
    fn get_turn_count(&self) -> SkillIdType;
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
