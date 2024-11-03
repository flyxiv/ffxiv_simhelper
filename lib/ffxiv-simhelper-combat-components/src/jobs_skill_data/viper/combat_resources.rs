use crate::combat_resources::ffxiv_combat_resources::COMBO_MAX_TIME_LEFT_MILLISECOND;
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
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const VIPER_STACK_COUNT: usize = 10;

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
    1,
];

/// Viper Combat Resources Mechanism
///
/// # 0. Term Explanation
/// - Normal Filler: **The 170 potency fillers (Twinfang/Twinblood bite, Uncoiled Twinfang/Uncoiled Twinblood)**
///                  These fillers are merged to one skill for performance reasons.
///
/// # 1. Resource Explanation
/// - resource[0]: Serpent Offerings
/// - resource[1]: Normal Filler 1 Flag. Increases when using skills that generate normal fillers: Uncoiled Fury/Hunter's Coil/Swiftskin's Coil
/// - resource[2]: Normal Filler 2 Flag. Increases when using skills that generate normal fillers: Uncoiled Fury/Hunter's Coil/Swiftskin's Coil
/// - resource[3]: Reawaken Filler Flag. Increases when Reawaken combos are used.
/// - resource[4]: Rattling Coil
/// - resource[5]: Hunter's coil Flag. Increases when Vicewinder is used, and consumed when using Hunter's Coil.
/// - resource[6]: Swiftskin's Coil Flag. Increases when Vicewinder is used, and consumed when using Swiftskin's Coil.
/// - resource[7]: Reawaken Combo Stack. Increases by 5 Reawaken is used, and Reawaken combos consume the stack.
/// - resource[8]: Death Rattle Flag.
/// - resource[9]: Swiftskin/Hunter's sting flag. Used to interweave Swiftskin and Hunter's sting. Swiftskin generates the stack, and Hunter's sting consumes it.
#[derive(Clone)]
pub(crate) struct ViperCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,

    resources: [ResourceType; VIPER_STACK_COUNT],
    combo_time_left_millisecond: TimeType,
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
    fn update_other_time_related_states(&mut self, time_elapsed_millisecond: TimeType) {
        self.combo_time_left_millisecond = max(
            self.combo_time_left_millisecond - time_elapsed_millisecond,
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

impl ViperCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_viper_skill_list(player_id),
            current_combo: None,

            resources: [0; VIPER_STACK_COUNT],
            combo_time_left_millisecond: COMBO_MAX_TIME_LEFT_MILLISECOND,
        }
    }
}
