use crate::combat_resources::CombatResource;
use crate::event::FfxivEventQueue;
use crate::jobs_skill_data::summoner::abilities::make_summoner_skill_list;
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

const SUMMONER_STACK_COUNT: usize = 6;

const ENERGY_MAX_STACK: ResourceType = 2;
const IFRIT_MAX_STACK: ResourceType = 1;
const TITAN_MAX_STACK: ResourceType = 1;
const GARUDA_MAX_STACK: ResourceType = 1;
const TRANCE_MAX_STACK: ResourceType = 1;
const SOLAR_BAHAMUT_MAX_STACK: ResourceType = 1;

const SUMMONER_MAX_STACKS: [ResourceType; SUMMONER_STACK_COUNT] = [
    ENERGY_MAX_STACK,
    IFRIT_MAX_STACK,
    TITAN_MAX_STACK,
    GARUDA_MAX_STACK,
    TRANCE_MAX_STACK,
    SOLAR_BAHAMUT_MAX_STACK,
];

/// Summoner Combat Resources Mechanism
///
/// # 1. Resource Explanation
/// - resource[0]: Aether Stack used by Energy Drain
///
/// - resource[1]: Increases when Ifrit is summoned
/// - resource[2]: Increases when Titan is summoned
/// - resource[3]: Increases when Garuda is summoned
///
/// * Ifrit/Titan/Garuda stack is required for summoning Trance Skills such as Solar Bahamut/Bahamut/Pheonix
///
/// - resource[4] : Trance stack - Determines whether next non-solar bahamut trance is bahamut or pheonix.
///                                Stack is raised when Bahamut trance is used, and Pheonix trance uses the stack.
/// - resource[5] : Solar Bahamut stack - Determines whether next trance is Solar Bahamut's turn or Bahamut/Pheonix's turn.
///                                       Stack is raised when Solar Bahamut is used, and Bahamut/Pheonix trance uses the stack.
///
#[derive(Clone)]
pub(crate) struct SummonerCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    resources: [ResourceType; SUMMONER_STACK_COUNT],
}

impl CombatResource for SummonerCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            SUMMONER_MAX_STACKS[resource_id],
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

    fn trigger_on_gcd_crit(&mut self) {}

    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_other_time_related_states(&mut self, _: TimeType) {}
    fn get_combo_remaining_time(&self) -> TimeType {
        0
    }
}

impl SummonerCombatResources {
    pub(crate) fn new(
        player_id: PlayerIdType,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> Self {
        Self {
            skills: make_summoner_skill_list(player_id, ffxiv_event_queue),
            current_combo: None,
            resources: [0, 0, 0, 0, 0, 1],
        }
    }
}
