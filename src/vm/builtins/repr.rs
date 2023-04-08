use super::PtyStr;
use crate::vm::{core::Vm, object::PettyObject};
use macros::pettymethod;

#[pettymethod]
pub fn repr(obj: &PettyObject, vm: &Vm) -> PtyStr {
    obj.force_repr(vm)
}
