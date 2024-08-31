use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::reaper::abilities::make_reaper_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType};
use crate::types::{IdType, TimeType};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

lazy_static! {
    static ref REAPER_NORMAL_GCD_IDS: Vec<IdType> = vec![1200, 1201, 1202];
}

const SOUL_GAUGE_MAX: ResourceType = 100;
const ENSHROUD_GAUGE_MAX: ResourceType = 100;
const SOUL_REAVER_MAX: ResourceType = 1;
const ENSHROUD_STACK_MAX: ResourceType = 5;
const LEMURES_STACK_MAX: ResourceType = 4;
const EXECUTIONER_STACK_MAX: ResourceType = 2;

#[derive(Clone)]
pub(crate) struct ReaperCombatResources {
    skills: SkillTable<AttackSkill>,

    #[allow(unused)]
    player_id: PlayerIdType,
    current_combo: ComboType,
    soul_gauge: ResourceType,
    enshroud_gauge: ResourceType,
    soul_reaver_stack: ResourceType,
    enshroud_stack: ResourceType,
    lemures_stack: ResourceType,
    executioner_stack: ResourceType,
    enshroud_count: ResourceType,
}

impl CombatResource for ReaperCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
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
        } else if resource_id == 5 {
            self.executioner_stack = min(
                EXECUTIONER_STACK_MAX,
                self.executioner_stack + resource_amount,
            );
        } else if resource_id == 6 {
            self.enshroud_count = self.enshroud_count + resource_amount;
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
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
        } else if resource_id == 5 {
            self.executioner_stack
        } else if resource_id == 6 {
            self.enshroud_count
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &ComboType) {
        if let Some(combo) = combo {
            self.current_combo = Some(*combo);
        }
    }

    fn trigger_on_event(
        &mut self,
        skill_id: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        if REAPER_NORMAL_GCD_IDS.contains(&skill_id) {
            self.enshroud_count = 0;
        }

        (vec![], vec![])
    }

    fn trigger_on_crit(&mut self) {}

    fn get_next_buff_target(&self, _: IdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl ReaperCombatResources {
    pub(crate) fn new(player_id: PlayerIdType, player_count: usize) -> Self {
        Self {
            skills: make_reaper_skill_list(player_id, player_count),
            player_id,
            current_combo: None,
            soul_gauge: 0,
            enshroud_gauge: 0,
            soul_reaver_stack: 0,
            enshroud_stack: 0,
            lemures_stack: 0,
            executioner_stack: 0,
            enshroud_count: 0,
        }
    }
}
