use super::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};
use std::fmt;

pub type RawFn = fn(vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject;
#[derive(Clone)]
pub struct RawFunction(pub RawFn);

impl PettyObjectType for RawFunction {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn call(&self, vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
        self.0(vm, this, args)
    }
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject {
        todo!()
    }
}

impl fmt::Display for RawFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
