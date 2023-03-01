use crate::ast;

mod builtins;
mod core;
mod field_dict;
mod function_args;
mod object;
mod petty_class;
mod petty_function;

pub fn run_virtual_machine(ast: &ast::Node) {
    let mut vm = core::VirtualMachine::new();
    builtins::load_builtins(&mut vm);
    vm.evaluate(ast);
}
