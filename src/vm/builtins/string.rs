use std::fmt;

use crate::{
    rc_str::RcStr,
    vm::{
        core::Vm,
        function_args::FuncArgs,
        object::{PettyObject, PettyObjectType},
    },
};

use super::function_template::{BinOpTemplate, SingleTemplate};

#[derive(Clone)]
pub struct PtyStr(pub RcStr);

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
            "__repr__" => SingleTemplate(|this: &Self| this.clone()).into(),
            "__add__" => {
                BinOpTemplate(|lhs: Self, rhs: Self| Self((lhs.0.to_string() + &rhs.0).into()))
                    .into()
            }
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
