use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::dancer::abilities::make_dancer_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, ResourceType};
use crate::{IdType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const ESPRIT_MAX_STACK: ResourceType = 100;
const FEATHER_MAX_STACK: ResourceType = 4;

#[derive(Clone)]
pub(crate) struct DancerCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    player_id: IdType,
    partner_player_id: IdType,
    esprit: ResourceType,
    feather: ResourceType,
}

impl CombatResource for DancerCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.esprit = min(ESPRIT_MAX_STACK, self.esprit + resource_amount);
        } else if resource_id == 1 {
            self.feather = min(FEATHER_MAX_STACK, self.feather + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.esprit
        } else if resource_id == 1 {
            self.feather
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
        self.partner_player_id
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
    fn trigger_on_crit(&mut self) {}
}

impl DancerCombatResources {
    pub(crate) fn new(player_id: IdType, partner_player_id: IdType) -> Self {
        Self {
            skills: make_dancer_skill_list(player_id, partner_player_id),
            current_combo: None,
            player_id,
            partner_player_id,
            esprit: 0,
            feather: 0,
        }
    }
}
