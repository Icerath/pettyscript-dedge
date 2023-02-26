use std::fmt;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::{function_template, PtyNull};

#[derive(Clone)]
pub struct PtyPrint;
impl PettyObjectType for PtyPrint {
    fn call(&self, vm: &mut Vm, _this: PettyObject, args: FuncArgs) -> PettyObject {
        for arg in args.0 {
            let repr_object = arg.get_item(vm, arg.clone(), "__repr__");
            let string = repr_object
                .call(vm, repr_object.clone(), FuncArgs(vec![arg]))
                .to_string();
            println!("{string:?}");
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
        function_template::display_function_object(self, f)
    }
}
