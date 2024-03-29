use crate::vm::prelude::*;

pub struct Module {
    pub name: Arc<str>,
    pub dict: Arc<Mutex<Dict>>,
}

impl PettyObjectType for Module {
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, key: &str) -> PettyObject {
        let dict = self.dict.lock().unwrap();
        dict.get(key).unwrap().clone()
    }
    fn call(&self, _vm: &mut Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
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
