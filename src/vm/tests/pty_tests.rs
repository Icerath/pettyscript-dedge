use std::fs;

use crate::{parser::parse, vm};

#[test]
fn run_pty_tests() {
    let files = fs::read_dir("pty_tests")
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    for file in files {
        let content = fs::read_to_string(file.path()).unwrap();
        let ast = parse(&content).unwrap();
        vm::run_virtual_machine(&ast);
    }
}
