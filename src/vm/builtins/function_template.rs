use std::{any::Any, fmt};

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

pub struct BinOpTemplate<Lhs, Rhs, Ret>(pub fn(Lhs, Rhs) -> Ret)
where
    Lhs: Into<PettyObject> + 'static,
    Rhs: Into<PettyObject> + 'static,
    Ret: Into<PettyObject> + 'static;
pub struct SingleTemplate<I, O>(pub fn(&I) -> O)
where
    I: Into<PettyObject> + 'static,
    O: Into<PettyObject> + 'static;
impl<Lhs, Rhs, Ret> PettyObjectType for BinOpTemplate<Lhs, Rhs, Ret>
where
    Lhs: Into<PettyObject> + Clone,
    Rhs: Into<PettyObject> + Clone,
    Ret: Into<PettyObject> + Clone,
{
    fn call(&self, _vm: &mut Vm, _this: PettyObject, args: FuncArgs) -> PettyObject {
        if args.0.len() != 2 {
            todo!("Expected 2 arguments got {}", args.0.len());
        }
        let (lhs, rhs) = (args.0[0].clone(), args.0[1].clone());
        let (Some(lhs), Some(rhs)) = (
            lhs.as_any().downcast_ref::<Lhs>(),
            rhs.as_any().downcast_ref::<Rhs>())
        else {
            todo!("Incorrect Types");
        };
        self.0(lhs.clone(), rhs.clone()).into()
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, _str: &str) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl<I, O> PettyObjectType for SingleTemplate<I, O>
where
    I: Into<PettyObject> + Clone,
    O: Into<PettyObject> + Clone,
{
    fn call(&self, _vm: &mut Vm, _this: PettyObject, args: FuncArgs) -> PettyObject {
        if args.0.len() != 1 {
            todo!("Expected 1 arguments got {}", args.0.len());
        }
        let arg = args.0[0].clone();
        let Some(arg) = arg.as_any().downcast_ref::<I>() else {
            todo!("Incorrect Type")
        };
        self.0(arg).into()
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, _str: &str) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl<Lhs, Rhs, Ret> fmt::Display for BinOpTemplate<Lhs, Rhs, Ret>
where
    Lhs: Into<PettyObject> + Clone,
    Rhs: Into<PettyObject> + Clone,
    Ret: Into<PettyObject> + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_function_object(self, f)
    }
}
impl<I, O> fmt::Display for SingleTemplate<I, O>
where
    I: Into<PettyObject> + Clone,
    O: Into<PettyObject> + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_function_object(self, f)
    }
}

#[inline]
pub fn display_class_object<T>(this: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    T: Into<PettyObject>,
{
    let ptr = this as *const T;
    write!(f, "class Object at {ptr:?}")
}

#[inline]
pub fn display_function_object<T>(this: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    T: Into<PettyObject>,
{
    let ptr = this as *const T;
    write!(f, "Function Object at {ptr:?}")
}
