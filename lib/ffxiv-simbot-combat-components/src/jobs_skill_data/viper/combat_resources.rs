use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::viper::abilities::make_viper_skill_list;
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

const SERPENT_OFFERINGS_MAX: ResourceType = 100;
const RATTLING_COIL_MAX: ResourceType = 3;
const REAWAKEN_STACK_MAX: ResourceType = 5;

#[derive(Clone)]
#[allow(unused)]
pub(crate) struct ViperCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: PlayerIdType,
    current_combo: ComboType,

    serpent_offering_stack: ResourceType,
    filler_stack1: ResourceType,
    filler_stack2: ResourceType,
    reawaken_filler_stack: ResourceType,
    rattling_coil_stack: ResourceType,
    hunters_coil_stack: ResourceType,
    swiftskin_coil_stack: ResourceType,
    reawaken_stack: ResourceType,
    death_rattle_stack: ResourceType,
}

impl CombatResource for ViperCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, amount: ResourceType) {
        if resource_id == 0 {
            self.serpent_offering_stack =
                min(self.serpent_offering_stack + amount, SERPENT_OFFERINGS_MAX);
        } else if resource_id == 1 {
            self.filler_stack1 = min(self.filler_stack1 + amount, 1);
        } else if resource_id == 2 {
            self.filler_stack2 = min(self.filler_stack2 + amount, 1);
        } else if resource_id == 3 {
            self.reawaken_filler_stack = min(self.reawaken_filler_stack + amount, 1);
        } else if resource_id == 4 {
            self.rattling_coil_stack = min(self.rattling_coil_stack + amount, RATTLING_COIL_MAX);
        } else if resource_id == 5 {
            self.hunters_coil_stack = min(self.hunters_coil_stack + amount, 1);
        } else if resource_id == 6 {
            self.swiftskin_coil_stack = min(self.swiftskin_coil_stack + amount, 1);
        } else if resource_id == 7 {
            self.reawaken_stack = min(self.reawaken_stack + amount, REAWAKEN_STACK_MAX);
        } else if resource_id == 8 {
            self.death_rattle_stack = min(self.death_rattle_stack + amount, 1);
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        if resource_id == 0 {
            self.serpent_offering_stack
        } else if resource_id == 1 {
            self.filler_stack1
        } else if resource_id == 2 {
            self.filler_stack2
        } else if resource_id == 3 {
            self.reawaken_filler_stack
        } else if resource_id == 4 {
            self.rattling_coil_stack
        } else if resource_id == 5 {
            self.hunters_coil_stack
        } else if resource_id == 6 {
            self.swiftskin_coil_stack
        } else if resource_id == 7 {
            self.reawaken_stack
        } else if resource_id == 8 {
            self.death_rattle_stack
        } else {
            -1
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
        _: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        (vec![], vec![])
    }

    fn trigger_on_crit(&mut self) {}
    fn get_next_buff_target(&self, _: IdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl ViperCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_viper_skill_list(player_id),
            player_id,
            current_combo: None,

            serpent_offering_stack: 0,
            filler_stack1: 0,
            filler_stack2: 0,
            reawaken_filler_stack: 0,
            rattling_coil_stack: 0,
            hunters_coil_stack: 0,
            swiftskin_coil_stack: 0,
            reawaken_stack: 0,
            death_rattle_stack: 0,
        }
    }
}
