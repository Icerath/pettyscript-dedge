use crate::vm::prelude::*;

#[derive(Clone)]
pub struct PtyStr(pub Arc<str>);

impl PettyObjectType for PtyStr {
    fn call(&self, _vm: &mut Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
        todo!("String is not Callable")
    }
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, key: &str) -> PettyObject {
        match key {
            "__repr__" => __REPR__.clone(),
            "__add__" => __ADD__.clone(),
            "__mul__" => __MUL__.clone(),
            "__is_eq__" => __IS_EQ__.clone(),
            "upper" => UPPER.clone(),
            "lower" => LOWER.clone(),
            "find" => FIND.clone(),
            "trim" => TRIM.clone(),
            "trim_start" | "trim_left" => TRIM_START.clone(),
            "trim_end" | "trim_right" => TRIM_END.clone(),
            "trim_start_matches" | "trim_left_matches" => TRIM_START_MATCHES.clone(),
            "trim_end_matches" | "trim_right_matches" => TRIM_END_MATCHES.clone(),
            "format" => STR_FORMAT.clone(),
            _ => todo!("{key}"),
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

#[pettymethod]
fn upper(this: &PtyStr) -> PtyStr {
    PtyStr(this.0.to_uppercase().into())
}

#[pettymethod]
fn lower(this: &PtyStr) -> PtyStr {
    PtyStr(this.0.to_lowercase().into())
}

#[pettymethod]
fn find(this: &PtyStr, substr: &PtyStr) -> Option<PettyObject> {
    #[allow(clippy::cast_precision_loss)]
    this.0
        .find(substr.0.as_ref())
        .map(|i| PtyNum(i as f64).into())
}

#[pettymethod]
fn trim(this: &PtyStr) -> PtyStr {
    PtyStr(this.0.trim().into())
}

#[pettymethod]
fn trim_start(this: &PtyStr) -> PtyStr {
    PtyStr(this.0.trim_start().into())
}

#[pettymethod]
fn trim_end(this: &PtyStr) -> PtyStr {
    PtyStr(this.0.trim_end().into())
}

#[pettymethod]
fn trim_start_matches(this: &PtyStr, pat: &PtyStr) -> PtyStr {
    PtyStr(this.0.trim_start_matches(pat.0.as_ref()).into())
}

#[pettymethod]
fn trim_end_matches(this: &PtyStr, pat: &PtyStr) -> PtyStr {
    PtyStr(this.0.trim_end_matches(pat.0.as_ref()).into())
}

// TODO - use #[pettymethod]
pub static STR_FORMAT: Lazy<PettyObject> = Lazy::new(|| RawFunction(str_format).into());
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
