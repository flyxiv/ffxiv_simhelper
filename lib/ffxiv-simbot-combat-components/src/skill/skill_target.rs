use crate::types::SkillIdType;

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum SkillTarget {
    Target,
    Player(SkillIdType),
}
