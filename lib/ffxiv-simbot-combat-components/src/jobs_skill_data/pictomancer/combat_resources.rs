use crate::combat_resources::CombatResource;
use crate::jobs_skill_data::pictomancer::abilities::make_pictomancer_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, ResourceType};
use crate::types::{IdType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const PALLETE_STACK_MAX: ResourceType = 100;
const HAMMER_STACK_MAX: ResourceType = 3;
const HARD_GCD_STACK_MAX: ResourceType = 3;

const HYPERPHANTASIA_STACK_MAX: ResourceType = 5;
const CREATURE_STACK_MAX: ResourceType = 4;
const SHOT_STACK_MAX: ResourceType = 2;

pub(crate) const HAMMER_STACK_ID: IdType = 1;
pub(crate) const STARRY_SKY_STACK_ID: IdType = 2;
pub(crate) const SHOT_STACK_ID: IdType = 3;
pub(crate) const BLACK_PAINT_STACK_ID: IdType = 7;
pub(crate) const HARD_GCD_STACK_ID: IdType = 8;
pub(crate) const SHOT_MOOGLE_ID: IdType = 11;
pub(crate) const HAMMER_READY_ID: IdType = 12;
pub(crate) const HYPERPHANTASIA_STACK_ID: IdType = 13;
pub(crate) const HAS_CREATURE_ID: IdType = 14;
pub(crate) const CREATURE_STACK_ID: IdType = 15;

#[derive(Clone)]
pub(crate) struct PictomancerCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,

    pallete_stack: ResourceType,
    hammer_stack: ResourceType,
    starry_sky_stack: ResourceType,
    shot_stack: ResourceType,
    black_paint_stack: ResourceType,
    hard_gcd_stack: ResourceType,
    shot_moogle: ResourceType,
    hammer_ready: ResourceType,
    hyperphantasia_stack: ResourceType,
    has_creature: ResourceType,
    creature_stack: ResourceType,
}

impl CombatResource for PictomancerCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.pallete_stack = min(PALLETE_STACK_MAX, self.pallete_stack + resource_amount);
        } else if resource_id == HAMMER_STACK_ID {
            self.hammer_stack = min(HAMMER_STACK_MAX, self.hammer_stack + resource_amount);
        } else if resource_id == STARRY_SKY_STACK_ID {
            self.starry_sky_stack = min(1, self.starry_sky_stack + resource_amount);
        } else if resource_id == SHOT_STACK_ID {
            self.shot_stack = min(SHOT_STACK_MAX, self.shot_stack + resource_amount);
        } else if resource_id == BLACK_PAINT_STACK_ID {
            self.black_paint_stack = min(1, self.black_paint_stack + resource_amount);
        } else if resource_id == HARD_GCD_STACK_ID {
            self.hard_gcd_stack = min(HARD_GCD_STACK_MAX, self.hard_gcd_stack + resource_amount);
        } else if resource_id == SHOT_MOOGLE_ID {
            self.shot_moogle = min(1, self.shot_moogle + resource_amount);
        } else if resource_id == HAMMER_READY_ID {
            self.hammer_ready = min(1, self.hammer_ready + resource_amount);
        } else if resource_id == HYPERPHANTASIA_STACK_ID {
            self.hyperphantasia_stack = min(
                HYPERPHANTASIA_STACK_MAX,
                self.hyperphantasia_stack + resource_amount,
            );
        } else if resource_id == HAS_CREATURE_ID {
            self.has_creature = min(1, self.has_creature + resource_amount);
        } else if resource_id == CREATURE_STACK_ID {
            self.creature_stack = min(CREATURE_STACK_MAX, self.creature_stack + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.pallete_stack
        } else if resource_id == HAMMER_STACK_ID {
            self.hammer_stack
        } else if resource_id == STARRY_SKY_STACK_ID {
            self.starry_sky_stack
        } else if resource_id == SHOT_STACK_ID {
            self.shot_stack
        } else if resource_id == BLACK_PAINT_STACK_ID {
            self.black_paint_stack
        } else if resource_id == HARD_GCD_STACK_ID {
            self.hard_gcd_stack
        } else if resource_id == SHOT_MOOGLE_ID {
            self.shot_moogle
        } else if resource_id == HAMMER_READY_ID {
            self.hammer_ready
        } else if resource_id == HYPERPHANTASIA_STACK_ID {
            self.hyperphantasia_stack
        } else if resource_id == HAS_CREATURE_ID {
            self.has_creature
        } else if resource_id == CREATURE_STACK_ID {
            self.creature_stack
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &Option<IdType>) {
        if let Some(combo_id) = combo {
            self.current_combo = Some(*combo_id);
        }
    }

    fn trigger_on_event(
        &mut self,
        _: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        (vec![], vec![])
    }

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}

    fn trigger_on_crit(&mut self) {}
}

impl PictomancerCombatResources {
    pub(crate) fn new(player_id: IdType) -> Self {
        Self {
            player_id,
            skills: make_pictomancer_skill_list(player_id),
            current_combo: None,
            pallete_stack: 0,
            hammer_stack: 0,
            starry_sky_stack: 1,
            shot_stack: 0,
            black_paint_stack: 0,
            hard_gcd_stack: 0,
            shot_moogle: 0,
            hammer_ready: 1,
            hyperphantasia_stack: 0,
            has_creature: 1,
            creature_stack: 0,
        }
    }
}
