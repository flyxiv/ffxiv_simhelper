use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::ResourceRequirements;
use crate::types::{PlayerIdType, ResourceIdType, ResourceType, StatusIdType, TimeType};

const SKILL_SIMULATION_EVENT_CAPACITY: usize = 5;

/// Keeps track of "future events" that affect the status of the player
/// that will happen in the future if the skill does get used.
pub(crate) enum SkillSimulationEvent {
    /// status id, duration, max duration, refresh
    AddBuff(StatusIdType, TimeType, TimeType, bool),
    AddDebuff(StatusIdType, TimeType, TimeType, bool),
    RemoveBuff(StatusIdType),
    RemoveDebuff(StatusIdType),
    AddResource(ResourceIdType, ResourceType),
    UseResource(ResourceIdType, ResourceType),
    ReduceCooldown(StatusIdType, TimeType),
    UpdateCombo(u8),
}

/// Get skill events that affect the player statuses which will affect the
/// rotation decision of the jobs(ex) apply or delete buff, reduce skill cooldown, increase resource, ...)  
pub(crate) fn extract_skill_simulation_event(
    skill: &AttackSkill,
    time_offset: TimeType,
    player_id: PlayerIdType,
) -> Vec<SkillSimulationEvent> {
    let mut events = Vec::with_capacity(SKILL_SIMULATION_EVENT_CAPACITY);

    for ffxiv_event in &skill.additional_skill_events {
        match ffxiv_event {
            FfxivEvent::ApplyBuff(
                _,
                target_player_id,
                buff,
                duration,
                max_duration,
                apply_time,
            ) => {
                if time_offset >= *apply_time && *target_player_id == player_id {
                    events.push(SkillSimulationEvent::AddBuff(
                        buff.get_id(),
                        *duration,
                        *max_duration,
                        true,
                    ))
                }
            }
            FfxivEvent::ApplyBuffStack(
                _,
                target_player_id,
                buff,
                duration,
                refresh,
                apply_time,
            ) => {
                if time_offset >= *apply_time && *target_player_id == player_id {
                    events.push(SkillSimulationEvent::AddBuff(
                        buff.get_id(),
                        *duration,
                        *duration,
                        *refresh,
                    ))
                }
            }
            FfxivEvent::ApplyDebuff(_, debuff, duration, max_duration, apply_time) => {
                if time_offset >= *apply_time {
                    events.push(SkillSimulationEvent::AddDebuff(
                        debuff.get_id(),
                        *duration,
                        *max_duration,
                        true,
                    ))
                }
            }
            FfxivEvent::RemoveTargetBuff(_, target_player_id, buff_id, apply_time) => {
                if *target_player_id == player_id && time_offset >= *apply_time {
                    events.push(SkillSimulationEvent::RemoveBuff(*buff_id))
                }
            }
            FfxivEvent::RemoveDebuff(_, debuff_id, apply_time) => {
                if time_offset >= *apply_time {
                    events.push(SkillSimulationEvent::RemoveDebuff(*debuff_id))
                }
            }

            FfxivEvent::IncreasePlayerResource(
                target_player_id,
                resource_id,
                increase_amount,
                apply_time,
            ) => {
                if time_offset >= *apply_time && *target_player_id == player_id {
                    events.push(SkillSimulationEvent::UseResource(
                        *resource_id,
                        *increase_amount,
                    ))
                }
            }

            FfxivEvent::ApplyRaidBuff(_, buff, duration, max_duration, apply_time) => {
                if time_offset >= *apply_time {
                    events.push(SkillSimulationEvent::AddBuff(
                        buff.get_id(),
                        *duration,
                        *max_duration,
                        true,
                    ))
                }
            }

            FfxivEvent::RefreshBuff(
                _,
                target_player_id,
                buff,
                duration,
                max_duration,
                apply_time,
            ) => {
                if time_offset >= *apply_time && *target_player_id == player_id {
                    events.push(SkillSimulationEvent::AddBuff(
                        buff.get_id(),
                        *duration,
                        *max_duration,
                        true,
                    ))
                }
            }

            FfxivEvent::ReduceSkillCooldown(_, skill_id, reduce_amount, apply_time) => {
                if time_offset >= *apply_time {
                    events.push(SkillSimulationEvent::ReduceCooldown(
                        *skill_id,
                        *reduce_amount,
                    ))
                }
            }

            FfxivEvent::RemoveRaidBuff(_, buff_id, apply_time) => {
                if time_offset >= *apply_time {
                    events.push(SkillSimulationEvent::RemoveBuff(*buff_id))
                }
            }

            FfxivEvent::ApplyDebuffStack(_, debuff, duration, refresh, apply_time) => {
                if time_offset >= *apply_time && *refresh {
                    events.push(SkillSimulationEvent::AddDebuff(
                        debuff.get_id(),
                        *duration,
                        *duration,
                        *refresh,
                    ))
                }
            }

            _ => {}
        }
    }

    for (&resource_id, &resource_amount) in &skill.resource_created {
        events.push(SkillSimulationEvent::AddResource(
            resource_id,
            resource_amount,
        ));
    }

    for resource_event in &skill.resource_required {
        match resource_event {
            ResourceRequirements::Resource(resource_id, resource_amount) => {
                events.push(SkillSimulationEvent::UseResource(
                    *resource_id,
                    *resource_amount,
                ));
            }

            ResourceRequirements::UseBuff(buff_id) => {
                events.push(SkillSimulationEvent::RemoveBuff(*buff_id));
            }

            ResourceRequirements::UseDebuff(debuff_id) => {
                events.push(SkillSimulationEvent::RemoveDebuff(*debuff_id));
            }

            _ => {}
        }
    }

    if let Some(combo) = skill.combo {
        events.push(SkillSimulationEvent::UpdateCombo(combo));
    }

    events
}

/// Extract the skill internal events that could affect the decision for the next skill,
/// such as resource increase or decrease.
pub(crate) fn simulate_resources(
    combat_resources: &FfxivCombatResources,
    skill_simulation_events: &[SkillSimulationEvent],
    target_resource_id: ResourceIdType,
) -> ResourceType {
    let mut resource = combat_resources.get_resource(target_resource_id);

    skill_simulation_events.iter().for_each(
        |skill_simulation_event| match skill_simulation_event {
            SkillSimulationEvent::AddResource(resource_id, resource_amount) => {
                if *resource_id == target_resource_id {
                    resource += *resource_amount;
                }
            }

            SkillSimulationEvent::UseResource(resource_id, resource_amount) => {
                if *resource_id == target_resource_id {
                    resource -= *resource_amount;
                }
            }

            _ => {}
        },
    );

    resource
}
