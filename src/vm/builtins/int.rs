use std::{fmt, any::Any};

use crate::{vm::{object::{PettyObject, PettyObjectType}, core::Vm, function_args::FuncArgs}, slim_rc::Rc};

#[derive(Clone, Copy)]
pub struct PtyInt(i128);
impl PtyInt {
    pub fn new(int: i128) -> Self {
        Self(int)
    }
}
impl PettyObjectType for PtyInt {
    fn call(&self, vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__add__" => PtyIntAdd.into(),
            _ => todo!(),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl fmt::Display for PtyInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Copy)]
pub struct PtyIntAdd;

impl PettyObjectType for PtyIntAdd {
    fn call(&self, vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
        if args.0.len() != 2 {
            todo!("Expected 2 arguments got {}", args.0.len());
        }
        let (lhs, rhs) = (args.0[0].clone(), args.0[1].clone());
        let lhs = lhs.as_any().downcast_ref::<PtyInt>().unwrap();
        let rhs = rhs.as_any().downcast_ref::<PtyInt>().unwrap();
        PtyInt(lhs.0 + rhs.0).into()
    }
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl fmt::Display for PtyIntAdd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self as *const Self;
        write!(f, "Function Object at {ptr:?}")
    }
}