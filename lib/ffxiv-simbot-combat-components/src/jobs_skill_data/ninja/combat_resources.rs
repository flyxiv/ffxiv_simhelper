use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::jobs_skill_data::ninja::abilities::{
    bunshin_clone_id, bunshin_stack_id, make_ninja_skill_list,
};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::priority_simulation_data::EMPTY_RESOURCE;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType};
use crate::types::{SkillIdType, TimeType};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const NINKI_MAX: ResourceType = 100;
const SHURIKEN_MAX_STACK: ResourceType = 5;

lazy_static! {
    static ref NINJA_BUNSHIN_COMBO_IDS: Vec<SkillIdType> = vec![1005, 1006, 1007, 1008, 1002];
}

#[derive(Clone)]
pub(crate) struct NinjaCombatResources {
    skills: SkillTable<AttackSkill>,
    ninki: ResourceType,
    shuriken: ResourceType,
    current_combo: ComboType,
}

impl CombatResource for NinjaCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_type: ResourceType) {
        if resource_id == 0 {
            let ninki = self.ninki;
            self.ninki = min(NINKI_MAX, ninki + resource_type);
        } else if resource_id == 1 {
            let shuriken = self.shuriken;
            self.shuriken = min(SHURIKEN_MAX_STACK, shuriken + resource_type);
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        if resource_id == 0 {
            self.ninki
        } else if resource_id == 1 {
            self.shuriken
        } else {
            EMPTY_RESOURCE
        }
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &ComboType) {
        if let Some(combo_id) = combo {
            self.current_combo = Some(*combo_id);
        }
    }

    fn trigger_on_event(
        &mut self,
        skill_id: SkillIdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents {
        if NINJA_BUNSHIN_COMBO_IDS.contains(&skill_id) {
            let key = StatusKey::new(bunshin_stack_id(), player.get_id());

            if buff_list.borrow().contains_key(&key) {
                return (
                    vec![FfxivEvent::UseSkill(
                        player.get_id(),
                        None,
                        bunshin_clone_id(),
                        current_time_millisecond,
                    )],
                    vec![],
                );
            }
        }

        return (vec![], vec![]);
    }

    fn trigger_on_crit(&mut self) {}

    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl NinjaCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_ninja_skill_list(player_id),
            ninki: 0,
            shuriken: 0,
            current_combo: None,
        }
    }
}
