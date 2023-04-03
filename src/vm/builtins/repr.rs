use std::fmt;

use crate::vm::{
    builtins::PtyNull,
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
        let Some(repr) = args.0[0].repr(vm) else {
            eprintln!("TODO - {}", &args.0[0]);
            return PtyNull.into();
        };
        repr.into()
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
