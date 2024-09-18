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
use crate::types::{SkillIdType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const VIPER_STACK_COUNT: usize = 9;

const SERPENT_OFFERINGS_MAX: ResourceType = 100;
const RATTLING_COIL_MAX: ResourceType = 3;
const REAWAKEN_STACK_MAX: ResourceType = 5;

const VIPER_MAX_STACKS: [ResourceType; VIPER_STACK_COUNT] = [
    SERPENT_OFFERINGS_MAX,
    1,
    1,
    1,
    RATTLING_COIL_MAX,
    1,
    1,
    REAWAKEN_STACK_MAX,
    1,
];

#[derive(Clone)]
#[allow(unused)]
pub(crate) struct ViperCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: PlayerIdType,
    current_combo: ComboType,

    resources: [ResourceType; VIPER_STACK_COUNT],
}

impl CombatResource for ViperCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            VIPER_MAX_STACKS[resource_id],
            self.resources[resource_id] + amount,
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

            resources: [0; VIPER_STACK_COUNT],
        }
    }
}
