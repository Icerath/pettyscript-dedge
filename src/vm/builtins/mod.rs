#![allow(clippy::needless_pass_by_value)]
mod null;
mod number;
mod option;
mod print;
mod pty_bool;
mod repr;
mod string;

use std::fmt;

pub use null::PtyNull;
pub use number::PtyNum;
pub use pty_bool::PtyBool;
pub use string::PtyStr;

use super::{core::VirtualMachine, object::PettyObject, raw_function::RawFunction};
use crate::ast::Literal;

pub fn load_builtins(vm: &mut VirtualMachine) {
    let builtins = [
        ("print", RawFunction(print::print).into()),
        ("repr", RawFunction(repr::repr).into()),
        ("Some", RawFunction(option::some).into()),
        ("None", RawFunction(option::none).into()),
    ];
    for (name, builtin) in builtins {
        vm.load_builtin(name, builtin);
    }
}

pub fn create_literal(literal: &Literal) -> PettyObject {
    match literal {
        #[allow(clippy::cast_precision_loss)]
        Literal::Int(int) => PtyNum(*int as f64).into(),
        Literal::Float(float) => PtyNum(*float).into(),
        Literal::Null => PtyNull.into(),
        Literal::Bool(bool) => PtyBool(*bool).into(),
        Literal::String(string) => PtyStr(string.clone()).into(),
        Literal::List(_list) => todo!(),
    }
}

#[inline]
pub fn display_class_object<T>(this: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    T: Into<PettyObject>,
{
    let ptr = this as *const T;
    write!(f, "class Object at {ptr:?}")
}
