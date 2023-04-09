use super::PtyNum;
use crate::vm::object::PettyObject;
use macros::pettymethod;

#[pettymethod]
pub fn getrefcount(obj: &PettyObject) -> PtyNum {
    #[allow(clippy::cast_precision_loss)]
    PtyNum(obj.strong_count() as f64)
}
