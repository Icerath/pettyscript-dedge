mod handle;
mod pool;

use crate::vm::{
    builtins::{Module, PtyNum},
    core::Vm,
    dict::Dict,
    object::PettyObject,
    stdlib::thread::handle::ThreadHandle,
};
use macros::pettymethod;
use std::sync::{Arc, Mutex};

pub fn init() -> Module {
    let dict = Dict::from([
        ("sleep".into(), SLEEP.clone()),
        ("spawn".into(), SPAWN.clone()),
        ("ThreadPool".into(), pool::__INIT__.clone()),
    ]);
    Module {
        name: "thread".into(),
        dict: Arc::new(Mutex::new(dict)),
    }
}

#[pettymethod]
pub fn sleep(duration: PtyNum) {
    std::thread::sleep(std::time::Duration::from_secs_f64(duration.0));
}

#[pettymethod]
pub fn spawn(vm: &mut Vm, func: &PettyObject) -> ThreadHandle {
    ThreadHandle::spawn(vm, func)
}
