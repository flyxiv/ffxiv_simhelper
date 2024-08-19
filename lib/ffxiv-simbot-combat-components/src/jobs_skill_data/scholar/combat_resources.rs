use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::scholar::abilities::make_scholar_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType};
use crate::types::{IdType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const AETHER_MAX_STACK: ResourceType = 3;

#[derive(Clone)]
pub(crate) struct ScholarCombatResources {
    skills: SkillTable<AttackSkill>,
    aether_stack: ResourceType,
}

impl CombatResource for ScholarCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.aether_stack = min(AETHER_MAX_STACK, self.aether_stack + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        if resource_id == 0 {
            self.aether_stack
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        None
    }

    fn update_combo(&mut self, _: &ComboType) {}

    fn trigger_on_event(
        &mut self,
        _: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        (vec![], vec![])
    }

    fn get_next_buff_target(&self, _: IdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
    fn trigger_on_crit(&mut self) {}
}

impl ScholarCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_scholar_skill_list(player_id),
            aether_stack: 0,
        }
    }
}
