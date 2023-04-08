use std::fmt;

use macros::pettymethod;
use once_cell::sync::Lazy;

use crate::vm::{
    builtins::pty_bool::FALSE,
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::PtyStr;

pub static NULL: Lazy<PettyObject> = Lazy::new(|| PtyNull.into());
pub static NULL_STR: Lazy<PettyObject> = Lazy::new(|| PtyStr("null".into()).into());

#[derive(Clone, Copy)]
pub struct PtyNull;
impl PettyObjectType for PtyNull {
    fn call(&self, _vm: &Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, _vm: &Vm, _this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "__bool__" => __BOOL__.clone(),
            "__repr__" => __REPR__.clone(),
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
fn __bool__(_self: PtyNull) -> PettyObject {
    FALSE.clone()
}

#[pettymethod]
fn __repr__(_self: PtyNull) -> PettyObject {
    NULL_STR.clone()
}
