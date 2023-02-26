use super::{builtins::PtyNull, object::PettyObject};
use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

#[derive(Default)]
pub struct FieldDict {
    globals: HashMap<String, PettyObject>,
    scopes: Vec<HashMap<String, PettyObject>>,
}

impl FieldDict {
    pub fn write(&mut self, str: &str, value: PettyObject) {
        self.current_scope().insert(str.into(), value);
    }
    pub fn read(&mut self, str: &str) -> PettyObject {
        self.current_scope().get(str).unwrap_or_else(|| panic!("Not found: {str}")).clone()
    }
    fn current_scope(&mut self) -> &mut HashMap<String, PettyObject> {
        self.scopes.last_mut().unwrap_or(&mut self.globals)
    }
    pub fn new_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn drop_scope(&mut self) {
        self.scopes.pop();
    }
}
