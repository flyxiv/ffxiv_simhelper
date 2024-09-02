use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::combat_resources::CombatResource;
use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{ResourceRequirements, NON_GCD_DELAY_MILLISECOND};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::Status;
use crate::types::{ComboType, ResourceIdType, ResourceType, StackType, StatusIdType, TimeType};
use std::cmp::max;
use std::collections::HashMap;

const MAX_RESOURCE: usize = 15;
pub(crate) const EMPTY_RESOURCE: ResourceType = -100;

#[derive(Clone, Copy)]
pub(crate) struct TruncatedBuffStatus {
    pub(crate) id: StatusIdType,
    pub(crate) stacks: StackType,
    pub(crate) duration_left_millisecond: TimeType,
}

#[derive(Clone, Copy)]
pub(crate) struct TruncatedDebuffStatus {
    pub(crate) id: StatusIdType,
    pub(crate) stacks: StackType,
    pub(crate) duration_left_millisecond: TimeType,
}

#[derive(Clone)]
pub(crate) struct TruncatedAttackSkill {
    pub(crate) id: StatusIdType,
    pub(crate) stacks: StackType,
    pub(crate) current_cooldown_millisecond: TimeType,
    pub(crate) cooldown_millisecond: TimeType,
    pub(crate) resource_required: Vec<ResourceRequirements>,
    pub(crate) delay_millisecond: Option<TimeType>,
}

impl From<&BuffStatus> for TruncatedBuffStatus {
    fn from(buff: &BuffStatus) -> TruncatedBuffStatus {
        TruncatedBuffStatus {
            id: buff.get_id(),
            stacks: buff.stacks,
            duration_left_millisecond: buff.get_duration_left_millisecond(),
        }
    }
}

impl From<&DebuffStatus> for TruncatedDebuffStatus {
    fn from(debuff: &DebuffStatus) -> TruncatedDebuffStatus {
        TruncatedDebuffStatus {
            id: debuff.get_id(),
            stacks: debuff.stacks,
            duration_left_millisecond: debuff.get_duration_left_millisecond(),
        }
    }
}

impl From<&AttackSkill> for TruncatedAttackSkill {
    fn from(skill: &AttackSkill) -> TruncatedAttackSkill {
        TruncatedAttackSkill {
            id: skill.get_id(),
            stacks: skill.stacks,
            current_cooldown_millisecond: skill.current_cooldown_millisecond,
            cooldown_millisecond: skill.cooldown_millisecond,
            resource_required: skill.resource_required.clone(),
            delay_millisecond: skill.delay_millisecond,
        }
    }
}

#[derive(Clone)]
pub(crate) struct PriorityDecisionTable {
    pub(crate) buff_list: HashMap<StatusKey, TruncatedBuffStatus>,
    pub(crate) debuff_list: HashMap<StatusKey, TruncatedDebuffStatus>,
    pub(crate) skill_list: HashMap<StatusIdType, TruncatedAttackSkill>,
    pub(crate) combo: ComboType,
    pub(crate) milliseconds_before_burst: TimeType,
    pub(crate) resources: Vec<ResourceType>,
}

pub(crate) fn to_priority_decision_table(
    buff_list: HashMap<StatusKey, BuffStatus>,
    debuff_list: HashMap<StatusKey, DebuffStatus>,
    milliseconds_before_burst: TimeType,
    combat_resources: &FfxivCombatResources,
) -> PriorityDecisionTable {
    let buff_list: HashMap<StatusKey, TruncatedBuffStatus> = buff_list
        .into_iter()
        .map(|(status_key, buff_status)| (status_key, TruncatedBuffStatus::from(&buff_status)))
        .collect();

    let debuff_list: HashMap<StatusKey, TruncatedDebuffStatus> = debuff_list
        .into_iter()
        .map(|(status_key, debuff_status)| {
            (status_key, TruncatedDebuffStatus::from(&debuff_status))
        })
        .collect();

    let skill_list: HashMap<StatusIdType, TruncatedAttackSkill> = combat_resources
        .get_skills()
        .iter()
        .map(|(&status_id, skill)| (status_id, TruncatedAttackSkill::from(skill)))
        .collect();

    let mut resources = Vec::with_capacity(MAX_RESOURCE);
    let mut resource_id = 0;

    while resource_id > EMPTY_RESOURCE {
        let resource_value = combat_resources.get_resource(resource_id as ResourceIdType);

        if resource_value >= -5 {
            resources.push(resource_value);
            resource_id += 1;
        } else {
            break;
        }
    }

    PriorityDecisionTable {
        buff_list,
        debuff_list,
        skill_list,
        combo: combat_resources.get_current_combo(),
        milliseconds_before_burst,
        resources,
    }
}

impl PriorityDecisionTable {
    pub(crate) fn use_resource(&mut self, resource_id: ResourceIdType, amount: ResourceType) {
        let new_resource = self.resources[resource_id as usize] - amount;
        self.resources[resource_id as usize] = max(new_resource, 0);
    }

    pub(crate) fn add_resource(&mut self, resource_id: ResourceIdType, amount: ResourceType) {
        self.resources[resource_id as usize] += amount;
    }

    pub(crate) fn update_cooldown(&mut self, elapsed_time: TimeType) {
        for (_, skill) in self.skill_list.iter_mut() {
            skill.update_cooldown(elapsed_time);
        }
    }

    pub(crate) fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        self.resources[resource_id as usize]
    }
}

impl TruncatedAttackSkill {
    fn update_cooldown(&mut self, elapsed_time: TimeType) {
        if self.current_cooldown_millisecond <= 0 || elapsed_time <= 0 {
            return;
        }

        let past_stack = self.get_stack();
        self.current_cooldown_millisecond =
            max(0, self.current_cooldown_millisecond - elapsed_time);

        let current_stack = self.get_stack();

        if past_stack != current_stack {
            self.stacks += 1;
        }
    }

    #[inline]
    fn get_stack(&self) -> StackType {
        f64::ceil(self.current_cooldown_millisecond as f64 / self.cooldown_millisecond as f64)
            as StackType
    }

    pub(crate) fn get_delay_millisecond(&self) -> TimeType {
        self.delay_millisecond.unwrap_or(NON_GCD_DELAY_MILLISECOND)
    }
}
