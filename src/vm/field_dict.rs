use crate::rc_str::RcStr;

use super::object::PettyObject;
use std::{collections::BTreeMap};

#[derive(Default)]
pub struct FieldDict {
    globals: BTreeMap<RcStr, PettyObject>,
    scopes: Vec<BTreeMap<RcStr, PettyObject>>,
}

impl FieldDict {
    pub fn write(&mut self, str: RcStr, value: PettyObject) {
        self.current_scope().insert(str, value);
    }
    pub fn read(&mut self, str: &RcStr) -> PettyObject {
        for scope in self.scopes.iter().rev() {
            if let Some(object) = scope.get(str) {
                return object.clone();
            }
        }
        self.globals
            .get(str)
            .unwrap_or_else(|| panic!("Not found: {str}"))
            .clone()
    }
    fn current_scope(&mut self) -> &mut BTreeMap<RcStr, PettyObject> {
        self.scopes.last_mut().unwrap_or(&mut self.globals)
    }
    pub fn new_scope(&mut self) {
        self.scopes.push(BTreeMap::new());
    }
    pub fn drop_scope(&mut self) {
        self.scopes.pop();
    }
}
