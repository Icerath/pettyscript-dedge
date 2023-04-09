use super::Module;
use crate::vm::builtins::PtyStr;
use crate::vm::dict::Dict;
use macros::pettymethod;
use std::sync::Mutex;

pub fn init() -> Module {
    let mut dict = Dict::new();

    dict.insert("__repr__".into(), __REPR__.clone());
    dict.insert("read_text".into(), READ_TEXT.clone());

    Module {
        name: "fs".into(),
        dict: Mutex::new(dict).into(),
    }
}

#[pettymethod]
fn __repr__(fs: &Module) -> PtyStr {
    PtyStr(format!("{fs}").into())
}

#[pettymethod]
fn read_text(path: &PtyStr) -> PtyStr {
    let contents = std::fs::read_to_string(&*path.0).unwrap();
    PtyStr(contents.into())
}
