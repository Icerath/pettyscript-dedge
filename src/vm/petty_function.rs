use super::{
    builtins::NULL,
    core::Vm,
    dict::Dict,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};
use crate::ast::Node;
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct PettyFunction {
    args: Arc<[Arc<str>]>,
    block: Arc<[Node]>,
    scopes: Vec<Dict>,
}
impl PettyFunction {
    pub fn new(args: Arc<[Arc<str>]>, block: Arc<[Node]>, scopes: Vec<Dict>) -> Self {
        Self {
            args,
            block,
            scopes,
        }
    }
}
impl PettyObjectType for PettyFunction {
    fn call(&self, vm: &Vm, _this: &PettyObject, args: FuncArgs) -> PettyObject {
        for scope in &self.scopes {
            vm.get().fields.scopes.push(scope.clone());
        }
        if self.args.len() != args.0.len() {
            todo!(
                "Expected {} arguments, got {}.",
                { self.args.len() },
                args.0.len()
            );
        }
        for (param, &arg) in self.args.iter().zip(args.0.into_iter()) {
            vm.get().fields.write(param.clone(), arg.clone());
        }
        vm.execute_nodes(&self.block);
        for _ in 0..self.scopes.len() {
            vm.get().fields.drop_scope();
        }
        vm.get().fields.drop_scope();
        vm.get().return_val.take().unwrap_or_else(|| NULL.clone())
    }
    fn get_item(&self, _vm: &Vm, _this: &PettyObject, _str: &str) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PettyFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
