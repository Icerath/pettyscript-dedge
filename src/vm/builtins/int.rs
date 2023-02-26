use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};
use std::{any::Any, fmt};

use super::{
    function_template::{BinOpTemplate, SingleTemplate},
    PtyBool, PtyStr,
};

#[derive(Clone, Copy)]
pub struct PtyInt(pub i128);
impl PettyObjectType for PtyInt {
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__add__" => BinOpTemplate::<Self>(|left, right| Self(left.0 + right.0)).into(),
            "__sub__" => BinOpTemplate::<Self>(|left, right| Self(left.0 - right.0)).into(),
            "__mul__" => BinOpTemplate::<Self>(|left, right| Self(left.0 * right.0)).into(),
            "__div__" => BinOpTemplate::<Self>(|left, right| Self(left.0 / right.0)).into(),
            "__bool__" => SingleTemplate::<Self, PtyBool>(|this: Self| PtyBool(this.0 != 0)).into(),
            "__repr__" => {
                SingleTemplate::<Self, PtyStr>(|this| PtyStr(this.to_string().into())).into()
            }
            "abs" => SingleTemplate::<Self, Self>(|this| Self(this.0.abs())).into(),
            _ => todo!("{str}"),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl fmt::Display for PtyInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}