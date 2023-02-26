use std::{fmt, ops::Deref, ptr::NonNull, any::Any};

use crate::slim_rc::Rc;

use super::{core::Vm, function_args::FuncArgs};

pub trait PettyObjectType: fmt::Display {
    fn get_item(&self, vm: &mut Vm, this: PettyObject, str: &str) -> PettyObject;
    fn call(&self, vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject;
    fn as_any(&self) -> &dyn Any;
}
/// An actually petty object.
#[derive(Clone)]
pub struct PettyObject(Rc<dyn PettyObjectType>);
impl PettyObject {
    pub fn new<Pty: PettyObjectType + 'static>(object: Pty) -> Self {
        Self(Rc {
            ref_count: Box::leak(Box::new(1)).into(),
            object: NonNull::from(Box::leak(Box::new(object))),
        })
    }
    pub fn inner(&self) -> &dyn PettyObjectType {
        unsafe { self.object.as_ref() }
    }
}
impl<Pty: PettyObjectType + 'static> From<Pty> for PettyObject {
    fn from(value: Pty) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for PettyObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for PettyObject {
    type Target = Rc<dyn PettyObjectType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}