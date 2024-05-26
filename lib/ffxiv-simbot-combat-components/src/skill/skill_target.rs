use crate::IdType;

#[derive(Clone, Debug)]
pub enum SkillTarget {
    Target,
    Player(IdType),
}
