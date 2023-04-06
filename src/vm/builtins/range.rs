use std::{
    fmt,
    sync::{Arc, Mutex},
};

use macros::pettymethod;

use crate::vm::{
    builtins::{option::NONE, PtyOption},
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::{PtyNum, PtyStr};

#[derive(Clone)]
pub struct PtyRange {
    end: f64,
    step: f64,
    current: Arc<Mutex<f64>>,
}

impl PettyObjectType for PtyRange {
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__iter__" | "iter" => __ITER__.clone(),
            "__next__" | "next" => __NEXT__.clone(),
            "__len__" | "len" => __LEN__.clone(),
            "__repr__" => __REPR__.clone(),

            _ => todo!("{str}"),
        }
    }
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PtyRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "range at {:?}", self as *const Self)
    }
}

#[pettymethod]
pub fn range(end: PtyNum) -> PtyRange {
    PtyRange {
        end: end.0,
        current: Mutex::new(0.0).into(),
        step: 1.0,
    }
}

#[pettymethod]
fn __iter__(this: PtyRange) -> PtyRange {
    this
}

#[pettymethod]
fn __next__(this: PtyRange) -> PettyObject {
    let mut current = this.current.lock().unwrap();
    let prev_current = *current;
    *current += this.step;
    if *current > this.end {
        return NONE.clone();
    }
    PtyOption::new(Some(PtyNum(prev_current).into()))
}

#[pettymethod]
fn __len__(_this: PtyRange) -> PtyNum {
    todo!()
}

#[pettymethod]
fn __repr__(this: PtyRange) -> PtyStr {
    PtyStr(format!("{this}").into())
}
