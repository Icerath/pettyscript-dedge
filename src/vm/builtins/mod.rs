#![allow(clippy::needless_pass_by_value)]
mod filesystem;
mod list;
mod list_iter;
mod module;
mod null;
mod number;
mod option;
mod print;
mod pty_bool;
mod range;
mod repr;
mod string;

pub mod threading;

use std::fmt;

pub use list::PtyList;
pub use module::Module;
pub use null::{PtyNull, NULL};
pub use number::PtyNum;
pub use option::PtyOption;
pub use pty_bool::PtyBool;
pub use range::RANGE;
pub use string::PtyStr;
use threading::thread::SPAWN_THREAD;

use super::{core::Vm, object::PettyObject, raw_function::RawFunction};

pub fn load_builtins(vm: &mut Vm) {
    let builtins = [
        ("print", RawFunction(print::print).into()),
        ("repr", RawFunction(repr::repr).into()),
        ("range", RANGE.clone()),
        ("Some", RawFunction(option::some).into()),
        ("sleep", RawFunction(threading::sleep::sleep).into()),
        ("spawn_thread", SPAWN_THREAD.clone()),
        ("fs", filesystem::init().into()),
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
