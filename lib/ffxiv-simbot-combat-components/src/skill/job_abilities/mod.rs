use crate::rotation::job_priorities::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::IdType;

//pub(crate) mod dragoon_abilities;
pub(crate) mod ninja_abilities;
pub(crate) mod sage_abilities;

pub(crate) fn make_skill_table(mut skill_list: Vec<AttackSkill>) -> SkillTable<AttackSkill> {
    skill_list
        .iter()
        .map(|skill| (skill.id, skill.clone()))
        .collect()
}
