use crate::event::ffxiv_event::FfxivEvent;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::{ResourceRequirements, ResourceTable};
use crate::IdType;

pub(crate) struct FirstSkillCombatSimulation<'a> {
    pub(crate) player_id: IdType,
    pub(crate) resource_produced: &'a ResourceTable,
    pub(crate) skill_buffs: &'a Vec<FfxivEvent>,
    pub(crate) skill_debuffs: &'a Vec<FfxivEvent>,
    pub(crate) skill_used_resource: &'a Vec<ResourceRequirements>,
}

impl FirstSkillCombatSimulation<'_> {
    pub(crate) fn new(player_id: IdType, attack_skill: &AttackSkill) -> FirstSkillCombatSimulation {
        FirstSkillCombatSimulation {
            player_id,
            resource_produced: &attack_skill.resource_created,
            skill_buffs: &attack_skill.buff_events,
            skill_debuffs: &attack_skill.debuff_events,
            skill_used_resource: &attack_skill.resource_required,
        }
    }
}
