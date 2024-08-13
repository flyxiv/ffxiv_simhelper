use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::monk::abilities::make_monk_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, ResourceType};
use crate::types::{IdType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const CHAKRA_MAX_STACK: ResourceType = 5;
const PERFECT_MAX_STACK: ResourceType = 3;

#[derive(Clone)]
pub(crate) struct MonkCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,
    chakra: ResourceType,
    perfect_1: ResourceType,
    perfect_2: ResourceType,
    perfect_3: ResourceType,
    lunar: ResourceType,
    solar: ResourceType,
    leaping_stack: ResourceType,
    raptor_stack: ResourceType,
    coeurl_stack: ResourceType,
}

impl CombatResource for MonkCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.chakra = min(CHAKRA_MAX_STACK, self.chakra + resource_amount);
        } else if resource_id == 1 {
            self.perfect_1 = min(PERFECT_MAX_STACK, self.perfect_1 + resource_amount);
        } else if resource_id == 2 {
            self.perfect_2 = min(PERFECT_MAX_STACK, self.perfect_2 + resource_amount);
        } else if resource_id == 3 {
            self.perfect_3 = min(PERFECT_MAX_STACK, self.perfect_3 + resource_amount);
        } else if resource_id == 4 {
            self.lunar = min(1, self.lunar + resource_amount);
        } else if resource_id == 5 {
            self.solar = min(1, self.solar + resource_amount);
        } else if resource_id == 6 {
            self.leaping_stack = min(1, self.leaping_stack + resource_amount);
        } else if resource_id == 7 {
            self.raptor_stack = min(1, self.raptor_stack + resource_amount);
        } else if resource_id == 8 {
            self.coeurl_stack = min(1, self.coeurl_stack + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.chakra
        } else if resource_id == 1 {
            self.perfect_1
        } else if resource_id == 2 {
            self.perfect_2
        } else if resource_id == 3 {
            self.perfect_3
        } else if resource_id == 4 {
            self.lunar
        } else if resource_id == 5 {
            self.solar
        } else if resource_id == 6 {
            self.leaping_stack
        } else if resource_id == 7 {
            self.raptor_stack
        } else if resource_id == 8 {
            self.coeurl_stack
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
    fn trigger_on_crit(&mut self) {
        self.chakra = min(CHAKRA_MAX_STACK, self.chakra + 1);
    }
}

impl MonkCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_monk_skill_list(player_id),
            player_id,
            current_combo: None,
            chakra: 5,
            perfect_1: 0,
            perfect_2: 0,
            perfect_3: 0,
            lunar: 0,
            solar: 0,
            leaping_stack: 0,
            raptor_stack: 0,
            coeurl_stack: 0,
        }
    }
}
