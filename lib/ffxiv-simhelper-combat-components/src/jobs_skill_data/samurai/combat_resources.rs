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
use crate::types::{SkillIdType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const SAMURAI_STACK_COUNT: usize = 6;

const GENKI_MAX_STACK: ResourceType = 100;
const MEDITATION_MAX_STACK: ResourceType = 3;
const SEN_1_MAX_STACK: ResourceType = 1;
const SEN_2_MAX_STACK: ResourceType = 1;
const SEN_3_MAX_STACK: ResourceType = 1;
const FILLER_MAX_STACK: ResourceType = 1;

const SAMURAI_MAX_STACKS: [ResourceType; SAMURAI_STACK_COUNT] = [
    GENKI_MAX_STACK,
    MEDITATION_MAX_STACK,
    SEN_1_MAX_STACK,
    SEN_2_MAX_STACK,
    SEN_3_MAX_STACK,
    FILLER_MAX_STACK,
];

#[derive(Clone)]
pub(crate) struct SamuraiCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    resources: [ResourceType; SAMURAI_STACK_COUNT],
}

impl CombatResource for SamuraiCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            SAMURAI_MAX_STACKS[resource_id],
            self.resources[resource_id] + resource_amount,
        );
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        self.resources[resource_id as usize]
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
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl SamuraiCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_samurai_skill_list(player_id),
            current_combo: None,
            resources: [0, 0, 0, 0, 0, 0],
        }
    }
}
