use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::pictomancer::abilities::make_pictomancer_skill_list;
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

const PICTOMANCER_STACK_COUNT: usize = 11;

const PALLETE_STACK_MAX: ResourceType = 100;
const HAMMER_STACK_MAX: ResourceType = 3;
const HARD_GCD_STACK_MAX: ResourceType = 3;

const HYPERPHANTASIA_STACK_MAX: ResourceType = 5;
const CREATURE_STACK_MAX: ResourceType = 4;
const SHOT_STACK_MAX: ResourceType = 2;

pub(crate) const HAMMER_STACK_ID: ResourceIdType = 1;
pub(crate) const STARRY_SKY_STACK_ID: ResourceIdType = 2;
/// Up whenever using creature muse
pub(crate) const SHOT_STACK_ID: ResourceIdType = 3;
pub(crate) const BLACK_PAINT_STACK_ID: ResourceIdType = 4;
/// CYM stack
pub(crate) const HARD_GCD_STACK_ID: ResourceIdType = 5;
/// Mog of the ages used flag
pub(crate) const SHOT_MOOGLE_ID: ResourceIdType = 6;
pub(crate) const HAMMER_READY_ID: ResourceIdType = 7;
pub(crate) const HYPERPHANTASIA_STACK_ID: ResourceIdType = 8;

/// Already has creature muse
pub(crate) const HAS_CREATURE_ID: ResourceIdType = 9;

/// 0, 4: creature, 1: wing, 2: maw, 3: claw
pub(crate) const CREATURE_STACK_ID: ResourceIdType = 10;

const PICTOMANCER_MAX_STACKS: [ResourceType; PICTOMANCER_STACK_COUNT] = [
    PALLETE_STACK_MAX,
    HAMMER_STACK_MAX,
    1,
    SHOT_STACK_MAX,
    1,
    HARD_GCD_STACK_MAX,
    1,
    1,
    HYPERPHANTASIA_STACK_MAX,
    1,
    CREATURE_STACK_MAX,
];

/// Pictomancer Combat Resources Mechanism
///
/// # 1. Resource Explanation
/// - resource[0]: Pallete stack
/// - resource[1]: Hammer stack - 3 stacks when striking muse is used, and a hammer skill uses one stack each.
/// - resource[2]: Starry sky stack - 1 stack when starry sky motif is used, and starry muse skill uses this stack.
///
/// - resource[3]: Shot stack - used to determine if mog of the ages/retribution of the madeem can be used. 1 stack increases every time creature muse is used. When there's 2 stacks of this stack, mog of the ages or retribution of the madeem can be used.
/// - resource[4]: Black paint stack
/// - resource[5]: Hard GCD stack - CYM uses this stack.
/// - resource[6]: Shot moogle - 1 stack when mog of the ages is used. Used to determine whether it is claw/maw motif to be drawn or pom/winged motif to be drawn.
///
/// - resource[7]: Hammer ready - 1 stack when hammer motif is used. Striking muse consumes this stack.
/// - resource[8]: Hyperphantasia stack
/// - resource[9]: Has creature - 1 stack when creature motif is used. Used to tell the PCT there is already a creature drawn so you can't draw any more creatures.
///
/// **retribution of the madeem uses all shot moogle and shot stack to reset the creature related stacks**
#[derive(Clone)]
pub(crate) struct PictomancerCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,

    resources: [ResourceType; PICTOMANCER_STACK_COUNT],
}

impl CombatResource for PictomancerCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            PICTOMANCER_MAX_STACKS[resource_id],
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

    fn trigger_on_gcd_crit(&mut self) {}
    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }

    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl PictomancerCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_pictomancer_skill_list(player_id),
            current_combo: None,
            resources: [0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0],
        }
    }
}
