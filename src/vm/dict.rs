use super::object::PettyObject;
use crate::slim_rc::Rc;
use std::collections::BTreeMap;
pub type Dict = BTreeMap<Rc<str>, PettyObject>;
