use crate::combat_resources::CombatResource;
use crate::consts::SIMULATION_START_TIME_MILLISECOND;
use crate::jobs_skill_data::white_mage::abilities::make_whitemage_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
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

const WHITEMAGE_STACK_COUNT: usize = 3;

const LILY_STACK_ID: ResourceIdType = 1;
const LILY_STACK_INTERVAL_MILLISECOND: TimeType = 20000;
const LILY_MAX_STACK: ResourceType = 3;
const GLARE4_MAX_STACK: ResourceType = 3;
const BLOOD_LILY_MAX_STACK: ResourceType = 3;

const WHITEMAGE_MAX_STACKS: [ResourceType; WHITEMAGE_STACK_COUNT] =
    [LILY_MAX_STACK, GLARE4_MAX_STACK, BLOOD_LILY_MAX_STACK];

#[derive(Clone)]
pub(crate) struct WhitemageCombatResources {
    skills: SkillTable<AttackSkill>,
    resources: [ResourceType; WHITEMAGE_STACK_COUNT],
    next_lily_time: TimeType,
}

impl CombatResource for WhitemageCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            WHITEMAGE_MAX_STACKS[resource_id],
            self.resources[resource_id] + resource_amount,
        );
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        self.resources[resource_id as usize]
    }

    fn get_current_combo(&self) -> ComboType {
        None
    }

    fn update_combo(&mut self, _: &ComboType) {}

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
    fn update_stack_timer(&mut self, elapsed_time_millisecond: TimeType) {
        if elapsed_time_millisecond >= self.next_lily_time {
            self.add_resource(LILY_STACK_ID, 1);
            self.next_lily_time += LILY_STACK_INTERVAL_MILLISECOND;
        }

        self.next_lily_time -= elapsed_time_millisecond;
    }
}

impl WhitemageCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_whitemage_skill_list(player_id),
            resources: [0; WHITEMAGE_STACK_COUNT],
            next_lily_time: LILY_STACK_INTERVAL_MILLISECOND
                + TimeType::abs(SIMULATION_START_TIME_MILLISECOND),
        }
    }
}
