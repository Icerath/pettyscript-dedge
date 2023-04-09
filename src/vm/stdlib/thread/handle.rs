use std::{
    fmt,
    sync::{Arc, Mutex},
    thread::{JoinHandle, ThreadId},
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
    pub id: ThreadId,
    pub handle: Arc<Mutex<Option<JoinHandle<PettyObject>>>>,
}

impl ThreadHandle {
    #[inline]
    pub fn spawn(vm: &mut Vm, func: &PettyObject) -> Self {
        let mut vm = vm.spawn_thread();
        let func = func.clone();
        let join_handle = std::thread::spawn(move || func.call(&mut vm, &func, FuncArgs(&[])));
        let id = join_handle.thread().id();
        Self {
            handle: Mutex::new(Some(join_handle)).into(),
            id,
        }
    }
}

impl PettyObjectType for ThreadHandle {
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "join" => JOIN.clone(),
            "__repr__" => __REPR__.clone(),
            _ => todo!("{str}"),
        }
    }
    fn call(&self, _vm: &mut Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[pettymethod]
pub fn join(this: ThreadHandle) -> PettyObject {
    let handle = this.handle.lock().unwrap().take().unwrap();
    handle.join().unwrap()
}

#[pettymethod]
pub fn __repr__(this: ThreadHandle) -> PtyStr {
    PtyStr(format!("{this}").into())
}

impl fmt::Display for ThreadHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.id)
    }
}
