#![allow(clippy::needless_pass_by_value)]
mod list;
mod null;
mod number;
mod option;
mod print;
mod pty_bool;
mod repr;
mod string;

use std::fmt;

pub use list::PtyList;
pub use null::{PtyNull, NULL};
pub use number::PtyNum;
pub use pty_bool::PtyBool;
pub use string::PtyStr;

use self::option::PtyOption;

use super::{core::VirtualMachine, object::PettyObject, raw_function::RawFunction};

pub fn load_builtins(vm: &mut VirtualMachine) {
    let builtins = [
        ("print", RawFunction(print::print).into()),
        ("repr", RawFunction(repr::repr).into()),
        ("Some", RawFunction(option::some).into()),
        ("None", PtyOption(None).into()),
    ];
    for (name, builtin) in builtins {
        vm.load_builtin(name, builtin);
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
