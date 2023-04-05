use std::sync::Arc;

use super::{dict::Dict, object::PettyObject};

#[derive(Default)]
pub struct FieldDict {
    globals: Dict,
    scopes: Vec<Dict>,
}

impl FieldDict {
    pub fn write(&mut self, str: Arc<str>, value: PettyObject) {
        self.current_scope().insert(str, value);
    }
    pub fn read(&mut self, str: &str) -> PettyObject {
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
    fn current_scope(&mut self) -> &mut Dict {
        self.scopes.last_mut().unwrap_or(&mut self.globals)
    }
    pub fn new_scope(&mut self) {
        self.scopes.push(Dict::new());
    }
    pub fn drop_scope(&mut self) {
        self.scopes.pop();
    }
}
