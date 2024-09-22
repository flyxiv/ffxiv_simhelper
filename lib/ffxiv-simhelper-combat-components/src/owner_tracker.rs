use crate::types::PlayerIdType;

pub trait OwnerTracker {
    fn get_owner_id(&self) -> PlayerIdType;
}
