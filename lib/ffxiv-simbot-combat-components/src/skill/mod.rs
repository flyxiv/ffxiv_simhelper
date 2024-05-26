pub mod attack_skill;
pub(crate) mod skill_target;
pub(crate) mod use_type;

use crate::event::ffxiv_event::FfxivEvent;
use crate::event::ffxiv_player_internal_event::FfxivPlayerInternalEvent;
use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, ResourceType, StatusTable, TimeType};
use std::collections::HashMap;

/// The normal delay time for o-GCD skills.
/// After using 1 oGCD, the player cannot use another skill for 0.7 seconds.
pub static NON_GCD_DELAY_MILLISECOND: i32 = 700;
pub static GCD_DEFAULT_DELAY_MILLISECOND: i32 = 2500;

pub static DEFAULT_AUTO_ATTACK_COOLDOWN_MILLISECOND: TimeType = 2500;
pub static AUTO_ATTACK_ID: IdType = 10000;

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

pub(crate) fn make_skill_table(mut skill_list: Vec<AttackSkill>) -> SkillTable<AttackSkill> {
    skill_list
        .iter()
        .map(|skill| (skill.id, skill.clone()))
        .collect()
}

pub(crate) type SkillEvents = (Vec<FfxivEvent>, Vec<FfxivPlayerInternalEvent>);

pub type ResourceTable = HashMap<IdType, ResourceType>;

/// a single FFXIV skill.
pub trait Skill: Sized + Clone + IdEntity {
    fn start_cooldown(&mut self);
    fn generate_skill_events(
        &self,
        buffs: StatusTable<BuffStatus>,
        debuffs: StatusTable<DebuffStatus>,
        current_combat_time_milliseconds: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents;
}
