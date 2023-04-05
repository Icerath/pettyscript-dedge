use macros::pettymethod;

use crate::vm::{
    builtins::{list_iter::PtyListIter, PtyNum, PtyStr},
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};
use std::{
    fmt,
    sync::{Arc, Mutex},
};

use super::PtyBool;

#[derive(Clone)]
pub struct PtyList(pub Arc<Mutex<Vec<PettyObject>>>);

impl PettyObjectType for PtyList {
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "push" => PUSH.clone(),
            "get" => GET.clone(),
            "set" => SET.clone(),
            "len" => LEN.clone(),
            "__repr__" => __REPR__.clone(),
            "__add__" => __ADD__.clone(),
            "__mul__" => __MUL__.clone(),
            "__bool__" => __BOOL__.clone(),
            "__iter__" | "iter" => __ITER__.clone(),
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

impl fmt::Display for PtyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (index, item) in self.0.lock().unwrap().iter().enumerate() {
            let seperator = if index == 0 { "" } else { ", " };
            write!(f, "{seperator}{item}")?;
        }

        write!(f, "]")
    }
}

#[pettymethod]
fn get(self_: PtyList, index: PtyNum) -> PettyObject {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let index = index.0.max(0.0) as usize;
    self_.0.lock().unwrap()[index].clone()
}

#[pettymethod]
fn set(self_: PtyList, index: PtyNum, obj: PettyObject) {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let index = index.0.max(0.0) as usize;
    self_.0.lock().unwrap()[index] = obj;
}

#[pettymethod]
fn push(self_: PtyList, obj: PettyObject) {
    self_.0.lock().unwrap().push(obj);
}

#[pettymethod]
#[allow(clippy::cast_precision_loss)]
fn len(self_: PtyList) -> PtyNum {
    PtyNum(self_.0.lock().unwrap().len() as f64)
}

#[pettymethod]
fn __repr__(self_: PtyList, vm: &mut Vm) -> PtyStr {
    let mut string = String::from("[");
    for (index, item) in self_.0.lock().unwrap().iter().enumerate() {
        let seperator = if index == 0 { "" } else { ", " };
        string.push_str(seperator);
        string.push_str(&item.force_repr(vm).0);
    }
    string.push(']');
    PtyStr(string.into())
}

#[pettymethod]
fn __bool__(self_: PtyList) -> PettyObject {
    PtyBool::new(!self_.0.lock().unwrap().is_empty())
}

#[pettymethod]
fn __add__(lhs: PtyList, rhs: PtyList) -> PtyList {
    let mut vec = { lhs.0.lock().unwrap().clone() };
    vec.extend_from_slice(&rhs.0.lock().unwrap());

    PtyList(Mutex::new(vec).into())
}

#[pettymethod]
fn __mul__(lhs: PtyList, rhs: PtyNum) -> PtyList {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let repeat = rhs.0.max(0.0) as usize;
    let mut vec = Vec::with_capacity(repeat * lhs.0.lock().unwrap().len());
    for _ in 0..repeat {
        for obj in lhs.0.lock().unwrap().iter() {
            vec.push(obj.clone());
        }
    }
    PtyList(Mutex::new(vec).into())
}

#[pettymethod]
fn __iter__(this: PtyList) -> PtyListIter {
    PtyListIter(this.0, Mutex::new(0).into())
}
