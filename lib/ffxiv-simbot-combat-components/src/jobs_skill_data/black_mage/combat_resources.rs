use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::black_mage::abilities::make_blackmage_skill_list;
use crate::live_objects::player::create_player::BLACKMAGE_START_TIME_MILLISECOND;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{ComboType, IdType, ResourceType, TimeType, SIMULATION_START_TIME_MILLISECOND};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const POLYGLOT_STACK_INTERVAL_MILLISECOND: TimeType = 30000;
const POLYGLOT_MAX_STACK: ResourceType = 2;
const PARADOX_GAUGE_MAX_STACK: ResourceType = 1;

#[derive(Clone)]
pub(crate) struct BlackmageCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,
    polyglot_stack: RefCell<ResourceType>,
    paradox_gauge_stack: RefCell<ResourceType>,
    fire4_stack: RefCell<ResourceType>,
    next_polyglot_time: RefCell<TimeType>,
}

impl CombatResource for BlackmageCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            let polyglot_stack = *self.polyglot_stack.borrow();
            self.polyglot_stack
                .replace(min(POLYGLOT_MAX_STACK, polyglot_stack + resource_amount));
        } else if resource_id == 1 {
            let paradox_stack = *self.paradox_gauge_stack.borrow();
            self.paradox_gauge_stack.replace(min(
                PARADOX_GAUGE_MAX_STACK,
                paradox_stack + resource_amount,
            ));
        } else if resource_id == 2 {
            let fire4_stack = *self.fire4_stack.borrow();
            self.fire4_stack
                .replace(min(6, fire4_stack + resource_amount));
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            *self.polyglot_stack.borrow()
        } else if resource_id == 1 {
            *self.paradox_gauge_stack.borrow()
        } else if resource_id == 2 {
            *self.fire4_stack.borrow()
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
        &self,
        skill_id: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        if skill_id == 1712 {
            self.fire4_stack.replace(0);
        }
        (vec![], vec![])
    }

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }
    fn update_stack_timer(&mut self, elapsed_time_millisecond: TimeType) {
        if elapsed_time_millisecond >= *self.next_polyglot_time.borrow() {
            let polyglot_stack = *self.polyglot_stack.borrow();
            *self.polyglot_stack.borrow_mut() = min(POLYGLOT_MAX_STACK, polyglot_stack + 1);
            *self.next_polyglot_time.borrow_mut() += POLYGLOT_STACK_INTERVAL_MILLISECOND;
        }

        *self.next_polyglot_time.borrow_mut() -= elapsed_time_millisecond;
    }
}

impl BlackmageCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: make_blackmage_skill_list(player_id),
            player_id,
            current_combo: None,
            polyglot_stack: RefCell::new(0),
            paradox_gauge_stack: RefCell::new(0),
            fire4_stack: RefCell::new(0),
            next_polyglot_time: RefCell::new(
                POLYGLOT_STACK_INTERVAL_MILLISECOND
                    + TimeType::abs(SIMULATION_START_TIME_MILLISECOND),
            ),
        }
    }
}
