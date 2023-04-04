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
    fn call(&self, vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject {
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

fn unwrap(vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
    let Some(self_) = args.0.get(0) else {
        todo!()
    };
    let Some(self_) = self_.as_any().downcast_ref::<PtyOption>() else {
        todo!()
    };
    self_.0.clone().unwrap()
}

fn __repr__(vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
    let Some(self_) = args.0[0].as_any().downcast_ref::<PtyOption>() else {
        todo!()
    };
    let Some(inner) = &self_.0 else {
        return PtyStr("None".into()).into()
    };
    let repr = inner.repr(vm).unwrap();
    PtyStr(format!("Some({})", repr.0).into()).into()
}
#[pettymethod]
pub fn some(obj: PettyObject) -> PtyOption {
    PtyOption(Some(obj))
}

#[pettymethod]
pub fn none() -> PtyOption {
    PtyOption(None)
}
