use super::object::PettyObject;
use std::{collections::BTreeMap, sync::Arc};
pub type Dict = BTreeMap<Arc<str>, PettyObject>;
