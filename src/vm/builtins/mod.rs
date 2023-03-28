mod null;
mod number;
mod print;
mod pty_bool;
mod repr;
mod string;

use std::fmt;

pub use self::repr::PtyRepr;
pub use null::PtyNull;
pub use number::PtyNum;
pub use print::PtyPrint;
pub use pty_bool::PtyBool;
pub use string::PtyStr;

use crate::ast::Literal;

use super::{core::VirtualMachine, object::PettyObject};
pub fn load_builtins(vm: &mut VirtualMachine) {
    let builtins = [("print", PtyPrint.into()), ("repr", PtyRepr.into())];
    for (name, builtin) in builtins {
        vm.load_builtin(name, builtin);
    }
}

pub fn create_literal(literal: &Literal) -> PettyObject {
    match literal {
        Literal::Int(int) => PtyNum(*int as f64).into(),
        Literal::Float(float) => PtyNum(*float).into(),
        Literal::Null => PtyNull.into(),
        Literal::Bool(bool) => PtyBool(*bool).into(),
        Literal::String(string) => PtyStr(string.clone()).into(),
        _ => todo!(),
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

#[inline]
pub fn display_function_object<T>(this: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    T: Into<PettyObject>,
{
    let ptr = this as *const T;
    write!(f, "Function Object at {ptr:?}")
}
