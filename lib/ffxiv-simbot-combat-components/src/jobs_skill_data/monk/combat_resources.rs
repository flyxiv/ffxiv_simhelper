use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::monk::abilities::make_monk_skill_list;
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

const CHAKRA_MAX_STACK: ResourceType = 5;
const PERFECT_MAX_STACK: ResourceType = 3;

#[derive(Clone)]
pub(crate) struct MonkCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,
    chakra: RefCell<ResourceType>,
    perfect_1: RefCell<ResourceType>,
    perfect_2: RefCell<ResourceType>,
    perfect_3: RefCell<ResourceType>,
    lunar: RefCell<ResourceType>,
    solar: RefCell<ResourceType>,
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
            let chakra_stack = *self.chakra.borrow();
            self.chakra
                .replace(min(CHAKRA_MAX_STACK, chakra_stack + resource_amount));
        } else if resource_id == 1 {
            let perfect_1_stack = *self.perfect_1.borrow();
            self.perfect_1
                .replace(min(PERFECT_MAX_STACK, perfect_1_stack + resource_amount));
        } else if resource_id == 2 {
            let perfect_2_stack = *self.perfect_2.borrow();
            self.perfect_2
                .replace(min(PERFECT_MAX_STACK, perfect_2_stack + resource_amount));
        } else if resource_id == 3 {
            let perfect_3_stack = *self.perfect_3.borrow();
            self.perfect_3
                .replace(min(PERFECT_MAX_STACK, perfect_3_stack + resource_amount));
        } else if resource_id == 4 {
            let lunar_stack = *self.lunar.borrow();
            self.lunar
                .replace(min(PERFECT_MAX_STACK, lunar_stack + resource_amount));
        } else if resource_id == 5 {
            let solar_stack = *self.solar.borrow();
            self.solar
                .replace(min(PERFECT_MAX_STACK, solar_stack + resource_amount));
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            *self.chakra.borrow()
        } else if resource_id == 1 {
            *self.perfect_1.borrow()
        } else if resource_id == 2 {
            *self.perfect_2.borrow()
        } else if resource_id == 3 {
            *self.perfect_3.borrow()
        } else if resource_id == 4 {
            *self.lunar.borrow()
        } else if resource_id == 5 {
            *self.solar.borrow()
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
        &self,
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
}

impl MonkCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_monk_skill_list(player_id),
            player_id,
            current_combo: None,
            chakra: RefCell::new(5),
            perfect_1: RefCell::new(0),
            perfect_2: RefCell::new(0),
            perfect_3: RefCell::new(0),
            lunar: RefCell::new(0),
            solar: RefCell::new(0),
        }
    }
}
