use super::object::PettyObject;
use std::collections::HashMap;

pub type Dict<'a> = HashMap<&'a str, PettyObject<'a>>;
