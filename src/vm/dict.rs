use super::object::PettyObject;
use hashbrown::HashMap;
use std::sync::Arc;
pub type Dict = HashMap<Arc<str>, PettyObject>;

#[inline]
pub fn insert_ref(dict: &mut Dict, key: &Arc<str>, value: PettyObject) {
    if let Some(val) = dict.get_mut(key) {
        *val = value;
    } else {
        dict.insert(key.clone(), value);
    }
}
