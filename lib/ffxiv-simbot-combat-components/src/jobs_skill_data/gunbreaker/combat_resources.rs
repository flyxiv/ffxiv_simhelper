use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::gunbreaker::abilities::make_gunbreaker_skill_list;
use crate::jobs_skill_data::paladin::abilities::make_paladin_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{ComboType, IdType, ResourceType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const SOIL_MAX: ResourceType = 3;

#[derive(Clone)]
pub(crate) struct GunbreakerCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,
    soil: ResourceType,
}

impl CombatResource for GunbreakerCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, amount: ResourceType) {
        if resource_id == 0 {
            self.soil = min(self.soil + amount, SOIL_MAX);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.soil
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
    fn trigger_on_crit(&mut self) {}
}

impl GunbreakerCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_gunbreaker_skill_list(player_id),
            player_id,
            current_combo: None,
            soil: 0,
        }
    }
}
