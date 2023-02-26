use super::{builtins::PtyNull, object::PettyObject};
use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

pub struct FieldDict(HashMap<String, PettyObject>, PettyObject);

impl FieldDict {
    pub fn new() -> Self {
        Self(HashMap::new(), PtyNull.into())
    }
    pub fn write(&mut self, str: &str, value: PettyObject) {
        self.0.insert(str.into(), value);
    }
    pub fn read(&self, str: &str) -> PettyObject {
        self.0.get(str).expect("Not found").clone()
    }
}

impl Index<&str> for FieldDict {
    type Output = PettyObject;
    fn index(&self, index: &str) -> &PettyObject {
        &self.1
    }
}
impl IndexMut<&str> for FieldDict {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        if !self.0.contains_key(index) {
            self.write(index, PtyNull.into());
        }
        self.0.get_mut(index).unwrap()
    }
}