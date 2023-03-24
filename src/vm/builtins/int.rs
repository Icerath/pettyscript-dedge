use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
    raw_function::RawFunction,
};
use macros::pettymethod;

use std::{any::Any, fmt};

use super::{PtyBool, PtyStr};

#[derive(Clone, Copy)]
pub struct PtyInt(pub i128);
impl PettyObjectType for PtyInt {
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__add__" => RawFunction(__add__).into(),
            "__sub__" => RawFunction(__sub__).into(),
            "__mul__" => RawFunction(__mul__).into(),
            "__div__" => RawFunction(__div__).into(),
            "__mod__" => RawFunction(__mod__).into(),
            "__is_eq__" => RawFunction(__is_eq__).into(),
            "__lt__" => RawFunction(__lt__).into(),
            "__gt__" => RawFunction(__gt__).into(),
            "__lt_eq__" => RawFunction(__lt_eq__).into(),
            "__gt_eq__" => RawFunction(__gt_eq__).into(),
            "__bool__" => RawFunction(__bool__).into(),
            "__neg__" => RawFunction(__neg__).into(),
            "__repr__" => RawFunction(__repr__).into(),
            "abs" => RawFunction(abs).into(),
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

#[pettymethod]
fn __add__(lhs: PtyInt, rhs: PtyInt) -> PtyInt {
    PtyInt(lhs.0 + rhs.0)
}
#[pettymethod]
fn __sub__(lhs: PtyInt, rhs: PtyInt) -> PtyInt {
    PtyInt(lhs.0 - rhs.0)
}
#[pettymethod]
fn __mul__(lhs: PtyInt, rhs: PtyInt) -> PtyInt {
    PtyInt(lhs.0 - rhs.0)
}
#[pettymethod]
fn __div__(lhs: PtyInt, rhs: PtyInt) -> PtyInt {
    PtyInt(lhs.0 - rhs.0)
}
#[pettymethod]
fn __mod__(lhs: PtyInt, rhs: PtyInt) -> PtyInt {
    PtyInt(lhs.0 % rhs.0)
}
#[pettymethod]
fn __is_eq__(lhs: PtyInt, rhs: PtyInt) -> PtyBool {
    PtyBool(lhs.0 == rhs.0)
}
#[pettymethod]
fn __lt__(lhs: PtyInt, rhs: PtyInt) -> PtyBool {
    PtyBool(lhs.0 < rhs.0)
}
#[pettymethod]
fn __gt__(lhs: PtyInt, rhs: PtyInt) -> PtyBool {
    PtyBool(lhs.0 > rhs.0)
}
#[pettymethod]
fn __lt_eq__(lhs: PtyInt, rhs: PtyInt) -> PtyBool {
    PtyBool(lhs.0 <= rhs.0)
}
#[pettymethod]
fn __gt_eq__(lhs: PtyInt, rhs: PtyInt) -> PtyBool {
    PtyBool(lhs.0 >= rhs.0)
}
#[pettymethod]
fn __bool__(lhs: PtyInt) -> PtyBool {
    PtyBool(lhs.0 != 0)
}
#[pettymethod]
fn __not__(lhs: PtyInt) -> PtyBool {
    PtyBool(lhs.0 == 0)
}
#[pettymethod]
fn __neg__(lhs: PtyInt) -> PtyInt {
    PtyInt(-lhs.0)
}
#[pettymethod]
fn __repr__(lhs: PtyInt) -> PtyStr {
    PtyStr(lhs.0.to_string().into())
}
#[pettymethod]
fn abs(self_: PtyInt) -> PtyInt {
    PtyInt(self_.0.abs())
}
