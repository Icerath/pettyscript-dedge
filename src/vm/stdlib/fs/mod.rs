mod file;

use crate::vm::prelude::*;

use self::file::OPEN;

pub fn init() -> Module {
    let dict = Dict::from([
        ("__repr__".into(), __REPR__.clone()),
        ("read_text".into(), READ_TEXT.clone()),
        ("open".into(), OPEN.clone()),
    ]);

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
