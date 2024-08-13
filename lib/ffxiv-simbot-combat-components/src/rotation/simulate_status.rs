use crate::combat_resources::ffxiv_combat_resources::FfxivCombatResources;
use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::rotation::priority_table::CombatInfo;
use crate::rotation::simulated_combat_resource::FirstSkillCombatSimulation;
use crate::skill::ResourceRequirements;
use crate::status::Status;
use crate::types::IdType;
use std::cmp::min;

pub(crate) fn simulate_status(
    combat_info: &mut CombatInfo,
    combat_resource: &mut FfxivCombatResources,
    simulation: &FirstSkillCombatSimulation,
    skill_id: IdType,
) {
    let skill_table = combat_resource.get_skills_mut();
    let skill = skill_table.get_mut(&skill_id).unwrap();
    skill.current_cooldown_millisecond += skill.cooldown_millisecond;

    for resource_required in simulation.skill_used_resource.iter() {
        match resource_required {
            ResourceRequirements::UseBuff(status_id) => {
                let key = StatusKey::new(*status_id, simulation.player_id);
                let mut buff_list = combat_info.buff_list.borrow_mut();
                let mut delete = true;

                if let Some(buff) = buff_list.get_mut(&key) {
                    buff.stacks -= 1;
                    delete = buff.stacks == 0;
                }

                if delete {
                    buff_list.remove(&key);
                }
            }
            ResourceRequirements::UseDebuff(status_id) => {
                let key = StatusKey::new(*status_id, simulation.player_id);
                let mut debuff_list = combat_info.debuff_list.borrow_mut();
                let mut delete = true;

                if let Some(debuff) = debuff_list.get_mut(&key) {
                    debuff.stacks -= 1;
                    delete = debuff.stacks == 0;
                }

                if delete {
                    debuff_list.remove(&key);
                }
            }
            ResourceRequirements::Resource(id, stack) => {
                combat_resource.use_resource(*id, *stack);
            }
            _ => {}
        }
    }

    for event in simulation.skill_events {
        match event {
            FfxivEvent::ApplyBuff(_, target_player_id, buff_status, duration, max_duration, _) => {
                let key = StatusKey::new(buff_status.get_id(), simulation.player_id);
                let mut buff_status = buff_status.clone();

                if *target_player_id == simulation.player_id {
                    let refreshed_duration =
                        if let Some(buff) = combat_info.buff_list.borrow().get(&key).clone() {
                            let duration_remaining = buff.get_duration_left_millisecond();
                            min(duration_remaining + duration, *max_duration)
                        } else {
                            *duration
                        };
                    buff_status.duration_left_millisecond = refreshed_duration;

                    combat_info.buff_list.borrow_mut().insert(key, buff_status);
                }
            }
            FfxivEvent::ApplyBuffStack(_, target_player_id, buff_status, duration, refresh, _) => {
                let key = StatusKey::new(buff_status.get_id(), simulation.player_id);
                let mut buff_status = buff_status.clone();

                if *target_player_id == simulation.player_id {
                    let new_stacks = if let Some(buff) = combat_info.buff_list.borrow().get(&key) {
                        let duration_remaining = buff.get_duration_left_millisecond();
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

                    combat_info.buff_list.borrow_mut().insert(key, buff_status);
                }
            }
            FfxivEvent::ApplyRaidBuff(player_id, buff_status, duration, max_duration, _) => {
                let key = StatusKey::new(buff_status.get_id(), *player_id);
                let mut buff_status = buff_status.clone();
                let refreshed_duration =
                    if let Some(buff) = combat_info.buff_list.borrow().get(&key) {
                        let duration_remaining = buff.get_duration_left_millisecond();
                        min(duration_remaining + duration, *max_duration)
                    } else {
                        *duration
                    };
                buff_status.duration_left_millisecond = refreshed_duration;
                combat_info.buff_list.borrow_mut().insert(key, buff_status);
            }
            FfxivEvent::ApplyDebuff(_, debuff_status, duration, max_duration, _) => {
                let key = StatusKey::new(debuff_status.get_id(), simulation.player_id);
                let mut debuff_status = debuff_status.clone();

                let refreshed_duration =
                    if let Some(debuff) = combat_info.debuff_list.borrow().get(&key).clone() {
                        let duration_remaining = debuff.get_duration_left_millisecond();
                        min(duration_remaining + duration, *max_duration)
                    } else {
                        *duration
                    };
                debuff_status.duration_left_millisecond = refreshed_duration;

                combat_info
                    .debuff_list
                    .borrow_mut()
                    .insert(key, debuff_status);
            }
            FfxivEvent::ApplyDebuffStack(_, debuff_status, duration, refresh, _) => {
                let key = StatusKey::new(debuff_status.get_id(), simulation.player_id);
                let mut debuff_status = debuff_status.clone();

                let new_stacks = if let Some(debuff) = combat_info.debuff_list.borrow().get(&key) {
                    let duration_remaining = debuff.get_duration_left_millisecond();
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

                combat_info
                    .debuff_list
                    .borrow_mut()
                    .insert(key, debuff_status);
            }

            _ => {}
        }
    }

    for (resource_id, amount) in simulation.resource_produced.iter() {
        combat_resource.add_resource(*resource_id, *amount);
    }
}
