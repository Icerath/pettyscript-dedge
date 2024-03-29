#![allow(clippy::needless_pass_by_value)]
mod list;
mod list_iter;
mod module;
mod null;
mod number;
mod option;
mod print;
mod pty_bool;
mod range;
mod ref_count;
mod repr;
mod string;

use std::fmt;

use self::ref_count::GETREFCOUNT;
use super::{core::Vm, object::PettyObject, raw_function::RawFunction, stdlib};
pub use list::PtyList;
pub use list_iter::PtyListIter;
pub use module::Module;
pub use null::{PtyNull, NULL};
pub use number::PtyNum;
pub use option::{PtyOption, NONE};
pub use pty_bool::{PtyBool, FALSE, TRUE};
pub use range::RANGE;
pub use string::PtyStr;

pub fn load_builtins(vm: &mut Vm) {
    let builtins = [
        ("print", RawFunction(print::print).into()),
        ("repr", RawFunction(repr::repr).into()),
        ("range", RANGE.clone()),
        ("Some", RawFunction(option::some).into()),
        ("getrefcount", GETREFCOUNT.clone()),
        ("None", PtyOption(None).into()),
        ("std", stdlib::init().into()),
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
