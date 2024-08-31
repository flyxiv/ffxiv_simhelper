use crate::types::IdType;

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum SkillTarget {
    Target,
    Player(IdType),
}
