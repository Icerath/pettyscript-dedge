use std::any::Any;

use crate::ast::Node;

use super::{interpreter::Interpreter, builtins::BoolBuiltin};
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
    fn __add__(&self, interpreter: &mut Interpreter, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __sub__(&self, interpreter: &mut Interpreter, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __mul__(&self, interpreter: &mut Interpreter, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __div__(&self, interpreter: &mut Interpreter, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __bool__(&self, interpreter: &mut Interpreter, source: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __and__(&self, interpreter: &mut Interpreter, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __or__(&self, interpreter: &mut Interpreter, other: PettyValue) -> Option<PettyValue> {
        None
    }
    fn __is_eq__(&self, other: PettyValue) -> Option<PettyValue> {
        if self.type_id() != other.type_id() {
            return Some(BoolBuiltin(false).into())
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
        None
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
        Self { nodes, params }
    }
}

impl PettyObject for PettyValueFunction {
    fn __call__(&self, interpreter: &mut Interpreter, args: Vec<PettyValue>) -> Option<PettyValue> {
        todo!();
        None
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
