use crate::slim_rc::Rc;
use macros::pettymethod;
use std::fmt;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
    raw_function::RawFunction,
};

#[derive(Clone)]
pub struct PtyStr(pub Rc<str>);

impl PettyObjectType for PtyStr {
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!("String is not Callable")
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__repr__" => RawFunction(__repr__).into(),
            "__add__" => RawFunction(__add__).into(),
            _ => todo!(),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PtyStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[pettymethod]
fn __repr__(self_: PtyStr) -> PtyStr {
    self_
}
#[pettymethod]
fn __add__(lhs: PtyStr, rhs: PtyStr) -> PtyStr {
    PtyStr((lhs.0.to_string() + &rhs.0).into())
}
