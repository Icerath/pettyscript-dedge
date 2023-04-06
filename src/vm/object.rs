use super::{
    builtins::{PtyStr, NULL},
    core::Vm,
    function_args::FuncArgs,
};
use std::{fmt, ops::Deref, sync::Arc};

pub trait PettyObjectType: fmt::Display + Sync + Send {
    fn get_item(&self, vm: &mut Vm, this: &PettyObject, str: &str) -> PettyObject;
    fn call(&self, vm: &mut Vm, this: &PettyObject, args: FuncArgs) -> PettyObject;
    fn as_any(&self) -> &dyn std::any::Any;
}
/// An actually petty object.
#[derive(Clone)]
pub struct PettyObject(Arc<dyn PettyObjectType>);

impl PettyObject {
    pub fn new<Pty: PettyObjectType + 'static>(object: Pty) -> Self {
        Self(Arc::new(object))
    }
    pub fn call_method(&self, vm: &mut Vm, func: &str, mut args: FuncArgs) -> PettyObject {
        args.0.push(self.clone());
        let function = self.get_item(vm, self, func);
        function.call(vm, &function, args)
    }
    #[inline]
    pub fn repr(&self, vm: &mut Vm) -> Option<PtyStr> {
        let repr = self.call_method(vm, "__repr__", FuncArgs(vec![]));
        repr.downcast::<PtyStr>()
    }
    #[inline]
    pub fn force_repr(&self, vm: &mut Vm) -> PtyStr {
        self.repr(vm)
            .unwrap_or_else(|| panic!("{self} did not have repr"))
    }
    #[inline]
    pub fn downcast_ref<T: PettyObjectType + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
    #[inline]
    pub fn downcast<T: PettyObjectType + Clone + 'static>(&self) -> Option<T> {
        self.as_any().downcast_ref::<T>().cloned()
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
    type Target = Arc<dyn PettyObjectType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<()> for PettyObject {
    #[inline]
    fn from(_: ()) -> Self {
        NULL.clone()
    }
}
