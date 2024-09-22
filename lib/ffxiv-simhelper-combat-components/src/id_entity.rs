use crate::types::SkillIdType;

pub trait IdEntity {
    fn get_id(&self) -> SkillIdType;
}
