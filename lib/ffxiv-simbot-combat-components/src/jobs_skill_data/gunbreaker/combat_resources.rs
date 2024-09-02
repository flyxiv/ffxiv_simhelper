use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::gunbreaker::abilities::make_gunbreaker_skill_list;
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
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const SOIL_MAX: ResourceType = 3;

#[derive(Clone)]
pub(crate) struct GunbreakerCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    soil: ResourceType,
    noble_blood_stack: ResourceType,
    lion_heart_stack: ResourceType,
}

impl CombatResource for GunbreakerCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, amount: ResourceType) {
        if resource_id == 0 {
            self.soil = min(self.soil + amount, SOIL_MAX);
        } else if resource_id == 1 {
            self.noble_blood_stack = min(self.noble_blood_stack + amount, 1);
        } else if resource_id == 2 {
            self.lion_heart_stack = min(self.lion_heart_stack + amount, 1);
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        if resource_id == 0 {
            self.soil
        } else if resource_id == 1 {
            self.noble_blood_stack
        } else if resource_id == 2 {
            self.lion_heart_stack
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
        _: SkillIdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        (vec![], vec![])
    }

    fn trigger_on_crit(&mut self) {}
    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl GunbreakerCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_gunbreaker_skill_list(player_id),
            current_combo: None,
            soil: 0,
            noble_blood_stack: 0,
            lion_heart_stack: 0,
        }
    }
}
