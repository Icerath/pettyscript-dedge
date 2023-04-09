use std::io::{Read, Write};

use crate::vm::prelude::*;

#[derive(Clone)]
pub struct File {
    inner: Arc<Mutex<std::fs::File>>,
}

impl File {
    pub fn new(file: std::fs::File) -> Self {
        Self {
            inner: Arc::new(Mutex::new(file)),
        }
    }
    pub fn open_readonly(path: &str) -> Self {
        Self::new(std::fs::File::open(path).unwrap())
    }
}

impl PettyObjectType for File {
    fn get_item(&self, vm: &mut Vm, this: &PettyObject, key: &str) -> PettyObject {
        match key {
            "__repr__" => __REPR__.clone(),
            "read" | "read_text" => READ.clone(),
            _ => todo!("{key}"),
        }
    }
    fn call(&self, vm: &mut Vm, this: &PettyObject, args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File Handle at {:?}", self as *const Self)
    }
}

#[pettymethod]
pub fn open(path: PtyStr) -> File {
    File::open_readonly(&path.0)
}

#[pettymethod]
pub fn __repr__(this: &File) -> PtyStr {
    PtyStr::from(format!("{this}"))
}

#[pettymethod]
pub fn read(this: &File) -> PtyStr {
    let mut buf = String::new();
    this.inner.lock().unwrap().read_to_string(&mut buf).unwrap();
    buf.into()
}

#[pettymethod]
pub fn write(this: &File, content: &PtyStr) {
    let content = content.0.as_bytes();
    this.inner.lock().unwrap().write(&content).unwrap();
}
