use std::{
    fmt,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use macros::pettymethod;

use crate::vm::{
    builtins::PtyStr,
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

#[derive(Clone)]
pub struct ThreadHandle {
    handle: Arc<Mutex<Option<JoinHandle<PettyObject>>>>,
}

impl PettyObjectType for ThreadHandle {
    fn get_item(&self, vm: &Vm, this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "join" => JOIN.clone(),
            "__repr__" => __REPR__.clone(),
            _ => todo!("{str}"),
        }
    }
    fn call(&self, vm: &Vm, this: &PettyObject, args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[pettymethod]
pub fn spawn_thread(vm: &Vm, func: &PettyObject) -> ThreadHandle {
    let vm = vm.clone();
    let func = func.clone();
    let join_handle = std::thread::spawn(move || func.call(&vm, &func, FuncArgs(&[])));
    ThreadHandle {
        handle: Mutex::new(Some(join_handle)).into(),
    }
}

#[pettymethod]
pub fn join(this: ThreadHandle) -> PettyObject {
    let potato = this.handle.lock().unwrap().take().unwrap();
    potato.join().unwrap()
}

#[pettymethod]
pub fn __repr__(this: ThreadHandle) -> PtyStr {
    PtyStr(format!("{this}").into())
}

impl fmt::Display for ThreadHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.handle)
    }
}
