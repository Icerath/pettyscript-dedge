use macros::pettymethod;

use crate::vm::{
    builtins::PtyStr,
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};
use std::{
    fmt,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct PtyList(pub Arc<Mutex<Vec<PettyObject>>>);

impl PettyObjectType for PtyList {
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject {
        match str {
            "push" => PUSH.clone(),
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
fn push(self_: PtyList, obj: PettyObject) {
    self_.0.lock().unwrap().push(obj);
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
