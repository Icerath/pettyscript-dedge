use super::PtyStr;
use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};
use macros::pettymethod;
use std::fmt;

#[derive(Clone, Copy)]
pub struct PtyBool(pub bool);
impl PettyObjectType for PtyBool {
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__bool__" => __BOOL__.clone(),
            "__not__" => __NOT__.clone(),
            "__and__" => __AND__.clone(),
            "__or__" => __OR__.clone(),
            "__repr__" => __REPR__.clone(),
            _ => todo!("{str}"),
        }
    }
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PtyBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[pettymethod]
fn __bool__(self_: PtyBool) -> PtyBool {
    self_
}
#[pettymethod]
fn __not__(self_: PtyBool) -> PtyBool {
    PtyBool(!self_.0)
}
#[pettymethod]
fn __and__(lhs: PtyBool, rhs: PtyBool) -> PtyBool {
    PtyBool(lhs.0 && rhs.0)
}
#[pettymethod]
fn __or__(lhs: PtyBool, rhs: PtyBool) -> PtyBool {
    PtyBool(lhs.0 || rhs.0)
}
#[allow(clippy::match_bool)]
#[pettymethod]
fn __repr__(self_: PtyBool) -> PtyStr {
    PtyStr(
        match self_.0 {
            true => "true",
            false => "false",
        }
        .into(),
    )
}
