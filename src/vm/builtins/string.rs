use std::fmt;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::function_template::{BinOpTemplate, SingleTemplate};

#[derive(Clone)]
pub struct PtyStr(pub Box<str>);

impl PtyStr {
    pub fn from_obj<PtyObj: PettyObjectType>(object: &PtyObj) -> Self {
        Self(object.to_string().into())
    }
}

impl PettyObjectType for PtyStr {
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!("String is not Callable")
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__repr__" => SingleTemplate(|this: Self| this).into(),
            "__add__" => BinOpTemplate(|lhs: Self, rhs: Self| {
                Self((lhs.0.to_string() + &rhs.0).into_boxed_str())
            })
            .into(),
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
