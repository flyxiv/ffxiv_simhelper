use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::monk::abilities::make_monk_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType};
use crate::types::{SkillIdType, TimeType};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

lazy_static! {
    static ref MONK_COMBO2_COMBO3_SKILL_IDS: Vec<SkillIdType> = vec![901, 902, 903, 904];
}

const MONK_STACKS_COUNT: usize = 10;

const CHAKRA_MAX_STACK: ResourceType = 5;
const PERFECT_MAX_STACK: ResourceType = 3;

const CHAKRA_STACK_ID: ResourceIdType = 0;
const FIRES_REPLY_STACK_ID: ResourceIdType = 9;

const MONK_MAX_STACKS: [ResourceType; MONK_STACKS_COUNT] = [
    CHAKRA_MAX_STACK,
    PERFECT_MAX_STACK,
    PERFECT_MAX_STACK,
    PERFECT_MAX_STACK,
    1,
    1,
    1,
    1,
    2,
    1,
];

/// Monk Combat Resources Mechanism
/// Only uses double lunar opener
///
/// ## Resource Explanation
/// resource[0]: chakra stack
/// resource[1]: the perfect balance stack for the Opo-Opo chakra
/// resource[2]: the perfect balance stack for the Raptor chakra
/// resource[3]: the perfect balance stack for the Coeurl chakra
/// resource[4]: lunar nadi
/// resource[5]: solar nadi
/// resource[6]: opo-opo fury
/// resource[7]: raptor fury
/// resource[8]: coeurl fury
///
/// ## Skill Usage
/// 3 of opo-opo chakra with lunar-solar nadi -> phantom rush
/// 3 of opo-opo chakra -> elixir burst
/// 1, 1, 1 of each chakra -> rising pheonix
#[derive(Clone)]
pub(crate) struct MonkCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    resources: [ResourceType; MONK_STACKS_COUNT],
}

impl CombatResource for MonkCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            MONK_MAX_STACKS[resource_id],
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
        skill_id: SkillIdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        if MONK_COMBO2_COMBO3_SKILL_IDS.contains(&skill_id) {
            self.resources[FIRES_REPLY_STACK_ID as usize] = 0;
        }

        (vec![], vec![])
    }

    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_other_time_related_states(&mut self, _: TimeType) {}
    fn trigger_on_gcd_crit(&mut self) {
        self.add_resource(CHAKRA_STACK_ID, 1)
    }

    fn get_combo_remaining_time(&self) -> TimeType {
        0
    }
}

impl MonkCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_monk_skill_list(player_id),
            current_combo: None,
            resources: [5, 0, 0, 0, -1, 0, 0, 0, 0, 1],
        }
    }
}
