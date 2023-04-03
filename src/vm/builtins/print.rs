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
            if let Some(repr) = arg.repr(vm) {
                println!("{}", repr.0);
            } else {
                eprintln!("TODO - {arg}"); // TODO
            }
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
