use crate::id_entity::IdEntity;
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::AttackSkill;
use crate::IdType;

pub(crate) mod dragoon_abilities;
pub(crate) mod ninja_abilities;
pub(crate) mod sage_abilities;

pub(crate) fn make_opener(
    player_id: IdType,
    mut opener: Vec<Option<AttackSkill>>,
) -> Vec<Option<AttackSkill>> {
    for skill in opener.iter_mut() {
        if let Some(skill) = skill {
            skill.player_id = player_id;
        }
    }

    opener
}

pub(crate) fn make_skill_table(
    player_id: IdType,
    mut skill_list: Vec<AttackSkill>,
) -> SkillTable<AttackSkill> {
    for skill in skill_list.iter_mut() {
        skill.player_id = player_id;
    }

    skill_list
        .iter()
        .map(|skill| (skill.id, skill.clone()))
        .collect()
}
