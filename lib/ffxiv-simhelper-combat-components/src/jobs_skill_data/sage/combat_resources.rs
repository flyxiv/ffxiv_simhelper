use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::sage::abilities::make_sage_skills;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType};
use crate::types::{SkillIdType, TimeType};
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

    fn add_resource(&mut self, _: ResourceIdType, _: ResourceType) {}

    fn get_resource(&self, _: ResourceIdType) -> ResourceType {
        -100
    }

    fn get_current_combo(&self) -> ComboType {
        None
    }

    fn update_combo(&mut self, _: &ComboType) {}

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

    fn trigger_on_gcd_crit(&mut self) {}

    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_other_time_related_states(&mut self, _: TimeType) {}
    fn get_combo_remaining_time(&self) -> TimeType {
        0
    }
}

impl SageCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_sage_skills(player_id),
        }
    }
}
