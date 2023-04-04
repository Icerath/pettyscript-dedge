use std::fmt;

use macros::pettymethod;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
    raw_function::RawFunction,
};

use super::{PtyBool, PtyStr};

#[derive(Clone, Copy)]
pub struct PtyNull;
impl PettyObjectType for PtyNull {
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__bool__" => RawFunction(__bool__).into(),
            "__repr__" => RawFunction(__repr__).into(),
            _ => todo!(),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl fmt::Display for PtyNull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "null")
    }
}

#[pettymethod]
fn __bool__(_self: PtyNull) -> PtyBool {
    PtyBool(false)
}
#[pettymethod]
fn __repr__(_self: PtyNull) -> PtyStr {
    PtyStr("null".into())
}

impl From<()> for PtyNull {
    fn from(value: ()) -> Self {
        PtyNull
    }
}
