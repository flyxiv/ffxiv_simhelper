use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::rotation::priority_simulation_data::{
    PriorityDecisionTable, TruncatedBuffStatus, TruncatedDebuffStatus,
};
use crate::rotation::simulated_combat_resource::FirstSkillCombatSimulation;
use crate::skill::ResourceRequirements;
use crate::types::SkillIdType;
use std::cmp::min;

pub(crate) fn simulate_status(
    combat_info: &mut PriorityDecisionTable,
    simulation: &FirstSkillCombatSimulation,
    skill_id: SkillIdType,
) {
    let skill_table = &mut combat_info.skill_list;
    let skill = skill_table.get_mut(&skill_id).unwrap();
    skill.current_cooldown_millisecond += skill.cooldown_millisecond;

    for resource_required in simulation.skill_used_resource.iter() {
        match resource_required {
            ResourceRequirements::UseBuff(status_id) => {
                let key = StatusKey::new(*status_id, simulation.player_id);
                let mut delete = true;

                if let Some(buff) = combat_info.buff_list.get_mut(&key) {
                    buff.stacks -= 1;
                    delete = buff.stacks == 0;
                }

                if delete {
                    combat_info.buff_list.remove(&key);
                }
            }
            ResourceRequirements::UseDebuff(status_id) => {
                let key = StatusKey::new(*status_id, simulation.player_id);
                let mut delete = true;

                if let Some(debuff) = combat_info.debuff_list.get_mut(&key) {
                    debuff.stacks -= 1;
                    delete = debuff.stacks == 0;
                }

                if delete {
                    combat_info.debuff_list.remove(&key);
                }
            }
            ResourceRequirements::Resource(id, resource) => {
                combat_info.use_resource(*id, *resource);
            }
            _ => {}
        }
    }

    for event in simulation.skill_events {
        match event {
            FfxivEvent::ApplyBuff(_, target_player_id, buff_status, duration, max_duration, _) => {
                let key = StatusKey::new(buff_status.get_id(), simulation.player_id);
                let mut buff_status = TruncatedBuffStatus::from(buff_status);

                if *target_player_id == simulation.player_id {
                    let refreshed_duration = if let Some(buff) = combat_info.buff_list.get(&key) {
                        let duration_remaining = buff.duration_left_millisecond;
                        min(duration_remaining + duration, *max_duration)
                    } else {
                        *duration
                    };
                    buff_status.duration_left_millisecond = refreshed_duration;

                    combat_info.buff_list.insert(key, buff_status);
                }
            }
            FfxivEvent::ApplyBuffStack(_, target_player_id, buff_status, duration, refresh, _) => {
                let key = StatusKey::new(buff_status.get_id(), simulation.player_id);
                let mut buff_status = TruncatedBuffStatus::from(buff_status);

                if *target_player_id == simulation.player_id {
                    let new_stacks = if let Some(buff) = combat_info.buff_list.get(&key) {
                        let duration_remaining = buff.duration_left_millisecond;
                        let refreshed_duration = if *refresh {
                            *duration
                        } else {
                            duration_remaining
                        };

                        let stack = buff_status.stacks;
                        (refreshed_duration, stack + 1)
                    } else {
                        (*duration, 1)
                    };

                    buff_status.duration_left_millisecond = new_stacks.0;
                    buff_status.stacks = new_stacks.1;

                    combat_info.buff_list.insert(key, buff_status);
                }
            }
            FfxivEvent::ApplyRaidBuff(player_id, buff_status, duration, max_duration, _) => {
                let key = StatusKey::new(buff_status.get_id(), *player_id);
                let mut buff_status = TruncatedBuffStatus::from(buff_status);
                let refreshed_duration = if let Some(buff) = combat_info.buff_list.get(&key) {
                    let duration_remaining = buff.duration_left_millisecond;
                    min(duration_remaining + duration, *max_duration)
                } else {
                    *duration
                };
                buff_status.duration_left_millisecond = refreshed_duration;
                combat_info.buff_list.insert(key, buff_status);
            }
            FfxivEvent::ApplyDebuff(_, debuff_status, duration, max_duration, _) => {
                let key = StatusKey::new(debuff_status.get_id(), simulation.player_id);
                let mut debuff_status = TruncatedDebuffStatus::from(debuff_status);

                let refreshed_duration =
                    if let Some(debuff) = combat_info.debuff_list.get(&key).clone() {
                        let duration_remaining = debuff.duration_left_millisecond;
                        min(duration_remaining + duration, *max_duration)
                    } else {
                        *duration
                    };
                debuff_status.duration_left_millisecond = refreshed_duration;

                combat_info.debuff_list.insert(key, debuff_status);
            }
            FfxivEvent::ApplyDebuffStack(_, debuff_status, duration, refresh, _) => {
                let mut debuff_status = TruncatedDebuffStatus::from(debuff_status);
                let key = StatusKey::new(debuff_status.id, simulation.player_id);

                let new_stacks = if let Some(debuff) = combat_info.debuff_list.get(&key) {
                    let duration_remaining = debuff.duration_left_millisecond;
                    let refreshed_duration = if *refresh {
                        *duration
                    } else {
                        duration_remaining
                    };

                    let stack = debuff_status.stacks;
                    (refreshed_duration, stack + 1)
                } else {
                    (*duration, 1)
                };

                debuff_status.duration_left_millisecond = new_stacks.0;
                debuff_status.stacks = new_stacks.1;

                combat_info.debuff_list.insert(key, debuff_status);
            }

            _ => {}
        }
    }

    for (&resource_id, &amount) in simulation.resource_produced.iter() {
        combat_info.add_resource(resource_id, amount);
    }
}
