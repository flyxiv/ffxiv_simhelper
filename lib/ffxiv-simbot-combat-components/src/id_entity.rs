use crate::types::IdType;

pub trait IdEntity {
    fn get_id(&self) -> IdType;
}
