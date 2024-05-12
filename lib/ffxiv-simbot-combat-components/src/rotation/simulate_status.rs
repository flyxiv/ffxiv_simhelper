use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::rotation::priority_table::CombatInfo;
use crate::rotation::simulated_combat_resource::FirstSkillCombatSimulation;
use crate::skill::ResourceRequirements;
use crate::status::Status;
use std::cmp::min;

pub(crate) fn simulate_status(
    combat_info: &mut CombatInfo,
    combat_resource: &mut Box<dyn CombatResource>,
    simulation: &FirstSkillCombatSimulation,
) {
    for resource_required in simulation.skill_used_resource.iter() {
        match resource_required {
            ResourceRequirements::UseBuff(status_id) => {
                let key = StatusKey::new(*status_id, *simulation.player_id);
                let mut buff_list = combat_info.buff_list.borrow_mut();
                let mut delete = false;

                if let Some(buff) = buff_list.get_mut(&key) {
                    buff.stacks -= 1;
                    delete = buff.stacks == 0;
                } else {
                    delete = true;
                }

                if delete {
                    buff_list.borrow_mut().remove(&key);
                }
            }
            ResourceRequirements::UseDebuff(status_id) => {
                let key = StatusKey::new(*status_id, *simulation.player_id);
                let mut debuff_list = combat_info.debuff_list.borrow_mut();
                let mut delete = false;

                if let Some(debuff) = debuff_list.get_mut(&key) {
                    debuff.stacks -= 1;
                    delete = debuff.stacks == 0;
                } else {
                    delete = true;
                }

                if delete {
                    debuff_list.borrow_mut().remove(&key);
                }
            }
            ResourceRequirements::Resource(id, stack) => {
                combat_resource.use_resource(*id, *stack);
            }
            _ => {}
        }
    }

    if let Some(buff_events) = simulation.skill_buffs {
        for event in buff_events.iter() {
            match event {
                FfxivEvent::ApplyBuff(
                    _,
                    target_player_id,
                    buff_status,
                    duration,
                    max_duration,
                    _,
                ) => {
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
                FfxivEvent::ApplyBuffStack(
                    _,
                    target_player_id,
                    buff_status,
                    duration,
                    refresh,
                    _,
                ) => {
                    let key = StatusKey::new(buff_status.get_id(), simulation.player_id);
                    let mut buff_status = buff_status.clone();

                    if *target_player_id == simulation.player_id {
                        let new_stacks =
                            if let Some(buff) = combat_info.buff_list.borrow().get(&key) {
                                let duration_remaining = buff.get_duration_left_millisecond();
                                let refreshed_duration = if refresh {
                                    *duration
                                } else {
                                    duration_remaining
                                };

                                let mut stack = buff_status.stacks.unwrap();
                                (refreshed_duration, stack + 1)
                            } else {
                                (*duration, 1)
                            };

                        buff_status.duration_left_millisecond = new_stacks.0;
                        buff_status.stacks = Some(new_stacks.1);

                        combat_info.buff_list.borrow_mut().insert(key, buff_status);
                    }
                }
                FfxivEvent::ApplyRaidBuff(player_id, buff_status, duration, max_duration, _) => {
                    let key = StatusKey::new(buff_status.get_id(), *player_id);
                    let buff_status = buff_status.clone();
                    let refreshed_duration =
                        if let Some(buff) = combat_info.buff_list.borrow().get(&key) {
                            let duration_remaining = buff.get_duration_left_millisecond();
                            min(duration_remaining + duration, *max_duration)
                        } else {
                            *duration
                        };
                    combat_info.buff_list.borrow_mut().insert(key, buff_status);
                }
                _ => {}
            }
        }
    }

    if let Some(debuff_events) = simulation.skill_debuffs {
        for event in debuff_events.iter() {
            match event {
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

                    let new_stacks =
                        if let Some(debuff) = combat_info.debuff_list.borrow().get(&key) {
                            let duration_remaining = debuff.get_duration_left_millisecond();
                            let refreshed_duration = if refresh {
                                *duration
                            } else {
                                duration_remaining
                            };

                            let mut stack = debuff_status.stacks.unwrap();
                            (refreshed_duration, stack + 1)
                        } else {
                            (*duration, 1)
                        };

                    debuff_status.duration_left_millisecond = new_stacks.0;
                    debuff_status.stacks = Some(new_stacks.1);

                    combat_info
                        .debuff_list
                        .borrow_mut()
                        .insert(key, debuff_status);
                }

                _ => {}
            }
        }
    }

    combat_resource.add_resources(simulation.resource_produced);
}
