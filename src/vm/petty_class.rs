use crate::ast::Node;
use macros::pettymethod;
use std::{fmt, sync::Arc};

use super::{
    builtins::{self, PtyStr},
    core::Vm,
    dict::Dict,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
    petty_function::PettyFunction,
    raw_function::RawFunction,
};

#[derive(Clone)]
pub struct PettyClassInstance {
    pub fields: Dict,
}
pub struct PettyClass {
    pub fields: Arc<[Arc<str>]>,
    pub methods: Arc<[Node]>,
}
impl PettyClass {
    pub fn new(fields: Arc<[Arc<str>]>, methods: Arc<[Node]>) -> Self {
        Self { fields, methods }
    }
}
impl PettyClassInstance {
    pub fn new(fields: Dict) -> Self {
        Self { fields }
    }
}
impl PettyObjectType for PettyClassInstance {
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, str: &str) -> PettyObject {
        if let Some(item) = self.fields.get(str) {
            return item.clone();
        }
        match str {
            "__repr__" => RawFunction(__repr__).into(),
            _ => todo!("{str}"),
        }
    }
    fn call<'a>(&self, vm: &mut Vm, this: &'a PettyObject, args: FuncArgs<'a>) -> PettyObject {
        let function = self.get_item(vm, this, "__call__");
        let mut args: Vec<&PettyObject> = args.0.to_vec();
        args.push(this);
        function.call(vm, this, FuncArgs(&args))
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl PettyObjectType for PettyClass {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn call(&self, vm: &mut Vm, _this: &PettyObject, args: FuncArgs) -> PettyObject {
        if self.fields.len() != args.0.len() {
            todo!(
                "Expected {} Arguments got {}",
                self.fields.len(),
                args.0.len()
            );
        }
        let mut fields: Dict = self
            .fields
            .iter()
            .cloned()
            .zip(args.0.iter().copied().cloned())
            .collect();
        for function in self.methods.iter().cloned() {
            let Node::FuncDef(name, params, body) = function else {
            unreachable!();
        };
            let function = PettyFunction::new(params, body, vm.scopes.clone());
            fields.insert(name, function.into());
        }
        PettyClassInstance::new(fields).into()
    }
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, _str: &str) -> PettyObject {
        todo!()
    }
}

impl fmt::Display for PettyClassInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Petty Class Instance at {:?}", self as *const Self)
    }
}

impl fmt::Display for PettyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        builtins::display_class_object(self, f)
    }
}

#[allow(clippy::needless_pass_by_value)]
#[pettymethod]
fn __repr__(self_: PettyClassInstance) -> PtyStr {
    PtyStr(format!("{self_}").into())
}
