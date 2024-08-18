use crate::types::IdType;

pub trait OwnerTracker {
    fn get_owner_id(&self) -> IdType;
}
