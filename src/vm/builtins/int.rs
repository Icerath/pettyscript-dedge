use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};
use std::{any::Any, fmt};

use super::{
    bool::PtyBool,
    function_template::{BinOpTemplate, SingleTemplate},
};

#[derive(Clone, Copy)]
pub struct PtyInt(i128);
impl PtyInt {
    pub fn new(int: i128) -> Self {
        Self(int)
    }
}
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
            "abs" => SingleTemplate::<Self, Self>(|this| Self(this.0.abs())).into(),
            _ => todo!(),
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
