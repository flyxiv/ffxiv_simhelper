use crate::combat_resources::CombatResource;
use crate::consts::SIMULATION_START_TIME_MILLISECOND;
use crate::jobs_skill_data::black_mage::abilities::make_blackmage_skill_list;
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

const BLACK_MAGE_STACK_COUNT: usize = 5;

const POLYGLOT_STACK_INTERVAL_MILLISECOND: TimeType = 30000;
const POLYGLOT_MAX_STACK: ResourceType = 3;
const PARADOX_GAUGE_MAX_STACK: ResourceType = 1;
const FIRE4_FLARESTAR_MAX_STACK: ResourceType = 6;
const FLARESTAR_MAX_STACK: ResourceType = 1;
const CONVERSION_TO_BLIZZARD_MAX_STACK: ResourceType = 1;

const POLYGLOT_STACK_ID: ResourceIdType = 0;
const FIRE4_FLARESTAR_STACK_ID: ResourceIdType = 2;
const FLARESTAR_STACK_ID: ResourceIdType = 3;

const BLIZZARD_III_IDS: [SkillIdType; 3] = [1712, 1721, 1723];

const RESOURCE_MAX_STACKS: [ResourceType; BLACK_MAGE_STACK_COUNT] = [
    POLYGLOT_MAX_STACK,
    PARADOX_GAUGE_MAX_STACK,
    FIRE4_FLARESTAR_MAX_STACK,
    FLARESTAR_MAX_STACK,
    CONVERSION_TO_BLIZZARD_MAX_STACK,
];

#[derive(Clone)]
pub(crate) struct BlackmageCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    resources: [ResourceType; BLACK_MAGE_STACK_COUNT],
    next_polyglot_time: TimeType,
}

impl CombatResource for BlackmageCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;

        self.resources[resource_id] = min(
            RESOURCE_MAX_STACKS[resource_id],
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
        if BLIZZARD_III_IDS.contains(&skill_id) {
            self.resources[FIRE4_FLARESTAR_STACK_ID as usize] = 0;
            self.resources[FLARESTAR_STACK_ID as usize] = 0;
        }
        (vec![], vec![])
    }

    fn trigger_on_crit(&mut self) {}
    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }

    fn update_stack_timer(&mut self, elapsed_time_millisecond: TimeType) {
        if elapsed_time_millisecond >= self.next_polyglot_time {
            self.add_resource(POLYGLOT_STACK_ID, 1);
            self.next_polyglot_time += POLYGLOT_STACK_INTERVAL_MILLISECOND;
        }

        self.next_polyglot_time -= elapsed_time_millisecond;
    }
}

impl BlackmageCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_blackmage_skill_list(player_id),
            current_combo: None,
            resources: [0, 0, 0, 0, 1],
            next_polyglot_time: POLYGLOT_STACK_INTERVAL_MILLISECOND
                + TimeType::abs(SIMULATION_START_TIME_MILLISECOND),
        }
    }
}
