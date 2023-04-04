use std::fmt;

use macros::pettymethod;

use super::string::PtyStr;
use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
    raw_function::RawFunction,
};

#[derive(Clone)]
pub struct PtyOption(pub Option<PettyObject>);

impl PettyObjectType for PtyOption {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "unwrap" => RawFunction(unwrap).into(),
            "__repr__" => RawFunction(__repr__).into(),
            _ => todo!(),
        }
    }
}

impl fmt::Display for PtyOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(obj) => write!(f, "Some({obj})"),
            None => write!(f, "None"),
        }
    }
}

#[pettymethod]
fn unwrap(opt: PtyOption) -> PettyObject {
    opt.0.unwrap()
}

#[pettymethod]
fn __repr__(self_: PtyOption, vm: &mut Vm) -> PtyStr {
    match self_.0 {
        Some(obj) => obj.force_repr(vm),
        None => PtyStr("None".into()),
    }
}

#[pettymethod]
pub fn some(obj: PettyObject) -> PtyOption {
    PtyOption(Some(obj))
}

#[pettymethod]
pub fn none() -> PtyOption {
    PtyOption(None)
}
