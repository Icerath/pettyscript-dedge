use std::thread::JoinHandle;

use crate::vm::prelude::*;

pub struct ThreadPool(Arc<Mutex<Vec<JoinHandle<PettyObject>>>>);

impl PettyObjectType for ThreadPool {
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, key: &str) -> PettyObject {
        match key {
            "__repr__" => __REPR__.clone(),
            "__init__" => __INIT__.clone(),
            "spawn" => SPAWN.clone(),
            "join" => JOIN.clone(),
            _ => todo!("{key}"),
        }
    }
    fn call(&self, _vm: &mut Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for ThreadPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[pettymethod]
pub fn __init__() -> ThreadPool {
    ThreadPool(Arc::new(Mutex::new(vec![])))
}

#[pettymethod]
fn __repr__(this: &ThreadPool) -> PtyStr {
    format!("{this}").into()
}

#[pettymethod]
fn spawn(this: &ThreadPool, func: &PettyObject, vm: &mut Vm) {
    let mut vm = vm.spawn_new();
    let func = func.clone();
    let join_handle = std::thread::spawn(move || func.call(&mut vm, &func, FuncArgs(&[])));
    this.0.lock().unwrap().push(join_handle);
}

#[pettymethod]
fn join(this: &ThreadPool) -> PtyList {
    let mut lock = this.0.lock().unwrap();
    let threads: Vec<_> = std::mem::take(lock.as_mut());
    drop(lock);
    let mut output = vec![];
    for thread in threads {
        let object = thread.join().unwrap();
        output.push(object);
    }
    PtyList(Mutex::new(output).into())
}
