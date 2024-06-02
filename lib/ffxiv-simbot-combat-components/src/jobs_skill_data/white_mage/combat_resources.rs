use crate::combat_resources::CombatResource;
use crate::event::FfxivEventQueue;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::{ComboType, IdType, ResourceType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const LILY_STACK_INTERVAL_MILLISECOND: TimeType = 30000;
const LILY_MAX_STACK: ResourceType = 3;
const BLOOD_LILY_MAX_STACK: ResourceType = 3;

#[derive(Clone)]
pub(crate) struct WhitemageCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    blood_lily_stack: RefCell<ResourceType>,
    lily_stack: RefCell<ResourceType>,
    next_lily_time: RefCell<TimeType>,
}

impl CombatResource for WhitemageCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            let blood_lily_stack = *self.blood_lily_stack.borrow();
            self.blood_lily_stack.replace(min(
                BLOOD_LILY_MAX_STACK,
                blood_lily_stack + resource_amount,
            ));
        } else if resource_id == 1 {
            let lily_stack = *self.lily_stack.borrow();
            self.lily_stack
                .replace(min(LILY_MAX_STACK, lily_stack + resource_amount));
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            *self.blood_lily_stack.borrow()
        } else if resource_id == 1 {
            *self.lily_stack.borrow()
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        None
    }

    fn update_combo(&mut self, _: &Option<IdType>) {}

    fn trigger_on_event(
        &self,
        skill_id: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_combat_time_millisecond: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        (vec![], vec![])
    }

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }
    fn update_stack_timer(&mut self, elapsed_time_millisecond: TimeType) {
        if elapsed_time_millisecond >= *self.next_lily_time.borrow() {
            let lily_stack = *self.lily_stack.borrow();
            *self.lily_stack.borrow_mut() = min(LILY_MAX_STACK, lily_stack + 1);
            *self.lily_stack.borrow_mut() += LILY_STACK_INTERVAL_MILLISECOND;
            *self.lily_stack.borrow_mut() -= elapsed_time_millisecond;
        }
    }
}

impl WhitemageCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            skills: SkillTable::new(),
            player_id,
            blood_lily_stack: RefCell::new(0),
            lily_stack: RefCell::new(0),
            next_lily_time: RefCell::new(LILY_STACK_INTERVAL_MILLISECOND),
        }
    }
}
