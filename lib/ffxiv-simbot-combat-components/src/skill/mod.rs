pub mod attack_skill;
pub mod damage_category;
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
use crate::types::{ResourceIdType, SkillIdType, TimeType};
use crate::types::{ResourceType, StatusTable};
use std::collections::HashMap;

/// The normal delay time for o-GCD skills.
/// After using 1 oGCD, the player cannot use another skill for 0.7 seconds.
pub static NON_GCD_DELAY_MILLISECOND: i32 = 670;
pub static GCD_DEFAULT_DELAY_MILLISECOND: i32 = 2500;

pub static DEFAULT_AUTO_ATTACK_COOLDOWN_MILLISECOND: TimeType = 2500;
pub static AUTO_ATTACK_ID: SkillIdType = 10001;

/// The resource requirements for a skill.
/// Skill might need mana, status(suiton status is needed for Trick Attack), or combo status.
#[derive(Clone)]
pub enum ResourceRequirements {
    UseBuff(SkillIdType),

    #[allow(unused)]
    UseDebuff(SkillIdType),

    CheckStatus(SkillIdType),
    Resource(ResourceIdType, ResourceType),
    UseAllResource(ResourceIdType),
}

pub(crate) fn make_skill_table(skill_list: Vec<AttackSkill>) -> SkillTable<AttackSkill> {
    skill_list
        .iter()
        .map(|skill| (skill.id, skill.clone()))
        .collect()
}

pub(crate) type SkillEvents = (Vec<FfxivEvent>, Vec<FfxivPlayerInternalEvent>);

pub type ResourceTable = HashMap<ResourceIdType, ResourceType>;

/// a single FFXIV skill.
pub trait Skill: Sized + Clone + IdEntity {
    fn start_cooldown(&mut self, player: &FfxivPlayer);
    fn generate_skill_events(
        &self,
        buffs: StatusTable<BuffStatus>,
        debuffs: StatusTable<DebuffStatus>,
        current_combat_time_milliseconds: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents;
}
