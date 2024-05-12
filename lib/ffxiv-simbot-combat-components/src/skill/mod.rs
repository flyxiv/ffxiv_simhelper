pub mod attack_skill;
pub(crate) mod job_abilities;

use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, ResourceType, TimeType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// The normal delay time for o-GCD skills.
/// After using 1 oGCD, the player cannot use another skill for 0.7 seconds.
pub static NON_GCD_DELAY_MILLISECOND: i32 = 700;

/// The resource requirements for a skill.
/// Skill might need mana, status(suiton status is needed for Trick Attack), or combo status.
#[derive(Clone)]
pub(crate) enum ResourceRequirements {
    Mana(ResourceType),
    UseBuff(IdType),
    UseDebuff(IdType),
    CheckStatus(IdType),
    Resource(IdType, ResourceType),
}

pub(crate) type SkillEvents = (Vec<FfxivEvent>, Vec<FfxivPlayerInternalEvent>);

pub type ResourceTable = HashMap<IdType, ResourceType>;

/// a single FFXIV skill.
pub trait Skill: Sized + Clone + IdEntity {
    fn start_cooldown(&mut self);
    fn generate_skill_events(
        &self,
        buffs: Rc<RefCell<Vec<BuffStatus>>>,
        debuffs: Rc<RefCell<Vec<DebuffStatus>>>,
        current_combat_time_milliseconds: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents;
}
