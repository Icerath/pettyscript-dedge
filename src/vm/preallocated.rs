use crate::vm::builtins::PtyNum;

use super::object::PettyObject;

pub struct PreAllocated {
    ints: [PettyObject; 256],
}
impl PreAllocated {
    pub fn new() -> Self {
        #[allow(clippy::cast_precision_loss)]
        let ints = std::array::from_fn(|i| PtyNum(i as f64).into());
        Self { ints }
    }
    #[inline]
    pub fn get(&self, int: usize) -> Option<PettyObject> {
        self.ints.get(int).cloned()
    }
}

impl Default for PreAllocated {
    fn default() -> Self {
        Self::new()
    }
}
