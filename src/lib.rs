use std::any::TypeId;

pub trait Grafted {
    type Local;
    
    fn foreign_type_name() -> &'static str;
    fn foreign_type_id() -> TypeId;
}
