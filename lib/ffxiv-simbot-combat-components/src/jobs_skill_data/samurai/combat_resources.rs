use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::samurai::abilities::make_samurai_skill_list;
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

const GENKI_MAX_STACK: ResourceType = 100;
const MEDITATION_MAX_STACK: ResourceType = 3;
const SEN_1_MAX_STACK: ResourceType = 1;
const SEN_2_MAX_STACK: ResourceType = 1;
const SEN_3_MAX_STACK: ResourceType = 1;
const FILLER_MAX_STACK: ResourceType = 1;

#[derive(Clone)]
pub(crate) struct SamuraiCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    genki_stack: ResourceType,
    meditation_stack: ResourceType,
    sen_1_stack: ResourceType,
    sen_2_stack: ResourceType,
    sen_3_stack: ResourceType,
    filler_stack: ResourceType,
}

impl CombatResource for SamuraiCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.genki_stack = min(GENKI_MAX_STACK, self.genki_stack + resource_amount);
        } else if resource_id == 1 {
            self.meditation_stack = min(
                MEDITATION_MAX_STACK,
                self.meditation_stack + resource_amount,
            );
        } else if resource_id == 2 {
            self.sen_1_stack = min(SEN_1_MAX_STACK, self.sen_1_stack + resource_amount);
        } else if resource_id == 3 {
            self.sen_2_stack = min(SEN_2_MAX_STACK, self.sen_2_stack + resource_amount);
        } else if resource_id == 4 {
            self.sen_3_stack = min(SEN_3_MAX_STACK, self.sen_3_stack + resource_amount);
        } else if resource_id == 5 {
            self.filler_stack = min(FILLER_MAX_STACK, self.filler_stack + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        if resource_id == 0 {
            self.genki_stack
        } else if resource_id == 1 {
            self.meditation_stack
        } else if resource_id == 2 {
            self.sen_1_stack
        } else if resource_id == 3 {
            self.sen_2_stack
        } else if resource_id == 4 {
            self.sen_3_stack
        } else if resource_id == 5 {
            self.filler_stack
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

impl SamuraiCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_samurai_skill_list(player_id),
            current_combo: None,
            genki_stack: 0,
            meditation_stack: 0,
            sen_1_stack: 0,
            sen_2_stack: 0,
            sen_3_stack: 0,
            filler_stack: 3,
        }
    }
}
