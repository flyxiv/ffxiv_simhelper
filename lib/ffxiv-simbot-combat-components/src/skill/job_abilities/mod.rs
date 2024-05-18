use crate::rotation::job_priorities::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::IdType;

//pub(crate) mod dragoon_abilities;
pub(crate) mod ninja_abilities;
pub(crate) mod sage_abilities;

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
