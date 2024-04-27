use crate::IdType;

pub trait IdEntity {
    fn get_id(&self) -> IdType;
}
