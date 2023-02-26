use std::fmt;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

pub struct PtyStr(Box<str>);
impl fmt::Display for PtyStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
impl PettyObjectType for PtyStr {
    fn call(&self, vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__repr__" => self.repr(this),
            _ => todo!(),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl PtyStr {
    pub fn repr(&self, this: PettyObject) -> PettyObject {
        this
    }
}
