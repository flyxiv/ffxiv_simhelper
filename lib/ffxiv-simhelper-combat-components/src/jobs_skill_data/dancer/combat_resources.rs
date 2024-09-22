use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::dancer::abilities::make_dancer_skill_list;
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

const DANCER_STACK_COUNT: usize = 2;

const ESPRIT_MAX_STACK: ResourceType = 100;
const FEATHER_MAX_STACK: ResourceType = 4;

const DANCER_MAX_STACKS: [ResourceType; DANCER_STACK_COUNT] = [ESPRIT_MAX_STACK, FEATHER_MAX_STACK];

#[derive(Clone)]
pub(crate) struct DancerCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    partner_player_id: PlayerIdType,
    resources: [ResourceType; DANCER_STACK_COUNT],
}

impl CombatResource for DancerCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            DANCER_MAX_STACKS[resource_id],
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
        if let Some(combo_id) = combo {
            self.current_combo = Some(*combo_id);
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
        self.partner_player_id
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl DancerCombatResources {
    pub(crate) fn new(player_id: PlayerIdType, partner_player_id: PlayerIdType) -> Self {
        Self {
            skills: make_dancer_skill_list(player_id, partner_player_id),
            current_combo: None,
            partner_player_id,
            resources: [0; DANCER_STACK_COUNT],
        }
    }
}
