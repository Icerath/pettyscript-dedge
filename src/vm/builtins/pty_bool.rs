use std::fmt;

use crate::vm::{
    core::Vm,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

use super::{function_template::SingleTemplate, PtyStr};

#[derive(Clone)]
pub struct PtyBool(pub bool);
impl PettyObjectType for PtyBool {
    fn get_item(&self, _vm: &mut Vm, _this: PettyObject, str: &str) -> PettyObject {
        match str {
            "__bool__" => SingleTemplate(Self::clone).into(),
            "__not__" => SingleTemplate(|this: &Self| Self(!this.0)).into(),
            "__repr__" => SingleTemplate(|this: &Self| PtyStr::from_obj(this)).into(),
            _ => todo!("{str}"),
        }
    }
    fn call(&self, _vm: &mut Vm, _this: PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PtyBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
