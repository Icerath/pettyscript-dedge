use super::object::PettyObject;
pub struct FuncArgs<'a>(pub &'a [&'a PettyObject]);
