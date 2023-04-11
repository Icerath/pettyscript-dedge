use crate::vm::prelude::*;

pub fn init() -> Module {
    let dict = Dict::from([
        ("__repr__".into(), __REPR__.clone()),
        ("assert".into(), ASSERT.clone()),
        ("assert_eq".into(), ASSERT_EQ.clone()),
        ("assert_ne".into(), ASSERT_NE.clone()),
    ]);

    Module {
        name: "fs".into(),
        dict: Mutex::new(dict).into(),
    }
}

#[pettymethod]
fn __repr__(this: &Module) -> PtyStr {
    PtyStr::from(this.to_string())
}

#[pettymethod]
fn assert(condition: &PettyObject, vm: &mut Vm) {
    let bool = condition.call_method(vm, "__bool__", FuncArgs(&[condition]));
    let bool: PtyBool = bool.downcast().expect("Not a bool");
    assert!(bool.0);
}

#[pettymethod]
fn assert_eq(lhs: &PettyObject, rhs: &PettyObject, vm: &mut Vm) {
    let bool = lhs.call_method(vm, "__is_eq__", FuncArgs(&[lhs, rhs]));
    let bool: PtyBool = bool.downcast().expect("Not a bool");
    assert!(bool.0);
}

#[pettymethod]
fn assert_ne(lhs: &PettyObject, rhs: &PettyObject, vm: &mut Vm) {
    let bool = lhs.call_method(vm, "__is_eq__", FuncArgs(&[lhs, rhs]));
    let bool: PtyBool = bool.downcast().expect("Not a bool");
    assert!(!bool.0);
}
