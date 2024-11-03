use crate::combat_resources::ffxiv_combat_resources::COMBO_MAX_TIME_LEFT_MILLISECOND;
use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::reaper::abilities::make_reaper_skill_list;
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
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

lazy_static! {
    static ref REAPER_NORMAL_GCD_IDS: Vec<SkillIdType> = vec![1200, 1201, 1202];
}

const REAPER_STACK_COUNT: usize = 6;

const ENSHROUD_COUNT_ID: ResourceIdType = 6;

const SOUL_GAUGE_MAX: ResourceType = 100;
const ENSHROUD_GAUGE_MAX: ResourceType = 100;
const SOUL_REAVER_MAX: ResourceType = 1;
const ENSHROUD_STACK_MAX: ResourceType = 5;
const LEMURES_STACK_MAX: ResourceType = 4;
const EXECUTIONER_STACK_MAX: ResourceType = 2;

const REAPER_MAX_STACKS: [ResourceType; REAPER_STACK_COUNT] = [
    SOUL_GAUGE_MAX,
    ENSHROUD_GAUGE_MAX,
    SOUL_REAVER_MAX,
    ENSHROUD_STACK_MAX,
    LEMURES_STACK_MAX,
    EXECUTIONER_STACK_MAX,
];

/// Reaper Combat Resources Mechanism
///
/// # 1. Resource Explanation
/// - resource[0]: soul gauge
/// - resource[1]: enshroud gauge
/// - resource[2]: soul reaver stack - gets one stack when blood stalk is used. Gallows or Gibbet comsumes the stack
/// - resource[3]: enshroud stack - gets five stacks when enshroud is used. Cross/Void Reaping + Communio consumes the stack
/// - resource[4]: lemures stack - gets one stacks when Cross/Void reaping is used. Lemure's slice consumes 2 stacks
/// - resource[5]: executioner stack - gets two stacks when gluttony is used. Executioner's Gallows/Gibbet consumes the stack
#[derive(Clone)]
pub(crate) struct ReaperCombatResources {
    skills: SkillTable<AttackSkill>,

    #[allow(unused)]
    player_id: PlayerIdType,
    current_combo: ComboType,
    resources: [ResourceType; REAPER_STACK_COUNT],
    combo_time_left_millisecond: TimeType,
}

impl CombatResource for ReaperCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            REAPER_MAX_STACKS[resource_id],
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
            self.combo_time_left_millisecond = if *combo_id == 1 {
                TimeType::MAX
            } else {
                COMBO_MAX_TIME_LEFT_MILLISECOND
            };

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
        if REAPER_NORMAL_GCD_IDS.contains(&skill_id) {
            self.resources[ENSHROUD_COUNT_ID as usize] = 0;
        }

        (vec![], vec![])
    }

    fn trigger_on_gcd_crit(&mut self) {}

    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_other_time_related_states(&mut self, elapsed_time_millisecond: TimeType) {
        self.combo_time_left_millisecond = max(
            self.combo_time_left_millisecond - elapsed_time_millisecond,
            0,
        );

        if self.combo_time_left_millisecond == 0 {
            self.current_combo = None;
        }
    }

    fn get_combo_remaining_time(&self) -> TimeType {
        self.combo_time_left_millisecond
    }
}

impl ReaperCombatResources {
    pub(crate) fn new(player_id: PlayerIdType, player_count: usize) -> Self {
        Self {
            skills: make_reaper_skill_list(player_id, player_count),
            player_id,
            current_combo: None,
            resources: [0; REAPER_STACK_COUNT],
            combo_time_left_millisecond: COMBO_MAX_TIME_LEFT_MILLISECOND,
        }
    }
}
