use crate::vm::prelude::*;

pub static NONE: Lazy<PettyObject> = Lazy::new(|| PtyOption(None).into());
#[derive(Clone)]
pub struct PtyOption(pub Option<PettyObject>);

impl PtyOption {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(inner: Option<PettyObject>) -> PettyObject {
        match inner {
            Some(_) => Self(inner).into(),
            None => NONE.clone(),
        }
    }
}

impl PettyObjectType for PtyOption {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn call(&self, _vm: &mut Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "unwrap" => UNWRAP.clone(),
            "is_some" => IS_SOME.clone(),
            "is_none" => IS_NONE.clone(),
            "__repr__" => __REPR__.clone(),
            _ => todo!(),
        }
    }
}

impl fmt::Display for PtyOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(obj) => write!(f, "Some({obj})"),
            None => write!(f, "None"),
        }
    }
}

#[pettymethod]
fn unwrap(opt: PtyOption) -> PettyObject {
    opt.0.unwrap()
}

#[pettymethod]
fn __repr__(self_: &PtyOption, vm: &mut Vm) -> PtyStr {
    match &self_.0 {
        Some(obj) => obj.force_repr(vm),
        None => PtyStr("None".into()),
    }
}

#[pettymethod]
fn is_some(self_: &PtyOption) -> PtyBool {
    PtyBool(self_.0.is_some())
}

#[pettymethod]
fn is_none(self_: &PtyOption) -> PtyBool {
    PtyBool(self_.0.is_none())
}

#[pettymethod]
pub fn some(obj: &PettyObject) -> PtyOption {
    PtyOption(Some(obj.clone()))
}

impl From<Option<PettyObject>> for PettyObject {
    #[inline]
    fn from(value: Option<PettyObject>) -> Self {
        PtyOption::new(value)
    }
}
