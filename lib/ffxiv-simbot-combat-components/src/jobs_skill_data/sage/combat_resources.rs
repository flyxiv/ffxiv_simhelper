use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::sage::abilities::make_sage_skills;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{ComboType, IdType, ResourceType, TimeType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct SageCombatResources {
    skills: SkillTable<AttackSkill>,
}

impl CombatResource for SageCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, _: IdType, _: ResourceType) {}

    fn get_resource(&self, _: IdType) -> ResourceType {
        -1
    }

    fn get_current_combo(&self) -> ComboType {
        None
    }

    fn update_combo(&mut self, _: &ComboType) {}

    fn trigger_on_event(
        &self,
        _: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        (vec![], vec![])
    }

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }

    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl SageCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_sage_skills(player_id),
        }
    }
}
