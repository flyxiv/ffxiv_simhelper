use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::dragoon::abilities::make_dragoon_skill_list;
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

const MIRAGE_MAX_STACK: ResourceType = 2;
const FIRSTMIND_MAX_STACK: ResourceType = 2;

#[derive(Clone)]
pub(crate) struct DragoonCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,
    mirage_gauge: ResourceType,
    firstmind_focus: ResourceType,
}

impl CombatResource for DragoonCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.mirage_gauge = min(MIRAGE_MAX_STACK, self.mirage_gauge + resource_amount);
        } else if resource_id == 1 {
            self.firstmind_focus = min(FIRSTMIND_MAX_STACK, self.firstmind_focus + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.mirage_gauge
        } else if resource_id == 1 {
            self.firstmind_focus
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

    // TODO: chakra on crit
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

impl DragoonCombatResources {
    pub(crate) fn new(player_id: IdType, partner_player_id: IdType) -> Self {
        Self {
            skills: make_dragoon_skill_list(player_id, partner_player_id),
            player_id,
            current_combo: None,
            mirage_gauge: 0,
            firstmind_focus: 0,
        }
    }
}
