use std::fmt;

use crate::ast::Node;

use super::{
    builtins::{self, PtyNull},
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

#[derive(Clone)]
pub struct PettyFunction {
    args: Box<[Box<str>]>,
    block: Box<[Node]>,
}
impl PettyFunction {
    pub fn new(args: Box<[Box<str>]>, block: Box<[Node]>) -> Self {
        Self { args, block }
    }
}
impl PettyObjectType for PettyFunction {
    fn call(&self, vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
        vm.fields.new_scope();
        if self.args.len() != args.0.len() {
            todo!("Expected {} arguments, got {}.", {self.args.len()}, args.0.len());
        }
        for (param, arg) in self.args.iter().zip(args.0.into_iter()) {
            vm.fields.write(param, arg);
        }
        vm.execute_nodes(&self.block);
        vm.fields.drop_scope();
        vm.return_val.take().unwrap_or_else(|| PtyNull.into())
    }
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PettyFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        builtins::display_function_object(self, f)
    }
}
