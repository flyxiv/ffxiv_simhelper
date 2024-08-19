use crate::event::ffxiv_event::FfxivEvent;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{ResourceRequirements, ResourceTable};
use crate::types::PlayerIdType;

pub(crate) struct FirstSkillCombatSimulation<'a> {
    pub(crate) player_id: PlayerIdType,
    pub(crate) resource_produced: &'a ResourceTable,
    pub(crate) skill_events: &'a Vec<FfxivEvent>,
    pub(crate) skill_used_resource: &'a Vec<ResourceRequirements>,
}

impl FirstSkillCombatSimulation<'_> {
    pub(crate) fn new(
        player_id: PlayerIdType,
        attack_skill: &AttackSkill,
    ) -> FirstSkillCombatSimulation {
        FirstSkillCombatSimulation {
            player_id,
            resource_produced: &attack_skill.resource_created,
            skill_events: &attack_skill.additional_skill_events,
            skill_used_resource: &attack_skill.resource_required,
        }
    }
}
