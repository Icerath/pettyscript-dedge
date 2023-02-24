use std::{any::Any, collections::HashMap};

use crate::ast::Node;

use super::{builtins::{BoolBuiltin, StringBuiltin, NullBuiltin}, interpreter::Interpreter};
pub type PtyObj = Box<dyn PettyObject>;
pub struct PettyValue {
    ref_count: *mut usize,
    pub object: *mut PtyObj,
}
impl<P: PettyObject + 'static> From<P> for PettyValue {
    fn from(value: P) -> Self {
        Self::new(Box::new(value))
    }
}
pub trait PettyObject: 'static {
    fn type_name(&self) -> &'static str;
    fn __add__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        None
    }
    fn __sub__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        None
    }
    fn __mul__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        None
    }
    fn __div__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        None
    }
    fn __bool__(&self, interpreter: &mut Interpreter, source: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __and__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        None
    }
    fn __or__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        None
    }
    fn __is_eq__(&self, other: PettyValue) -> Option<PettyValue> {
        if self.type_id() != other.type_id() {
            return Some(BoolBuiltin(false).into());
        }
        None
    }
    fn __lt__(&self, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __gt__(&self, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __lt_eq__(&self, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __gt_eq__(&self, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __call__(&self, interpreter: &mut Interpreter, args: Vec<PettyValue>) -> Option<PettyValue> {
        None
    }
    fn __repr__(&self, interpreter: &mut Interpreter, source: PettyValue) -> Option<PettyValue> {
        let string = format!("{} object at: {:?}", self.type_name(), source.object);
        Some(StringBuiltin(string).into())
    }
    fn as_any(&self) -> &dyn std::any::Any;
}

impl PettyValue {
    pub fn new(object: PtyObj) -> Self {
        Self {
            object: Box::into_raw(Box::new(object)),
            ref_count: Box::into_raw(Box::new(1)),
        }
    }
    pub fn inner(&self) -> &PtyObj {
        unsafe { self.object.as_ref().unwrap() }
    }
    pub fn ref_count(&self) -> usize {
        unsafe { *self.ref_count }
    }
}
impl Clone for PettyValue {
    fn clone(&self) -> Self {
        unsafe {
            *self.ref_count += 1;
        }
        Self {
            object: self.object,
            ref_count: self.ref_count,
        }
    }
}
impl Drop for PettyValue {
    fn drop(&mut self) {
        unsafe {
            *self.ref_count -= 1;
            if *self.ref_count == 0 {
                self.object.drop_in_place();
            }
        }
    }
}

pub struct PettyValueFunction {
    params: Vec<Box<str>>,
    nodes: Box<[Node]>,
}

impl PettyValueFunction {
    pub fn new(params: Vec<Box<str>>, nodes: Box<[Node]>) -> Self {
        Self { params, nodes }
    }
}

impl PettyObject for PettyValueFunction {
    fn type_name(&self) -> &'static str {
        "Function"
    }
    fn __call__(&self, interpreter: &mut Interpreter, args: Vec<PettyValue>) -> Option<PettyValue> {
        interpreter.variables.new_scope();
        for (param, arg) in self.params.iter().zip(args.into_iter()) {
            interpreter.variables.write(param.clone(), arg);
        }
        interpreter.execute_nodes(&self.nodes);
        interpreter.variables.drop_scope();
        interpreter.return_val.take()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct PettyValueCustom {
    fields: HashMap<Box<str>, PettyValue>,
}

impl PettyValueCustom {
    pub fn new(field_names: Box<[Box<str>]>) -> Self {
        let mut fields = HashMap::new();
        for field in field_names.to_vec() {
            fields.insert(field, NullBuiltin.into());
        }
        Self { fields }
    }
}

impl PettyObject for PettyValueCustom {
    fn type_name(&self) -> &'static str {
        "Class"
    }
    fn __add__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        let func = self.fields.get("__add__")?;
        func.inner().__call__(interpreter, vec![source, other]);
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
