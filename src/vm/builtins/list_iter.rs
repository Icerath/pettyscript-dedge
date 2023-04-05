use std::{
    fmt,
    sync::{Arc, Mutex},
};

use macros::pettymethod;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::{option::PtyOption, PtyStr};

#[derive(Clone)]
pub struct PtyListIter(pub Arc<Mutex<Vec<PettyObject>>>, pub Arc<Mutex<usize>>);

impl PettyObjectType for PtyListIter {
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__next__" => __NEXT__.clone(),
            "__iter__" | "iter" => __ITER__.clone(),
            "__repr__" => __REPR__.clone(),
            _ => todo!("{str}"),
        }
    }
    fn call(&self, vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PtyListIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "iterator at {:?}", self as *const Self)
    }
}

#[pettymethod]
fn __repr__(this: PtyListIter) -> PtyStr {
    PtyStr(format!("{this}").into())
}

#[pettymethod]
fn __next__(mut this: PtyListIter) -> PtyOption {
    let mut int = this.1.lock().unwrap();
    let next = this.0.lock().unwrap().get(*int).cloned();
    *int += 1;
    PtyOption(next)
}

#[pettymethod]
fn __iter__(this: PtyListIter) -> PtyListIter {
    this
}
