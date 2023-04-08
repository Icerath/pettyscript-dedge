use super::{PtyBool, PtyStr};
use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};
use macros::pettymethod;
use std::fmt;

#[derive(Clone, Copy)]
pub struct PtyNum(pub f64);
impl PettyObjectType for PtyNum {
    fn call(&self, _vm: &Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, _vm: &Vm, _this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "__add__" => __ADD__.clone(),
            "__sub__" => __SUB__.clone(),
            "__mul__" => __MUL__.clone(),
            "__div__" => __DIV__.clone(),
            "__mod__" => __MOD__.clone(),
            "__is_eq__" => __IS_EQ__.clone(),
            "__lt__" => __LT__.clone(),
            "__gt__" => __GT__.clone(),
            "__lt_eq__" => __LT_EQ__.clone(),
            "__gt_eq__" => __GT_EQ__.clone(),
            "__bool__" => __BOOL__.clone(),
            "__neg__" => __NEG__.clone(),
            "__repr__" => __REPR__.clone(),
            "abs" => ABS.clone(),
            _ => todo!("{str}"),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl fmt::Display for PtyNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[pettymethod]
fn __add__(lhs: PtyNum, rhs: PtyNum) -> PtyNum {
    PtyNum(lhs.0 + rhs.0)
}
#[pettymethod]
fn __sub__(lhs: PtyNum, rhs: PtyNum) -> PtyNum {
    PtyNum(lhs.0 - rhs.0)
}
#[pettymethod]
fn __mul__(lhs: PtyNum, rhs: PtyNum) -> PtyNum {
    PtyNum(lhs.0 * rhs.0)
}
#[pettymethod]
fn __div__(lhs: PtyNum, rhs: PtyNum) -> PtyNum {
    PtyNum(lhs.0 / rhs.0)
}
#[pettymethod]
fn __mod__(lhs: PtyNum, rhs: PtyNum) -> PtyNum {
    PtyNum(lhs.0 % rhs.0)
}
#[pettymethod]
#[allow(clippy::float_cmp)]
fn __is_eq__(lhs: PtyNum, rhs: PtyNum) -> PettyObject {
    PtyBool::new(lhs.0 == rhs.0)
}
#[pettymethod]
fn __lt__(lhs: PtyNum, rhs: PtyNum) -> PettyObject {
    PtyBool::new(lhs.0 < rhs.0)
}
#[pettymethod]
fn __gt__(lhs: PtyNum, rhs: PtyNum) -> PettyObject {
    PtyBool::new(lhs.0 > rhs.0)
}
#[pettymethod]
fn __lt_eq__(lhs: PtyNum, rhs: PtyNum) -> PettyObject {
    PtyBool::new(lhs.0 <= rhs.0)
}
#[pettymethod]
fn __gt_eq__(lhs: PtyNum, rhs: PtyNum) -> PettyObject {
    PtyBool::new(lhs.0 >= rhs.0)
}
#[pettymethod]
fn __bool__(lhs: PtyNum) -> PettyObject {
    PtyBool::new(lhs.0 != 0.0)
}
#[pettymethod]
fn __not__(lhs: PtyNum) -> PettyObject {
    PtyBool::new(lhs.0 == 0.0)
}
#[pettymethod]
fn __neg__(lhs: PtyNum) -> PtyNum {
    PtyNum(-lhs.0)
}
#[pettymethod]
fn __repr__(lhs: PtyNum) -> PtyStr {
    PtyStr(lhs.0.to_string().into())
}
#[pettymethod]
fn abs(self_: PtyNum) -> PtyNum {
    PtyNum(self_.0.abs())
}
