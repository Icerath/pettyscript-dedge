use std::fmt;

use macros::pettymethod;

use crate::vm::{
    builtins::PtyNull,
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::{display_function_object, PtyStr};
#[pettymethod]
pub fn repr(obj: PettyObject, vm: &mut Vm) -> PtyStr {
    obj.repr(vm).unwrap()
}
