use crate::IdType;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RaidDamageTableKey {
    pub(crate) status_id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) skill_id: IdType,
}
