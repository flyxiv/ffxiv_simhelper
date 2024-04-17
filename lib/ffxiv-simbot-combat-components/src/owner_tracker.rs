use crate::IdType;

pub trait OwnerTracker {
    fn get_owner_id(&self) -> IdType;
}
