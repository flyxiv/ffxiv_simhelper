use crate::combat_resources::CombatResource;
use crate::event::FfxivEventQueue;
use crate::jobs_skill_data::samurai::abilities::make_samurai_skill_list;
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

const SOUL_GAUGE_MAX: ResourceType = 100;
const ENSHROUD_GAUGE_MAX: ResourceType = 50;
const SOUL_REAVER_MAX: ResourceType = 2;
const ENSHROUD_STACK_MAX: ResourceType = 5;
const LEMURES_STACK_MAX: ResourceType = 2;

#[derive(Clone)]
pub(crate) struct ReaperCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,
    soul_gauge: ResourceType,
    enshroud_gauge: ResourceType,
    soul_reaver_stack: ResourceType,
    enshroud_stack: ResourceType,
    lemures_stack: ResourceType,
}

impl CombatResource for ReaperCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.soul_gauge = min(SOUL_GAUGE_MAX, self.soul_gauge + resource_amount);
        } else if resource_id == 1 {
            self.enshroud_gauge = min(ENSHROUD_GAUGE_MAX, self.enshroud_gauge + resource_amount);
        } else if resource_id == 2 {
            self.soul_reaver_stack = min(SOUL_REAVER_MAX, self.soul_reaver_stack + resource_amount);
        } else if resource_id == 3 {
            self.enshroud_stack = min(ENSHROUD_STACK_MAX, self.enshroud_stack + resource_amount);
        } else if resource_id == 4 {
            self.lemures_stack = min(LEMURES_STACK_MAX, self.lemures_stack + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.soul_gauge
        } else if resource_id == 1 {
            self.enshroud_gauge
        } else if resource_id == 2 {
            self.soul_reaver_stack
        } else if resource_id == 3 {
            self.enshroud_stack
        } else if resource_id == 4 {
            self.lemures_stack
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &Option<IdType>) {
        if let Some(combo) = combo {
            self.current_combo = Some(*combo);
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

    fn trigger_on_crit(&mut self) {}

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl ReaperCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_samurai_skill_list(player_id),
            player_id,
            current_combo: None,
            soul_gauge: 0,
            enshroud_gauge: 0,
            soul_reaver_stack: 0,
            enshroud_stack: 0,
            lemures_stack: 0,
        }
    }
}
