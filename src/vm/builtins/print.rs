use std::fmt;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::{display_function_object, PtyNull};

#[derive(Clone)]
pub struct PtyPrint;
impl PettyObjectType for PtyPrint {
    fn call(&self, vm: &mut Vm, _this: PettyObject, args: FuncArgs) -> PettyObject {
        for arg in args.0 {
            let repr_function = arg.get_item(vm, arg.clone(), "__repr__");
            let repr_object = repr_function.call(vm, repr_function.clone(), FuncArgs(vec![arg]));
            println!("{repr_object}");
        }
        PtyNull.into()
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, _str: &str) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl fmt::Display for PtyPrint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_function_object(self, f)
    }
}
