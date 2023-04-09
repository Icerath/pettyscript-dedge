use std::sync::{Arc, Mutex};

use crate::vm::{builtins::Module, dict::Dict};

pub fn init() -> Module {
    let dict = Dict::new();
    Module {
        name: "".into(),
        dict: Arc::new(Mutex::new(dict)),
    }
}
