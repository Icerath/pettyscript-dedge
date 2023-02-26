use std::{any::Any, fmt};

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

pub struct BinOpTemplate<T: Into<PettyObject> + Clone + 'static>(pub fn(T, T) -> T);
pub struct SingleTemplate<
    I: Into<PettyObject> + Clone + 'static,
    O: Into<PettyObject> + Clone + 'static,
>(pub fn(I) -> O);
impl<T: Into<PettyObject> + Clone> PettyObjectType for BinOpTemplate<T> {
    fn call(&self, _vm: &mut Vm, _this: PettyObject, args: FuncArgs) -> PettyObject {
        if args.0.len() != 2 {
            todo!("Expected 2 arguments got {}", args.0.len());
        }
        let (lhs, rhs) = (args.0[0].clone(), args.0[1].clone());
        let (Some(lhs), Some(rhs)) = (
            lhs.as_any().downcast_ref::<T>(),
            rhs.as_any().downcast_ref::<T>())
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
impl<I: Into<PettyObject> + Clone, O: Into<PettyObject> + Clone> PettyObjectType
    for SingleTemplate<I, O>
{
    fn call(&self, _vm: &mut Vm, _this: PettyObject, args: FuncArgs) -> PettyObject {
        if args.0.len() != 1 {
            todo!("Expected 1 arguments got {}", args.0.len());
        }
        let arg = args.0[0].clone();
        let Some(arg) = arg.as_any().downcast_ref::<I>() else {
            todo!("Incorrect Type")
        };
        self.0(arg.clone()).into()
    }
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, _str: &str) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl<T: Into<PettyObject> + Clone> fmt::Display for BinOpTemplate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_function_object(self, f)
    }
}
impl<I: Into<PettyObject> + Clone, O: Into<PettyObject> + Clone> fmt::Display
    for SingleTemplate<I, O>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_function_object(self, f)
    }
}
#[inline]
pub fn display_function_object<T: Into<PettyObject>>(
    this: &T,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let ptr = this as *const T;
    write!(f, "Function Object at {ptr:?}")
}
