use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::vm::{
    core::Vm,
    dict::Dict,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

pub struct Module {
    pub name: Arc<str>,
    pub dict: Arc<Mutex<Dict>>,
}

impl PettyObjectType for Module {
    fn get_item(&self, vm: &mut Vm, this: &PettyObject, key: &str) -> PettyObject {
        let dict = self.dict.lock().unwrap();
        dict.get(key).unwrap().clone()
    }
    fn call(&self, vm: &mut Vm, this: &PettyObject, args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Module({})", self.name)
    }
}
