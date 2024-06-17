use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_event::FfxivEvent::Damage;
use crate::id_entity::IdEntity;
use crate::jobs_skill_data::machinist::abilities::make_machinist_skill_list;
use crate::jobs_skill_data::ninja::abilities::{
    bunshin_clone_id, bunshin_stack_id, bunshin_trigger_gcd_ids, make_ninja_skill_list,
};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{ComboType, IdType, PotencyType, ResourceType, TimeType};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::rc::Rc;

const QUEEN_BASE_POTENCY: PotencyType = 1330;
const QUEEN_DELAY_MILLISECOND: TimeType = 5000;
const QUEEN_POTENCY_PER_STACK: PotencyType = 266;
const HEAT_MAX: ResourceType = 100;
const BATTERY_MAX: ResourceType = 10;
const WILDFIRE_POTENCY_PER_STACK: PotencyType = 240;
const WILDFIRE_DELAY_MILLISECOND: TimeType = 10000;

#[derive(Clone)]
pub(crate) struct MachinistCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: Option<IdType>,
    heat: ResourceType,
    battery: ResourceType,
    queen_damage_incoming: Option<(PotencyType, TimeType)>,
    wildfire_damage_incoming: Option<(PotencyType, TimeType)>,
}

impl CombatResource for MachinistCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_type: ResourceType) {
        if resource_id == 0 {
            self.heat = min(HEAT_MAX, self.heat + resource_type);
        } else if resource_id == 1 {
            self.battery = min(BATTERY_MAX, self.battery + resource_type);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.heat
        } else if resource_id == 1 {
            self.battery
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &Option<IdType>) {
        if let Some(combo_id) = combo {
            self.current_combo = Some(*combo_id);
        }
    }

    fn trigger_on_event(
        &mut self,
        skill_id: IdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents {
        let mut ffxiv_events = vec![];

        if self.wildfire_damage_incoming.is_some() {
            let (potency, delay) = self.wildfire_damage_incoming.unwrap();

            if delay == 0 {
                ffxiv_events.push(Damage(
                    player.get_id(),
                    player.get_id(),
                    potency,
                    120,
                    false,
                    false,
                    buff_list.borrow().clone(),
                    debuff_list.borrow().clone(),
                    DamageCategory::Direct,
                    current_time_millisecond,
                ));
                self.wildfire_damage_incoming = None;
            } else if skill_id == 1409 {
                self.wildfire_damage_incoming = Some((potency + WILDFIRE_POTENCY_PER_STACK, delay));
            }
        }

        if self.queen_damage_incoming.is_some() {
            let (potency, delay) = self.queen_damage_incoming.unwrap();

            if delay == 0 {
                ffxiv_events.push(Damage(
                    player.get_id(),
                    player.get_id(),
                    potency,
                    120,
                    false,
                    false,
                    buff_list.borrow().clone(),
                    debuff_list.borrow().clone(),
                    DamageCategory::Direct,
                    current_time_millisecond,
                ));

                self.queen_damage_incoming = None;
            }
        }

        if skill_id == 1413 {
            let current_stack = self.battery;
            let potency =
                QUEEN_BASE_POTENCY + QUEEN_POTENCY_PER_STACK * (current_stack as PotencyType - 5);
            self.queen_damage_incoming = Some((potency, QUEEN_DELAY_MILLISECOND));
            self.battery = 0;
        }

        if skill_id == 1410 {
            self.wildfire_damage_incoming = Some((0, WILDFIRE_DELAY_MILLISECOND));
        }

        return (ffxiv_events, vec![]);
    }

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }

    fn update_stack_timer(&mut self, elapsed_time: TimeType) {
        if let Some((potency, delay)) = self.queen_damage_incoming {
            self.queen_damage_incoming = Some((potency, max(delay - elapsed_time, 0)));
        }

        if let Some((potency, delay)) = self.wildfire_damage_incoming {
            self.wildfire_damage_incoming = Some((potency, max(delay - elapsed_time, 0)));
        }
    }
    fn trigger_on_crit(&mut self) {}
}

impl MachinistCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_machinist_skill_list(player_id),
            current_combo: None,
            heat: 0,
            battery: 0,
            queen_damage_incoming: None,
            wildfire_damage_incoming: None,
        }
    }
}
