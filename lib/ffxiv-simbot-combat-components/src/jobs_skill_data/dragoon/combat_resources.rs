use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::dragoon::abilities::make_dragoon_skill_list;
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

const NASTROND_MAX_STACK: ResourceType = 3;
const FIRSTMIND_MAX_STACK: ResourceType = 2;

#[derive(Clone)]
pub(crate) struct DragoonCombatResources {
    skills: SkillTable<AttackSkill>,

    #[allow(unused)]
    player_id: PlayerIdType,
    current_combo: ComboType,
    nastrond_stack: ResourceType,
    firstmind_focus: ResourceType,
}

impl CombatResource for DragoonCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.nastrond_stack = min(NASTROND_MAX_STACK, self.nastrond_stack + resource_amount);
        } else if resource_id == 1 {
            self.firstmind_focus = min(FIRSTMIND_MAX_STACK, self.firstmind_focus + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        if resource_id == 0 {
            self.nastrond_stack
        } else if resource_id == 1 {
            self.firstmind_focus
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

impl DragoonCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_dragoon_skill_list(player_id),
            player_id,
            current_combo: None,
            nastrond_stack: 0,
            firstmind_focus: 0,
        }
    }
}
