use std::fmt;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::display_function_object;

#[derive(Clone)]
pub struct PtyRepr;
impl PettyObjectType for PtyRepr {
    fn call(&self, vm: &mut Vm, _this: PettyObject, args: FuncArgs) -> PettyObject {
        if args.0.len() != 1 {
            todo!("Expected 1 argument got {}", args.0.len());
        }
        let arg = &args.0[0];
        let repr_function = arg.get_item(vm, arg.clone(), "__repr__");
        repr_function.call(vm, repr_function.clone(), FuncArgs(vec![arg.clone()]))
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, _str: &str) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl fmt::Display for PtyRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_function_object(self, f)
    }
}
