use crate::vm::prelude::*;

#[derive(Clone)]
pub struct PtyStr(pub Arc<str>);

impl PettyObjectType for PtyStr {
    fn call(&self, _vm: &mut Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
        todo!("String is not Callable")
    }
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "__repr__" => __REPR__.clone(),
            "__add__" => __ADD__.clone(),
            "__mul__" => __MUL__.clone(),
            "__is_eq__" => __IS_EQ__.clone(),
            "format" => RawFunction(str_format).into(),
            _ => todo!(),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PtyStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[pettymethod]
fn __repr__(self_: PtyStr) -> PtyStr {
    self_
}

#[pettymethod]
fn __add__(lhs: &PtyStr, rhs: &PtyStr) -> PtyStr {
    PtyStr((lhs.0.to_string() + &rhs.0).into())
}

#[pettymethod]
fn __mul__(lhs: &PtyStr, rhs: PtyNum) -> PtyStr {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    PtyStr(lhs.0.repeat(rhs.0.max(0.0) as usize).into())
}

#[pettymethod]
fn __is_eq__(lhs: &PtyStr, rhs: &PtyStr) -> PettyObject {
    PtyBool::new(lhs.0 == rhs.0)
}

#[pettymethod]
fn __lt_eq__(lhs: &PtyStr, rhs: &PtyStr) -> PettyObject {
    PtyBool::new(lhs.0 <= rhs.0)
}

#[pettymethod]
fn __lt__(lhs: &PtyStr, rhs: &PtyStr) -> PettyObject {
    PtyBool::new(lhs.0 < rhs.0)
}

#[pettymethod]
fn __gt_eq__(lhs: &PtyStr, rhs: &PtyStr) -> PettyObject {
    PtyBool::new(lhs.0 >= rhs.0)
}

#[pettymethod]
fn __gt__(lhs: &PtyStr, rhs: &PtyStr) -> PettyObject {
    PtyBool::new(lhs.0 > rhs.0)
}

// TODO - use #[pettymethod]
fn str_format(vm: &mut Vm, _this: &PettyObject, args: FuncArgs) -> PettyObject {
    let mut args = args.0.iter();
    let first_arg = args.next().unwrap();
    let Some(PtyStr(format_str)) = first_arg.downcast_ref::<PtyStr>() else {
        println!("{first_arg}");
        todo!();
    };
    let mut output = String::new();
    for (index, seg) in format_str.split("{}").enumerate() {
        if index != 0 {
            let repr = args.next().unwrap().repr(vm).unwrap();
            output.push_str(&repr.0);
        }
        output.push_str(seg);
    }
    PtyStr(output.into()).into()
}

impl From<String> for PtyStr {
    #[inline]
    fn from(value: String) -> Self {
        PtyStr(value.into())
    }
}
