mod fs;
mod test;
mod thread;
mod time;

use super::prelude::*;

pub fn init() -> Module {
    let dict = Dict::from([
        ("fs".into(), fs::init().into()),
        ("thread".into(), thread::init().into()),
        ("time".into(), time::init().into()),
        ("test".into(), test::init().into()),
    ]);
    Module {
        name: "std".into(),
        dict: Mutex::new(dict).into(),
    }
}
