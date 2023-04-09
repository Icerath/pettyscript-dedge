mod fs;
mod thread;
mod time;

use super::{builtins::Module, dict::Dict};
use std::sync::Mutex;

pub fn init() -> Module {
    let dict = Dict::from([
        ("fs".into(), fs::init().into()),
        ("thread".into(), thread::init().into()),
        ("time".into(), time::init().into()),
    ]);
    Module {
        name: "std".into(),
        dict: Mutex::new(dict).into(),
    }
}
