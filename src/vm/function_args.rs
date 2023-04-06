use super::object::PettyObject;

pub struct FuncArgs<'a>(pub Vec<&'a PettyObject>);
