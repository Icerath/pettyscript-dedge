use std::sync::Mutex;

use self::object::PettyObject;
use crate::ast::{self, Node};

mod builtins;
mod core;
mod dict;
mod field_dict;
mod function_args;
mod object;
mod petty_class;
mod petty_function;
mod preallocated;
mod raw_function;

pub fn run_virtual_machine(ast: &ast::Node) -> Vec<PettyObject> {
    let vm = core::Vm::new();
    builtins::load_builtins(&vm);
    match ast {
        Node::Block(nodes) | Node::Globals(nodes) => vm.evaluate_list(nodes),
        node => vec![vm.evaluate(node)],
    }
}
